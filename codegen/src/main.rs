#![recursion_limit = "128"]
use crate::error::CodegenError;
use crate::schema::ParsedSchemaType;
use inflector::Inflector;
use itertools::Itertools;
use serde_json;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::io::BufRead;
use std::{
    fs, io,
    path::{Path, PathBuf},
    process::Command,
    str::FromStr,
};
use tempdir::TempDir;

pub mod error;
mod render;
pub mod schema;

/// Kafka git repository
const REPOSITORY: &str = "https://github.com/apache/kafka";
/// Kafka release branch
const VERSION: &str = "2.2.1";
/// Path to Kafka RPC schema files
const SCHEMA_PATH: &str = "clients/src/main/resources/common/message";
/// Target Rust file to output Kafka RPC types
const API_TARGET: &str = "../kafka-api/src/api.rs";
/// Target Rust file to output Kafka ApiKeys enum
const API_KEY_TARGET: &str = "../kafka-api/src/apikey.rs";

/// Clone `REPOSITORY`, checking out the specified `VERSION`
fn clone_repo(path: &Path) {
    let path_str = path.to_str().expect("could not convert path to string");
    println!("cloning to dir: {}", path_str);
    Command::new("git")
        .args(&["clone", "--branch", VERSION, REPOSITORY, path_str])
        .output()
        .expect("failed to clone repo");
}

/// Collects all Kafka RPC files under `SCHEMA_PATH` for the given `path` root.
/// RequestHeader and ResponseHeader schema files are ignored, along with all non-json files.
///
/// Returns a collection of candidate Kafka RPC JSON files.
fn collect_paths(path: &Path) -> Vec<PathBuf> {
    let buf = path.to_path_buf();
    let buf =
        buf.join(PathBuf::from_str(SCHEMA_PATH).expect("could not form path from schema path"));
    println!(
        "collecting schema files in {}",
        buf.to_str().expect("could not convert path to string")
    );
    std::fs::read_dir(&buf)
        .expect("could not read directory")
        .filter(|p| p.is_ok())
        .map(|p| p.unwrap())
        .filter(|p| p.file_name().to_str().unwrap().contains("json"))
        .filter(|p| !p.file_name().to_str().unwrap().contains("RequestHeader"))
        .filter(|p| !p.file_name().to_str().unwrap().contains("ResponseHeader"))
        .map(|p| p.path())
        .collect()
}

/// Parses a Kafka RPC JSON schema file into an intermediate representation suitable
/// for manipulation and codegen.
fn parse_schema_file(path: &Path) -> Result<schema::ParsedSchema, CodegenError> {
    let file = fs::File::open(path).expect("could not open schema file");
    let reader = io::BufReader::new(file);
    let file_contents: String = reader
        .lines()
        .map(|line| line.expect("invalid line"))
        .filter(|line| !line.contains("//"))
        .collect::<Vec<String>>()
        .join("");
    let mut schema_deserializer = serde_json::Deserializer::from_str(&file_contents);
    let schema: schema::ParsedSchema = serde::Deserialize::deserialize(&mut schema_deserializer)
        .map_err(CodegenError::MalformedSchema)?;
    Ok(schema)
}

/// Strips the "Request" or "Response" suffix from a `name` string.
fn strip_root_schema_type_suffix(name: &str) -> &str {
    name.trim_end_matches("Request")
        .trim_end_matches("Response")
}
/// Specification for an RPC field. Includes versioning, type and documentation information
/// required to render the resulting field.
#[derive(Debug, Clone)]
pub(crate) struct RpcFieldSpec {
    /// True if this field should be wrapped in a Vec
    pub(crate) collection: bool,
    /// Nullability of this field
    pub(crate) nullable: bool,
    /// Name of this RPC field
    pub(crate) field_name: String,
    /// Type of this RPC field
    pub(crate) type_name: String,
    /// The version where this field was added
    pub(crate) version_added: i16,
    /// The version where this field was removed
    pub(crate) version_removed: Option<i16>,
    /// Default value for this RPC field
    pub(crate) default: Option<String>,
    /// Docstring for this RPC field
    pub(crate) docstring: Option<String>,
}

/// Santize strings that would conflict with Rust keywords
fn sanitize(s: String) -> String {
    match s.as_str() {
        "type" => String::from("_type"),
        _ => s,
    }
}

impl RpcFieldSpec {
    /// Generates an RpcFieldSpec from a ParsedSchemaField.
    ///
    /// If the field name starts with `[]`, it is considered to be a collection field.
    /// The field name is converted to snake case.
    fn from_schema(field: &schema::ParsedSchemaField) -> Self {
        let collection = field.type_name.starts_with("[]");
        let nullable = field.nullable_versions.is_some();
        let field_name = sanitize(field.name.to_snake_case());
        let type_name = field
            .type_name
            .trim_start_matches("[]")
            .to_owned()
            .to_pascal_case();
        let version_added = field.versions.version_start();
        let version_removed = field.versions.version_end();
        let default = field.default.clone();
        let docstring = field.about.clone();
        Self {
            collection,
            nullable,
            field_name,
            type_name,
            version_added,
            version_removed,
            default,
            docstring,
        }
    }
}
/// Representation of an RPC struct, containing zero ore more `RpcFieldSpec`.
#[derive(Debug, Clone)]
pub(crate) struct RpcStructSpec {
    pub(crate) struct_name: String,
    pub(crate) rpc_type: schema::ParsedSchemaType,
    pub(crate) api_key: i16,
    pub(crate) fields: Vec<RpcFieldSpec>,
}

/// Representation of a root RPC type. Contains zero or more `RpcFieldSpec`'s and any
/// associated `RpcStructSpec`'s.
#[derive(Debug, Clone)]
pub(crate) struct RpcRootSpec {
    pub(crate) rpc_name: String,
    pub(crate) rpc_type: schema::ParsedSchemaType,
    pub(crate) api_key: i16,
    pub(crate) fields: Vec<RpcFieldSpec>,
    pub(crate) structs: Vec<RpcStructSpec>,
    pub(crate) version_added: i16,
    pub(crate) version_removed: Option<i16>,
}

impl RpcRootSpec {
    /// Return the struct name for this root spec with the Request/Response suffix added back.
    pub(crate) fn reified_struct_name(&self) -> String {
        format!("{}{}", self.rpc_name, self.rpc_type.to_string())
    }
}

/// Recursively expand a nested Kafka RPC schema into `RpcFieldSpec`'s and `RpcStructSpec`'s.
/// Takes a `ParsedSchemaType` and a list of fields to expand, returning `RpcFieldSpec`'s for the
/// provided list of fields. If expansion necessitates a new struct (because the expanded field type is
/// a struct), then the `structs` `Vec` will have the new struct added to it.
///
/// Returns a list of `RpcFieldSpec`'s corresponding to the provided `fields`.
fn expand_fields(
    api_key: i16,
    rpc_type: ParsedSchemaType,
    structs: &mut Vec<RpcStructSpec>,
    fields: &[schema::ParsedSchemaField],
) -> Vec<RpcFieldSpec> {
    let mut converted_fields = vec![];
    for field in fields {
        let field_spec = RpcFieldSpec::from_schema(field);
        match &field.fields {
            None => {
                // This is a leaf field, we don't need to do anything besides add it to the vec
                converted_fields.push(field_spec);
            }
            Some(subfields) => {
                // There are subfields for this schema, creating a new struct is necessary.
                let substruct_name = field_spec.type_name.clone();
                let substruct_fields = expand_fields(api_key, rpc_type, structs, &subfields);
                let substruct = RpcStructSpec {
                    struct_name: substruct_name,
                    rpc_type,
                    api_key,
                    fields: substruct_fields,
                };
                structs.push(substruct);
                converted_fields.push(field_spec)
            }
        }
    }
    converted_fields
}
/// Expand a nested schema tree into a root schema struct and a flattened set of substructs.
fn expand_schema(schema: schema::ParsedSchema) -> RpcRootSpec {
    let rpc_name = strip_root_schema_type_suffix(&schema.name);
    println!("expanding schema for {} {:?}", rpc_name, schema.schema_type);
    let mut structs = vec![];

    let root_fields = expand_fields(
        schema.api_key,
        schema.schema_type,
        &mut structs,
        &schema.fields,
    );
    RpcRootSpec {
        rpc_name: rpc_name.to_owned(),
        rpc_type: schema.schema_type,
        api_key: schema.api_key,
        fields: root_fields,
        version_added: schema.valid_versions.version_start(),
        version_removed: schema.valid_versions.version_end(),
        structs,
    }
}
/// Group root RPC specs by the api key. Returns a map of tuples containing (request, response)
fn group_root_specs(specs: Vec<RpcRootSpec>) -> HashMap<i16, (RpcRootSpec, RpcRootSpec)> {
    let intermediate: HashMap<i16, Vec<RpcRootSpec>> = specs
        .into_iter()
        .map(|spec| (spec.api_key, spec))
        .into_group_map();
    for (api_key, specs) in &intermediate {
        if specs.len() != 2 {
            panic!(format!(
                "did not expect more than 2 request types for api key {}",
                api_key
            ));
        }
    }
    intermediate
        .into_iter()
        .map(|(api_key, mut specs)| {
            specs.sort_by(|s1, _| {
                if s1.rpc_type == schema::ParsedSchemaType::Request {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            });
            (api_key, (specs.remove(0), specs.remove(0)))
        })
        .collect::<HashMap<i16, (RpcRootSpec, RpcRootSpec)>>()
}

fn main() {
    let workspace = TempDir::new("kafka-protocol-codegen").expect("could not create workspace");
    clone_repo(workspace.path());
    let path = workspace.path();
    let paths = collect_paths(&path);
    println!("Found {} schema files", paths.len());
    let mut root_specs = vec![];
    for path in &paths[..] {
        let schema = parse_schema_file(path)
            .unwrap_or_else(|_| panic!("could not parse schema file {:?}", path));
        let root_rpc_spec = expand_schema(schema);
        root_specs.push(root_rpc_spec);
    }
    println!("expanded {} rpcs", root_specs.len());

    let grouped_specs = group_root_specs(root_specs);
    let mut api_file_contents = render::gen_api_file_contents(&grouped_specs);
    let api_key_file_contents = render::gen_api_key_file_contents(&grouped_specs);
    let root_enums = render::gen_root_enums(&grouped_specs);
    let root_enum_impls = render::gen_root_enum_impls(&grouped_specs);
    api_file_contents.extend(root_enums);
    api_file_contents.extend(root_enum_impls);

    fs::write(API_TARGET, api_file_contents.to_string()).expect("Unable to write file");
    fs::write(API_KEY_TARGET, api_key_file_contents.to_string()).expect("Unable to write file");
    println!("generated modules");
    println!("done!");
}

use crate::{RpcFieldSpec, RpcRootSpec, RpcStructSpec};
use inflector::Inflector;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use std::collections::HashMap;
use syn;

impl RpcFieldSpec {
    fn field_type(&self) -> proc_macro2::TokenStream {
        let mut tokens = match self.type_name.as_str() {
            "Bool" => quote! { bool },
            "Byte" => quote! { i8 },
            "Int8" => quote! { i8 },
            "Int16" => quote! { i16 },
            "Int32" => quote! { i32 },
            "Int64" => quote! { i64 },
            "Bytes" => quote! { Vec<u8> },
            "String" => quote! { String },
            v => {
                let type_ident = to_ident(v);
                quote! { #type_ident }
            }
        };
        if self.collection {
            tokens = quote! { Vec<#tokens> };
        }
        if self.nullable {
            tokens = quote! { Option<#tokens> };
        }
        tokens
    }
}

impl ToTokens for RpcFieldSpec {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let field_name = self.field_name.clone();
        let version_added = self.version_added;

        let field_name_ident = to_ident(&field_name);
        let field_type_ident = self.field_type();
        let version_added_ident = quote! { added = #version_added };
        let version_removed_ident = self.version_removed.map(|v| quote! {,removed = #v});
        let default_ident = self.default.clone().map(|v| quote! {,default = #v});
        let docstring_ident = self.docstring.clone().map(|v| quote! { #[doc = #v] });
        tokens.extend(quote! {
          #docstring_ident
          #[kafka(#version_added_ident #version_removed_ident #default_ident)]
          pub #field_name_ident: #field_type_ident
        })
    }
}

impl ToTokens for RpcStructSpec {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let full_struct_name = self.struct_name.to_owned();
        let struct_name_ident = syn::Ident::new(&full_struct_name, proc_macro2::Span::call_site());
        let struct_fields = &self.fields;
        tokens.extend(quote! {
            #[derive(Debug, PartialEq, KafkaRpc, Clone)]
            pub struct #struct_name_ident {
               #(#struct_fields),*
            }
        })
    }
}

impl ToTokens for RpcRootSpec {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let full_struct_name = self.reified_struct_name();
        let struct_name_ident = to_ident(&full_struct_name);
        let struct_fields = &self.fields;
        let substructs = &self.structs;
        let version_added = self.version_added;
        let version_added_ident = quote! { added = #version_added };
        let version_removed_ident = self.version_removed.map(|v| quote! {,removed = #v});

        let body_trait_name = format!("Kafka{}Body", self.rpc_type.to_string());
        let body_trait_ident = to_ident(&body_trait_name);
        let api_key = self.api_key;

        tokens.extend(quote! {
            #[derive(Debug, PartialEq, KafkaRpc, Clone)]
            #[kafka(#version_added_ident #version_removed_ident)]
            pub struct #struct_name_ident {
               #(#struct_fields),*
            }
            impl crate::#body_trait_ident for #struct_name_ident {
                fn api_key() -> crate::apikey::ApiKeys {
                    #api_key.into()
                }
            }
            #(#substructs)*
        })
    }
}

pub(crate) fn render_module(
    tokens: &mut TokenStream,
    request: &RpcRootSpec,
    response: &RpcRootSpec,
) {
    tokens.extend(quote! {
        #request
        #response
    });
}

pub(crate) fn render_api_keys(tokens: &mut TokenStream, pairs: Vec<(String, i16)>) {
    let variants = pairs
        .into_iter()
        .map(|(name_string, key)| (to_ident(&name_string.to_pascal_case()), key))
        .map(|(name, key)| quote! { #name = #key })
        .collect::<Vec<_>>();
    tokens.extend(quote! {
            pub type ApiKey = i16;
            #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
            #[repr(i16)]
            pub enum ApiKeys {
                #(#variants),*
            }

            impl ApiKeys {
              pub fn key(self) -> ApiKey {
                unsafe { std::mem::transmute(self) }
              }
            }
            impl From<ApiKey> for ApiKeys {
                fn from(v: ApiKey) -> Self {
                    unsafe { std::mem::transmute(v) }
                }
            }
            impl Into<i16> for ApiKeys {
                fn into(self) -> i16 {
                    self as i16
                }
            }
    });
}

fn gen_header_imports(file_contents: &mut TokenStream) {
    file_contents.extend(quote! { #![allow(dead_code)] });
    file_contents.extend(quote! { use kafka_protocol_derive::KafkaRpc; });
    file_contents.extend(quote! { use kafka_protocol::KafkaRpcType; });
    file_contents.extend(quote! { use from_variants::FromVariants; });
}

/// Generates Rust structs for each Kafka RPC type.
pub(crate) fn gen_api_file_contents(
    grouped_specs: &HashMap<i16, (RpcRootSpec, RpcRootSpec)>,
) -> TokenStream {
    let mut file_contents = TokenStream::new();
    gen_header_imports(&mut file_contents);

    for (request_spec, response_spec) in grouped_specs.values() {
        render_module(&mut file_contents, &request_spec, &response_spec);
    }
    file_contents
}

/// Render an enum containing all encountered Kafka ApiKey types, useful for dispatching and logging.
pub(crate) fn gen_api_key_file_contents(
    grouped_specs: &HashMap<i16, (RpcRootSpec, RpcRootSpec)>,
) -> TokenStream {
    let mut file_contents = TokenStream::new();
    let mut api_key_pairs: Vec<(String, i16)> = vec![];
    for (api_key, (request_spec, _response_spec)) in grouped_specs {
        api_key_pairs.push((request_spec.rpc_name.clone(), *api_key));
    }
    render_api_keys(&mut file_contents, api_key_pairs);
    file_contents
}

pub(crate) fn gen_root_enums(
    grouped_specs: &HashMap<i16, (RpcRootSpec, RpcRootSpec)>,
) -> TokenStream {
    let mut file_contents = TokenStream::new();
    let mut req_enum_variants = vec![];
    let mut resp_enum_variants = vec![];

    for (req_spec, resp_spec) in grouped_specs.values() {
        let req = to_ident(&req_spec.reified_struct_name());
        req_enum_variants.push(quote! { #req(#req) });

        let resp = to_ident(&resp_spec.reified_struct_name());
        resp_enum_variants.push(quote! { #resp(#resp) });
    }
    file_contents.extend(quote! {
        #[allow(dead_code)]
        #[derive(Debug, Clone, PartialEq, FromVariants)]
        pub enum RequestBody {
           #(#req_enum_variants),*
        }
        #[allow(dead_code)]
        #[derive(Debug, Clone, PartialEq, FromVariants)]
        pub enum ResponseBody {
           #(#resp_enum_variants),*
        }
    });
    file_contents
}

fn to_ident(s: &str) -> syn::Ident {
    syn::Ident::new(s, proc_macro2::Span::call_site())
}

/// Generate idents for both the api key for a root struct, and it's reified name
fn gen_api_key_struct_ident(spec: &RpcRootSpec) -> (syn::Ident, syn::Ident) {
    let api_key_ident = to_ident(&spec.rpc_name);
    let struct_name_ident = to_ident(&spec.reified_struct_name());
    (api_key_ident, struct_name_ident)
}

pub(crate) fn gen_root_enum_impls(
    grouped_specs: &HashMap<i16, (RpcRootSpec, RpcRootSpec)>,
) -> TokenStream {
    let mut file_contents = TokenStream::new();

    let mut req_read_impls = vec![];
    let mut req_size_impls = vec![];
    let mut req_write_impls = vec![];

    let mut resp_read_impls = vec![];
    let mut resp_size_impls = vec![];
    let mut resp_write_impls = vec![];

    for (req_spec, resp_spec) in grouped_specs.values() {
        // Requests
        let (req_api_key, req_struct_ident) = gen_api_key_struct_ident(req_spec);
        req_read_impls.push(
            quote! { crate::apikey::ApiKeys::#req_api_key => crate::util::version_check_read::<#req_struct_ident, R>(ctx).map(Into::into) }
        );
        req_size_impls
            .push(quote! { RequestBody::#req_struct_ident(inner) => inner.size(version) });
        req_write_impls.push(quote! { RequestBody::#req_struct_ident(inner) => inner.write(ctx) });

        // Responses
        let (resp_api_key, resp_struct_ident) = gen_api_key_struct_ident(resp_spec);
        resp_read_impls.push(
            quote! { crate::apikey::ApiKeys::#resp_api_key => crate::util::version_check_read::<#resp_struct_ident, R>(ctx).map(Into::into) }
        );
        resp_size_impls
            .push(quote! { ResponseBody::#resp_struct_ident(inner) => inner.size(version) });
        resp_write_impls
            .push(quote! { ResponseBody::#resp_struct_ident(inner) => inner.write(ctx) });
    }

    file_contents.extend(quote! {
        impl RequestBody {
            pub fn read<R: ::std::io::Read>(
                ctx: &mut kafka_protocol::DeserializeCtx<R>,
                api_key: i16,
            ) -> Result<Self, kafka_protocol::CodecError> {
                let api_key: crate::apikey::ApiKeys = crate::apikey::ApiKeys::from(api_key);
                match api_key {
                    #(#req_read_impls),*
                }
            }

            pub fn size(&self, version: i16) -> usize {
                match self {
                    #(#req_size_impls),*
                }
            }

            pub fn write<W: ::std::io::Write>(
                &self,
                ctx: &mut kafka_protocol::SerializeCtx<W>,
            ) -> Result<(), kafka_protocol::CodecError> {
                match self {
                    #(#req_write_impls),*
                }
            }
        }
    });

    file_contents.extend(quote! {
        impl ResponseBody {
            pub fn read<R: ::std::io::Read>(
                ctx: &mut kafka_protocol::DeserializeCtx<R>,
                api_key: i16,
            ) -> Result<Self, kafka_protocol::CodecError> {
                let api_key: crate::apikey::ApiKeys = crate::apikey::ApiKeys::from(api_key);
                match api_key {
                    #(#resp_read_impls),*
                }
            }

            pub fn size(&self, version: i16) -> usize {
                match self {
                    #(#resp_size_impls),*
                }
            }

            pub fn write<W: ::std::io::Write>(
                &self,
                ctx: &mut kafka_protocol::SerializeCtx<W>,
            ) -> Result<(), kafka_protocol::CodecError> {
                match self {
                    #(#resp_write_impls),*
                }
            }
        }
    });

    file_contents
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::schema;
    #[test]
    fn tokenize_struct() {
        assert_eq!(
            quote! {
            #[derive(Debug, PartialEq, KafkaRpc, Clone)]
            pub struct Metadata {
                #[doc = "some docs"]
                #[kafka(added = 0i16, removed = 3i16, default = "32")]
                pub foo: Option<Vec<i32> >
            }
            }
            .to_string(),
            RpcStructSpec {
                struct_name: "Metadata".to_string(),
                rpc_type: schema::ParsedSchemaType::Request,
                api_key: 0,
                fields: vec![RpcFieldSpec {
                    collection: true,
                    nullable: true,
                    field_name: "foo".to_string(),
                    type_name: "Int32".to_string(),
                    version_added: 0,
                    version_removed: Some(3),

                    default: Some(String::from("32")),
                    docstring: Some(String::from("some docs")),
                }]
            }
            .into_token_stream()
            .to_string()
        );
    }
    #[test]
    fn tokenize_field() {
        assert_eq!(
            quote! {
            # [ doc = "some docs" ]
            # [ kafka ( added = 0i16 , removed = 3i16 , default = "32" ) ]
            pub foo : Option < Vec < i32 > >
            }
            .to_string(),
            RpcFieldSpec {
                collection: true,
                nullable: true,
                field_name: "foo".to_string(),
                type_name: "Int32".to_string(),
                version_added: 0,
                version_removed: Some(3),

                default: Some(String::from("32")),
                docstring: Some(String::from("some docs")),
            }
            .into_token_stream()
            .to_string()
        );

        assert_eq!(
            quote! {
            # [ doc = "some docs" ]
            # [ kafka ( added = 0i16 , removed = 3i16 , default = "32" ) ]
            pub foo : Option < i32 >
            }
            .to_string(),
            RpcFieldSpec {
                collection: false,
                nullable: true,
                field_name: "foo".to_string(),
                type_name: "Int32".to_string(),
                version_added: 0,
                version_removed: Some(3),

                default: Some(String::from("32")),
                docstring: Some(String::from("some docs")),
            }
            .into_token_stream()
            .to_string()
        );

        assert_eq!(
            quote! {
            # [ doc = "some docs" ]
            # [ kafka ( added = 0i16, default = "32" ) ]
            pub foo : i32
            }
            .to_string(),
            RpcFieldSpec {
                collection: false,
                nullable: false,
                field_name: "foo".to_string(),
                type_name: "Int32".to_string(),
                version_added: 0,
                version_removed: None,

                default: Some(String::from("32")),
                docstring: Some(String::from("some docs")),
            }
            .into_token_stream()
            .to_string()
        );
    }
}

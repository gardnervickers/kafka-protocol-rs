//! Automatic `derive` for [KafkaRpc] types, supporting version elision and default types.
//!
//! The [Kafka Protocol](https://kafka.apache.org/protocol) is rather unique in that it versions
//! each RPC type individually. Any change to the client-facing API requires a version bump,
//! including new errors and interpretation of old fields.
//!
//! Because the product of RPCs x Versions can be in the hundreds, its difficult to develop an
//! ergonomic, type safe API.
//!
//! Instead, this crate attempts to unify the representation of each version of an RPC across the
//! superset of types contained within each RPC. This is done by allowing `default` field
//! annotations and `added`/`removed` version annotations.
//!
//! For example, to specify a Kafka [`MetadataRequest`](https://github.com/apache/kafka/blob/trunk/clients/src/main/resources/common/message/MetadataRequest.json):
//!
//! ```rust
//! use kafka_protocol_derive::KafkaRpc;
//! #[derive(KafkaRpc)]
//! #[kafka(added = 0)]
//! struct MetadataRequestTopic {
//!   #[kafka(added = 0)]
//!   name: String
//! }
//! #[derive(KafkaRpc)]
//! struct MetadataRequest {
//!   topics: Option<Vec<MetadataRequestTopic>>,
//!   #[kafka(added = 4, default = "true")]
//!   allow_auto_creation: bool,
//!   #[kafka(added = 8)]
//!   include_cluster_authorize_operations: bool,
//!   #[kafka(added = 8)]
//!   include_topic_authorized_operations: bool,
//! }
//! ```
//! This will create implementations of [KafkaRpcType] and [KafkaRpc] for both the `MetadataRequest`
//! struct and the `MetadataRequestTopic` struct. The `#[kafka(added)` and `#[kafka(removed)`
//! attributes can be specified to indicate the applicable version range for serializing and
//! deserializing fields. If a `#[kafka(default)` field is specified, it will be used instead of
//! `std::default::Default::default()` for that field when constructing the type for a version where
//! the field is not present.
//!
//! In addition to [KafkaRpcType], each struct will have a [Default] implementation which
//! incorporates the specified `#[kafka(default)]` values, otherwise falling back to the
//! `Default::default()` value for the type.
//!
//! [KafkaRpcType] : kafka_protocol::KafkaRpcType
//! [KafkaRpc] : kafka_protocol::KafkaRpc
//! [Default] : std::default::Default

#![recursion_limit = "128"]
#![crate_type = "proc-macro"]
extern crate proc_macro;
use darling::{self, FromDeriveInput, FromField};
use proc_macro::TokenStream;
use proc_macro2::{self, Ident};
use quote::{quote, ToTokens};
use syn::{self, parse_macro_input, Attribute};

#[proc_macro_derive(KafkaRpc, attributes(kafka))]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as syn::DeriveInput);
    gen(ast).into()
}

fn gen(ast: syn::DeriveInput) -> proc_macro2::TokenStream {
    let opts: KafkaRpcOpts = match KafkaRpcOpts::from_derive_input(&ast) {
        Ok(val) => val,
        Err(err) => {
            return err.write_errors();
        }
    };
    opts.render()
}

#[derive(Debug, Clone, FromField)]
#[darling(attributes(kafka), forward_attrs(doc, cfg, allow))]
struct Field {
    ident: Option<Ident>,
    attrs: Vec<Attribute>,
    vis: syn::Visibility,
    ty: syn::Type,
    #[darling(default)]
    added: i16,
    #[darling(default)]
    removed: Option<i16>,
    #[darling(default)]
    default: Option<String>,
}

#[derive(Debug, Clone, FromDeriveInput)]
#[darling(attributes(kafka), supports(struct_named))]
struct KafkaRpcOpts {
    ident: Ident,
    attrs: Vec<Attribute>,
    #[darling(default)]
    added: i16,
    #[darling(default)]
    removed: Option<i16>,
    #[darling(default)]
    ignorable: bool,
    data: darling::ast::Data<darling::util::Ignored, Field>,
}

fn parse_default_block(input: String) -> syn::Block {
    syn::parse_str(&format!("{{{}}}", input))
        .map_err(|e| format!("{}", e))
        .expect("invalid default value")
}

impl KafkaRpcOpts {
    fn version_impls(&self) -> proc_macro2::TokenStream {
        let added = self.added;
        let removed = self
            .removed
            .map_or(quote! { None }, |v| quote! { Some(#v) });
        quote! {
            fn version_added() -> i16 {
                return #added
            }

            fn version_removed() -> Option<i16> {
                return #removed
            }
        }
    }
    fn default_impls(&self) -> proc_macro2::TokenStream {
        if self.data.is_enum() {
            panic!("enum derivations are unsupported");
        } else {
            let fields = self.extract_fields();
            let mut tokens = proc_macro2::TokenStream::new();
            if fields.fields.is_empty() {
                return quote! { Self {} };
            }
            let mut field_set = Vec::new();

            for field in fields.fields.into_iter() {
                let id = field.ident.expect("only named fields are supported");
                let default_expr = field
                    .default
                    .map(|v| parse_default_block(v).into_token_stream())
                    .unwrap_or(quote! {::std::default::Default::default()});
                tokens.extend(quote! {
                    let #id = #default_expr;
                });
                field_set.push(id);
            }
            tokens.extend(quote! {
               Self { #(#field_set),* }
            });
            tokens
        }
    }

    fn extract_fields(&self) -> darling::ast::Fields<Field> {
        self.data
            .clone()
            .take_struct()
            .expect("expected struct fields only")
    }

    /// Generate the size calculation impls of KafkaRpcType
    fn size_impls(&self) -> proc_macro2::TokenStream {
        if self.data.is_enum() {
            panic!("enum derivations are unsupported");
        } else {
            let fields = self.extract_fields();
            let mut tokens = proc_macro2::TokenStream::new();
            if fields.fields.is_empty() {
                return quote! { 0 };
            }
            tokens.extend(quote! { let mut size = 0; });
            for field in fields.fields.into_iter() {
                let id = field.ident.expect("only named fields are supported");
                let ty = field.ty;
                let added_version = field.added;
                let removed_version_check = field
                    .removed
                    .map(|removed_v| quote! { && version < #removed_v});
                let version_check = quote! { (version >= #added_version #removed_version_check) };
                tokens.extend(quote! {
                    if #version_check {
                       size += <#ty as kafka_protocol::KafkaRpcType>::size(&self.#id, version);
                    }
                });
            }
            tokens.extend(quote! {return size;});
            tokens
        }
    }

    fn read_impls(&self) -> proc_macro2::TokenStream {
        if self.data.is_enum() {
            panic!("enum derivations are unsupported");
        } else {
            let fields = self.extract_fields();
            let mut tokens = proc_macro2::TokenStream::new();
            if fields.fields.is_empty() {
                return quote! { Ok(Self {}) };
            }
            let mut field_set = Vec::new();

            for field in fields.fields.into_iter() {
                let id = field.ident.expect("only named fields are supported");
                let ty = field.ty;
                let added_version = field.added;
                let removed_version_check = field
                    .removed
                    .map(|removed_v| quote! { && ctx.version < #removed_v});
                let version_check =
                    quote! { (ctx.version >= #added_version #removed_version_check) };
                let default_expr = field
                    .default
                    .map(|v| parse_default_block(v).into_token_stream())
                    .unwrap_or(quote! {::std::default::Default::default()});

                tokens.extend(quote! {
                let #id = if #version_check {
                             <#ty as kafka_protocol::KafkaRpcType>::read(ctx)?
                          } else {
                             #default_expr
                          };
                });
                field_set.push(id);
            }
            tokens.extend(quote! {
               Ok(Self { #(#field_set),* })
            });
            tokens
        }
    }

    fn write_impls(&self) -> proc_macro2::TokenStream {
        if self.data.is_enum() {
            panic!("enum derivations are unsupported");
        } else {
            let fields = self.extract_fields();
            let mut tokens = proc_macro2::TokenStream::new();
            if fields.fields.is_empty() {
                return quote! { Ok(()) };
            }
            let mut field_set = Vec::new();

            for field in fields.fields.into_iter() {
                let id = field.ident.expect("only named fields are supported");
                let added_version = field.added;
                let removed_version_check = field
                    .removed
                    .map(|removed_v| quote! { && ctx.version < #removed_v});
                let version_check =
                    quote! { (ctx.version >= #added_version #removed_version_check) };
                tokens.extend(quote! {
                if #version_check {
                   self.#id.write(ctx)?;
                }
                });
                field_set.push(id);
            }
            tokens.extend(quote! {
               Ok(())
            });
            tokens
        }
    }

    fn render(&self) -> proc_macro2::TokenStream {
        let ident = &self.ident;
        let default_impls = self.default_impls();
        let version_impls = self.version_impls();
        let size_impls = self.size_impls();
        let read_impls = self.read_impls();
        let write_impls = self.write_impls();
        quote! {
            impl kafka_protocol::KafkaRpc for #ident {
                #version_impls
            }
            impl ::std::default::Default for #ident {
                fn default() -> Self {
                  #default_impls
                }
            }
            impl kafka_protocol::KafkaRpcType for #ident {
                fn read<R: ::std::io::Read>(ctx: &mut kafka_protocol::DeserializeCtx<R>) -> Result<Self, kafka_protocol::CodecError> {
                   #read_impls
                }
                fn size(&self, version: i16) -> usize {
                   #size_impls
                }
                fn write<W: ::std::io::Write>(&self, ctx: &mut kafka_protocol::SerializeCtx<W>) -> Result<(), kafka_protocol::CodecError> {
                   #write_impls
                }
            }
        }
    }
}

// Copyright 2020 Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use super::super::{DEFAULT_FN, SEMANTIC_DE_FN, SEMANTIC_SER_FN};
use common::{Exists, FieldType};
use helpers::{get_end_version, get_ident_attr, get_start_version, parse_field_attributes};
use quote::{format_ident, quote};
use std::collections::hash_map::HashMap;

#[derive(Debug, Eq, PartialEq, Clone)]
pub(crate) struct StructField {
    ty: syn::Type,
    name: String,
    start_version: u16,
    end_version: u16,
    attrs: HashMap<String, syn::Lit>,
}

impl Exists for StructField {
    fn start_version(&self) -> u16 {
        self.start_version
    }

    fn end_version(&self) -> u16 {
        self.end_version
    }
}

impl FieldType for StructField {
    fn ty(&self) -> syn::Type {
        self.ty.clone()
    }
}

impl StructField {
    pub fn new(
        base_version: u16,
        ast_field: syn::punctuated::Pair<&syn::Field, &syn::token::Comma>,
    ) -> Self {
        let attrs = parse_field_attributes(&ast_field.value().attrs);

        StructField {
            ty: ast_field.value().ty.clone(),
            name: ast_field.value().ident.as_ref().unwrap().to_string(),
            start_version: get_start_version(&attrs).unwrap_or(base_version),
            end_version: get_end_version(&attrs).unwrap_or_default(),
            attrs,
        }
    }

    pub fn generate_semantic_serializer(&self, target_version: u16) -> proc_macro2::TokenStream {
        // Generate semantic serializer for this field only if it does not exist in target_version.
        if !self.exists_at(target_version) {
            if let Some(semantic_ser_fn) = get_ident_attr(&self.attrs, SEMANTIC_SER_FN) {
                return quote! {
                    copy_of_self.#semantic_ser_fn(version)?;
                };
            }
        }
        quote! {}
    }

    pub fn generate_semantic_deserializer(&self, source_version: u16) -> proc_macro2::TokenStream {
        // Generate semantic deserializer for this field only if it does not exist in source_version.
        if !self.exists_at(source_version) {
            if let Some(semantic_de_fn) = get_ident_attr(&self.attrs, SEMANTIC_DE_FN) {
                // Object is an instance of the structure.
                return quote! {
                    object.#semantic_de_fn(version)?;
                };
            }
        }
        quote! {}
    }

    pub fn generate_serializer(&self, target_version: u16) -> proc_macro2::TokenStream {
        let field_ident = format_ident!("{}", self.name);

        // Generate serializer for this field only if it exists in target_version.
        if !self.exists_at(target_version) {
            return proc_macro2::TokenStream::new();
        }

        // For foreign types (i.e. types used from external crates, that have their own app
        // version domain) we invoke serialize/deserialize using the version recorded in the
        // version map instead of the local app version.
        let mut token_stream = quote! {
            let tid = std::any::TypeId::of::<Self>();

            let app_version = if crate::FOREIGN_TYPES.contains(&tid) {
                crate::VERSION_MAP.get_type_version(app_version, tid)
            } else {
                app_version
            };
        };

        token_stream.extend(match &self.ty {
            syn::Type::Array(_) => quote! {
                Versionize::serialize(&copy_of_self.#field_ident.to_vec(), writer, app_version)?;
            },
            syn::Type::Path(_) => quote! {
                Versionize::serialize(&copy_of_self.#field_ident, writer, app_version)?;
            },
            syn::Type::Reference(_) => quote! {
                Versionize::serialize(&copy_of_self.#field_ident, writer, app_version)?;
            },
            _ => panic!("Unsupported field type {:?}", self.ty),
        });

        token_stream
    }

    pub fn generate_deserializer(&self, source_version: u16) -> proc_macro2::TokenStream {
        let field_ident = format_ident!("{}", self.name);

        // If the field does not exist in source version, use default annotation or Default trait.
        if !self.exists_at(source_version) {
            if let Some(default_fn) = get_ident_attr(&self.attrs, DEFAULT_FN) {
                return quote! {
                    // The default_fn is called with source version of the struct:
                    // - `version` is set to crate::VERSION_MAP.get_type_version(app_version, Self::type_id());
                    // - `app_version` is source application version.
                    #field_ident: Self::#default_fn(version),
                };
            } else {
                return quote! { #field_ident: Default::default(), };
            }
        }

        let ty = &self.ty;
        match ty {
            syn::Type::Array(array) => {
                let array_type_token;
                let array_len: usize;

                match *array.elem.clone() {
                    syn::Type::Path(token) => {
                        array_type_token = token;
                    }
                    _ => panic!("Unsupported array type."),
                }

                match &array.len {
                    syn::Expr::Lit(expr_lit) => match &expr_lit.lit {
                        syn::Lit::Int(lit_int) => array_len = lit_int.base10_parse().unwrap(),
                        _ => panic!("Unsupported array len literal."),
                    },
                    _ => panic!("Unsupported array len expression."),
                }

                quote! {
                    #field_ident: {
                        let v: Vec<#array_type_token> = <Vec<#array_type_token> as Versionize>::deserialize(&mut reader, app_version)?;
                        vec_to_arr_func!(transform_vec, #array_type_token, #array_len);
                        transform_vec(&v)
                    },
                }
            }
            syn::Type::Path(_) => quote! {
                #field_ident: {
                   let tid = std::any::TypeId::of::<#ty>();
                        let app_version = if crate::FOREIGN_TYPES.contains(&tid) {
                        crate::VERSION_MAP.get_type_version(app_version, tid)
                    } else {
                        app_version
                    };
                    <#ty as Versionize>::deserialize(&mut reader, app_version)?
                },
            },
            syn::Type::Reference(_) => quote! {
                #field_ident: <#ty as Versionize>::deserialize(&mut reader, app_version)?,
            },
            _ => panic!("Unsupported field type {:?}", self.ty),
        }
    }
}

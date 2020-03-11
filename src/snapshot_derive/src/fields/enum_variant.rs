// Copyright 2020 Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

use super::super::DEFAULT_FN;
use common::Exists;
use helpers::{get_end_version, get_ident_attr, get_start_version, parse_field_attributes};
use quote::quote;
use std::collections::hash_map::HashMap;

#[derive(Debug, Eq, PartialEq, Clone)]
pub(crate) struct EnumVariant {
    ident: syn::Ident,
    discriminant: u16, // Only u16 discriminants allowed.
    start_version: u16,
    end_version: u16,
    attrs: HashMap<String, syn::Lit>,
}

impl Exists for EnumVariant {
    fn start_version(&self) -> u16 {
        self.start_version
    }

    fn end_version(&self) -> u16 {
        self.end_version
    }
}

impl EnumVariant {
    pub fn new(base_version: u16, ast_variant: &syn::Variant) -> Self {
        let discriminant: u16;
        if let Some(discriminant_expr) = &ast_variant.discriminant {
            // We only support ExprLit
            match &discriminant_expr.1 {
                syn::Expr::Lit(lit_expr) => match &lit_expr.lit {
                    syn::Lit::Int(lit_int) => {
                        // Get variant discriminant as u16.
                        discriminant = lit_int.base10_parse().unwrap()
                    }
                    _ => panic!("A u16 discriminant is required for versioning Enums."),
                },
                _ => panic!("A u16 discriminant is required for versioning Enums."),
            }
        } else {
            panic!("A u16 discriminant is required for versioning Enums.")
        }

        let attrs = parse_field_attributes(&ast_variant.attrs);
        EnumVariant {
            ident: ast_variant.ident.clone(),
            discriminant,
            // Set base version.
            start_version: get_start_version(&attrs).unwrap_or(base_version),
            end_version: get_end_version(&attrs).unwrap_or_default(),
            attrs,
        }
    }

    // Emits code that serializes an enum variant.
    pub fn generate_serializer(&self, target_version: u16) -> proc_macro2::TokenStream {
        let field_ident = &self.ident;

        if !self.exists_at(target_version) {
            if let Some(default_fn_ident) = get_ident_attr(&self.attrs, DEFAULT_FN) {
                return quote! {
                    Self::#field_ident => {
                        let variant = self.#default_fn_ident(version);
                        bincode::serialize_into(writer, &variant).map_err(|ref err| Error::Serialize(format!("{:?}", err)))?;
                    },
                };
            } else {
                panic!("Variant {} does not exist in version {}, please implement a default_fn function that provides a default value for this variant.", field_ident.to_string(), target_version);
            }
        }

        quote! {
            Self::#field_ident => {
                bincode::serialize_into(writer, &self).map_err(|ref err| Error::Serialize(format!("{:?}", err)))?;
            },
        }
    }
}

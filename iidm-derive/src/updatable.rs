use convert_case::{Case, Casing};
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Data, DeriveInput, Fields, LitStr};

pub fn impl_error_for_struct(ast: &DeriveInput) -> TokenStream {
    let struct_name = &ast.ident;
    let error_name = format!("{}Error", struct_name);
    let error_ident = syn::Ident::new(&error_name, struct_name.span());

    let fields = match &ast.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => panic!("Only named fields are supported"),
        },
        _ => panic!("Only structs are supported"),
    };

    let error_variants = fields.iter().map(|f| {
        let field_name = &f.ident;
        let variant_name = field_name.as_ref().unwrap().to_string();

        // Get the serde rename attribute if it exists
        let rename = f
            .attrs
            .iter()
            .find(|attr| attr.path().is_ident("serde"))
            .and_then(|attr| {
                let mut rename_value = None;
                attr.parse_nested_meta(|meta| {
                    if meta.path.is_ident("rename") {
                        rename_value = Some(meta.value()?.parse::<LitStr>()?.value());
                    }
                    Ok(())
                })
                .ok();
                rename_value
            })
            .unwrap_or_else(|| variant_name.clone());

        let error_message = format!("{} error: {{0}}", rename);
        let variant_ident = syn::Ident::new(&variant_name.to_case(Case::Pascal), Span::call_site());

        quote! {
            #[error(#error_message)]
            #variant_ident(String)
        }
    });

    quote::quote! {
        #[derive(Debug, thiserror::Error)]
        pub enum #error_ident {
            #(#error_variants,)*

            #[error("Deserialization error: {0}")]
            Deserialization(#[from] serde_json::Error),

            #[error("Date parsing error: {0}")]
            DateParse(#[from] chrono::ParseError),

            #[error("Unknown error: {0}")]
            Unknown(String)
        }
    }
    .into()
}

pub fn impl_updatable_trait(ast: DeriveInput) -> TokenStream {
    // get struct identifier
    let name = &ast.ident;
    let update_name = syn::Ident::new(&format!("{}Updater", name), name.span());
    let error_name = syn::Ident::new(&format!("{}Error", name), name.span());

    // generate the error type
    let error_type = impl_error_for_struct(&ast);

    // generate fields
    let fields = match ast.data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => &fields.named,
            _ => panic!("Updatable only supports named fields"),
        },
        _ => panic!("Updatable only supports structs"),
    };

    let field_defs = fields
        .iter()
        .filter(|f| f.ident.as_ref().map_or(true, |id| id != "id"))
        .map(|f| {
            let name = &f.ident;
            let ty = &f.ty;
            let attrs = &f.attrs;
            quote::quote! {
                #(#attrs)*
                pub #name: Option<#ty>
            }
        });

    // Generate fields_json implementation by getting all field names with serde renames
    let fields_json_impl = fields
        .iter()
        .filter(|f| f.ident.as_ref().map_or(true, |id| id != "id"))
        .map(|f| {
            let field_name = f.ident.as_ref().unwrap().to_string();

            // Get the serde rename attribute if it exists
            let rename = f
                .attrs
                .iter()
                .find(|attr| attr.path().is_ident("serde"))
                .and_then(|attr| {
                    let mut rename_value = None;
                    attr.parse_nested_meta(|meta| {
                        if meta.path.is_ident("rename") {
                            rename_value = Some(meta.value()?.parse::<LitStr>()?.value());
                        }
                        Ok(())
                    })
                    .ok();
                    rename_value
                })
                .unwrap_or_else(|| field_name.clone());

            quote::quote! {
                #rename.to_string()
            }
        });

    let update_impl = fields
        .iter()
        .filter(|f| f.ident.as_ref().map_or(true, |id| id != "id"))
        .map(|f| {
            let name = &f.ident;
            quote::quote! {
                if let Some(value) = updates.#name {
                    self.#name = value;
                }
            }
        });

    // generate impl
    quote::quote! {

        // First include the error enum
        #error_type

        #[derive(Default, Clone, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
        #[serde(default)]
        pub struct #update_name {
            #(#field_defs,)*
        }

        impl crate::extensions::JsonSchema for #update_name {
            type Err = #error_name;

            fn fields_json() -> Vec<String> {
                vec![
                    #(#fields_json_impl,)*
                ]
            }

            fn validate_json(json: &str) -> Result<Self, Self::Err> {
                crate::libs::json::validate_json(json).map_err(|e| Self::Err::Deserialization(e))
            }
        }

        impl Updatable for #name {

            type Updater = #update_name;
            type Err = #error_name;

            fn update(&mut self, updates: Self::Updater) {
                #(#update_impl)*
            }

            fn update_from_json(&mut self, json: &str) -> Result<(), Self::Err> {
                serde_json::from_str(json)
                    .map_err(|e| Self::Err::Deserialization(e))
                    .and_then(|updates| {
                        self.update(updates);
                        Ok(())
                    })
            }

        }
    }
    .into()
}

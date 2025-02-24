use convert_case::{Case, Casing};
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Data, DeriveInput, Fields, LitStr};

// Utility to extract field name from serde rename attribute
fn extract_serde_rename(attrs: &[syn::Attribute], default_name: &str) -> String {
    attrs
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
        .unwrap_or_else(|| default_name.to_string())
}

// Utility to extract named fields from a struct
fn extract_named_fields(
    ast: &DeriveInput,
) -> &syn::punctuated::Punctuated<syn::Field, syn::token::Comma> {
    match &ast.data {
        Data::Struct(data) => match &data.fields {
            Fields::Named(fields) => &fields.named,
            _ => panic!("Only named fields are supported"),
        },
        _ => panic!("Only structs are supported"),
    }
}

pub fn impl_error_for_struct(ast: &DeriveInput) -> TokenStream {
    let struct_name = &ast.ident;
    let error_name = format!("{}Error", struct_name);
    let error_ident = syn::Ident::new(&error_name, struct_name.span());

    let fields = extract_named_fields(ast);

    let error_variants = fields.iter().map(|f| {
        let field_name = &f.ident;
        let variant_name = field_name.as_ref().unwrap().to_string();
        let rename = extract_serde_rename(&f.attrs, &variant_name);

        let error_message = format!("{} error: {{0}}", rename);
        let variant_ident = syn::Ident::new(&variant_name.to_case(Case::Pascal), Span::call_site());

        quote! {
            #[error(#error_message)]
            #variant_ident(String)
        }
    });

    quote! {
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
}

pub fn impl_updatable_trait(ast: DeriveInput) -> TokenStream {
    // Extract struct identifier
    let name = &ast.ident;
    let update_name = syn::Ident::new(&format!("{}Updater", name), name.span());
    let error_name = syn::Ident::new(&format!("{}Error", name), name.span());

    // Generate error type
    let error_type = impl_error_for_struct(&ast);

    // Extract fields
    let fields = extract_named_fields(&ast);

    // Filter to exclude 'id' field
    let filtered_fields = fields
        .iter()
        .filter(|f| f.ident.as_ref().map_or(true, |id| id != "id"));

    // Generate field definitions for the Updater
    let field_defs = filtered_fields.clone().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;
        let attrs = &f.attrs;
        quote! {
            #(#attrs)*
            pub #name: Option<#ty>
        }
    });

    // Generate implementation for fields_json
    let fields_json_impl = filtered_fields.clone().map(|f| {
        let field_name = f.ident.as_ref().unwrap().to_string();
        let rename = extract_serde_rename(&f.attrs, &field_name);

        quote! {
            #rename.to_string()
        }
    });

    // Generate implementation for update
    let update_impl = filtered_fields.map(|f| {
        let name = &f.ident;
        quote! {
            if let Some(value) = updates.#name {
                self.#name = value;
            }
        }
    });

    // Generate complete implementation
    quote! {
        // Include error enum
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::quote;
    use syn::parse_quote;

    // Fonction utilitaire pour tester le formatage du code généré
    fn assert_tokens_eq(left: TokenStream, right: TokenStream) {
        let left_str = left.to_string();
        let right_str = right.to_string();
        assert_eq!(
            left_str, right_str,
            "Les TokenStreams ne correspondent pas.\nGauche:\n{}\n\nDroite:\n{}",
            left_str, right_str
        );
    }

    #[test]
    fn test_impl_error_for_simple_struct() {
        // Déclarer une structure simple pour le test
        let input: DeriveInput = parse_quote! {
            struct User {
                name: String,
                email: String,
            }
        };

        // Générer l'implémentation d'error
        let generated = impl_error_for_struct(&input);

        // La sortie attendue
        let expected = quote! {
            #[derive(Debug, thiserror::Error)]
            pub enum UserError {
                #[error("name error: {0}")]
                Name(String),

                #[error("email error: {0}")]
                Email(String),

                #[error("Deserialization error: {0}")]
                Deserialization(#[from] serde_json::Error),

                #[error("Date parsing error: {0}")]
                DateParse(#[from] chrono::ParseError),

                #[error("Unknown error: {0}")]
                Unknown(String)
            }
        };

        assert_tokens_eq(generated, expected);
    }

    #[test]
    fn test_impl_error_with_serde_rename() {
        // Test avec des attributs serde rename
        let input: DeriveInput = parse_quote! {
            struct Product {
                #[serde(rename = "product_name")]
                name: String,
                #[serde(rename = "product_price")]
                price: f64,
            }
        };

        let generated = impl_error_for_struct(&input);

        let expected = quote! {
            #[derive(Debug, thiserror::Error)]
            pub enum ProductError {
                #[error("product_name error: {0}")]
                Name(String),

                #[error("product_price error: {0}")]
                Price(String),

                #[error("Deserialization error: {0}")]
                Deserialization(#[from] serde_json::Error),

                #[error("Date parsing error: {0}")]
                DateParse(#[from] chrono::ParseError),

                #[error("Unknown error: {0}")]
                Unknown(String)
            }
        };

        assert_tokens_eq(generated, expected);
    }

    #[test]
    fn test_impl_updatable_trait() {
        // Structure pour tester l'implémentation du trait Updatable
        let input: DeriveInput = parse_quote! {
            struct User {
                id: i64,
                name: String,
                #[serde(rename = "user_email")]
                email: String,
                active: bool,
            }
        };

        let generated = impl_updatable_trait(input);

        // Vérifier que l'Updater a été généré correctement
        assert!(generated.to_string().contains("struct UserUpdater"));

        // Vérifier que les champs ont été générés correctement (sans le champ id)
        assert!(generated
            .to_string()
            .contains("pub name : Option < String >"));
        assert!(generated
            .to_string()
            .contains("pub email : Option < String >"));
        assert!(generated
            .to_string()
            .contains("pub active : Option < bool >"));

        // Vérifier que l'implémentation de fields_json utilise les noms serde
        assert!(generated.to_string().contains("\"name\" . to_string ()"));
        assert!(generated
            .to_string()
            .contains("\"user_email\" . to_string ()"));

        // Vérifier que l'implementation de l'update a été générée
        assert!(generated.to_string().contains("impl Updatable for User"));
        assert!(generated.to_string().contains("type Updater = UserUpdater"));
        assert!(generated.to_string().contains("type Err = UserError"));
    }
}

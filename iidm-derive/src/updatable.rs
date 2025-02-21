use proc_macro::TokenStream;
use syn::{Data, DeriveInput, Fields};

pub(crate) fn impl_updatable_trait(ast: DeriveInput) -> TokenStream {
    // get struct identifier
    let name = ast.ident;
    let update_name = syn::Ident::new(&format!("{}Update", name), name.span());

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
        #[derive(Default, Deserialize)]
        #[serde(default)]
        pub struct #update_name {
            #(#field_defs,)*
        }

        impl Updatable for #name {

            type Updater = #update_name;

            fn update(&mut self, updates: Self::Updater) {
                #(#update_impl)*
            }

            fn update_from_json(&mut self, json: &str) -> Result<(), serde_json::Error> {
                let updates: #update_name = serde_json::from_str(json)?;
                self.update(updates);
                Ok(())
            }
        }
    }
    .into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;

    // Manual implementation of TestStructUpdate for comparison
    #[derive(Default, Deserialize)]
    #[serde(default)]
    struct TestStructUpdate {
        field1: Option<String>,
        field2: Option<i32>,
        field3: Option<bool>,
    }

    // #[test]
    // fn test_derive_updatable_structure() {
    //     // Verify that the macro correctly generates the Update structure
    //     let input = syn::parse_quote! {
    //         struct TestStruct {
    //             field1: String,
    //             field2: i32,
    //             field3: bool,
    //         }
    //     };

    //     let output = impl_updatable_trait(input);
    //     let expected = quote::quote! {
    //         #[derive(Default, Deserialize)]
    //         #[serde(default)]
    //         pub struct TestStructUpdate {
    //             pub field1: Option<String>,
    //             pub field2: Option<i32>,
    //             pub field3: Option<bool>,
    //         }
    //         impl TestStruct {
    //             pub fn update(&mut self, updates: TestStructUpdate) {
    //                 if let Some(value) = updates.field1 {
    //                     self.field1 = value;
    //                 }
    //                 if let Some(value) = updates.field2 {
    //                     self.field2 = value;
    //                 }
    //                 if let Some(value) = updates.field3 {
    //                     self.field3 = value;
    //                 }
    //             }
    //             pub fn update_from_json(&mut self, json: &str) -> Result<(), serde_json::Error> {
    //                 let updates: TestStructUpdate = serde_json::from_str(json)?;
    //                 self.update(updates);
    //                 Ok(())
    //             }
    //         }
    //     };

    //     assert_eq!(output.to_string(), expected.to_string());
    // }

    #[test]
    #[should_panic(expected = "Updatable only supports structs")]
    fn test_non_struct_input() {
        let input = syn::parse_quote! {
            enum TestEnum {
                Variant1,
                Variant2,
            }
        };

        impl_updatable_trait(input);
    }

    #[test]
    #[should_panic(expected = "Updatable only supports named fields")]
    fn test_tuple_struct_input() {
        let input = syn::parse_quote! {
            struct TestTuple(String, i32);
        };

        impl_updatable_trait(input);
    }
}

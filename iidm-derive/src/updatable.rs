use proc_macro::TokenStream;
use syn::{Data, DeriveInput, Fields};

pub fn impl_updatable_trait(ast: DeriveInput) -> TokenStream {
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

    let field_defs = fields.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;
        let attrs = &f.attrs;
        quote::quote! {
            #(#attrs)*
            pub #name: Option<#ty>
        }
    });

    let update_impl = fields.iter().map(|f| {
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

        impl #name {
            pub fn update(&mut self, updates: #update_name) {
                #(#update_impl)*
            }

            pub fn update_from_json(&mut self, json: &str) -> Result<(), serde_json::Error> {
                let updates: #update_name = serde_json::from_str(json)?;
                self.update(updates);
                Ok(())
            }
        }
    }
    .into()
}

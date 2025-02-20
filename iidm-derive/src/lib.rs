use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

fn impl_reflective_trait(ast: DeriveInput) -> TokenStream {
    // get struct identifier
    let ident = ast.ident;

    // generate impl
    quote::quote! {
        impl Identifiable for #ident {
            fn id(&self) -> String {
                self.id.clone()
            }
        }
    }
    .into()
}

#[proc_macro_derive(Identifiable)]
pub fn reflective_derive_macro(item: TokenStream) -> TokenStream {
    // parse
    let ast: DeriveInput = syn::parse(item).unwrap();

    // generate
    impl_reflective_trait(ast)
}

#[proc_macro_derive(Updatable)]
pub fn derive_updatable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let update_name = syn::Ident::new(&format!("{}Update", name), name.span());

    let fields = match input.data {
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
        quote! {
            #(#attrs)*
            pub #name: Option<#ty>
        }
    });

    let update_impl = fields.iter().map(|f| {
        let name = &f.ident;
        quote! {
            if let Some(value) = updates.#name {
                self.#name = value;
            }
        }
    });

    let expanded = quote! {
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
    };

    TokenStream::from(expanded)
}

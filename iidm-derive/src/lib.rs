use proc_macro::TokenStream;
use syn::DeriveInput;

fn impl_reflective_trait(ast: DeriveInput) -> TokenStream {
    // get struct identifier
    let ident = ast.ident;
    let ident_str = ident.to_string();

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

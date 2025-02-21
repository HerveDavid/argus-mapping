use proc_macro2::TokenStream;
use syn::DeriveInput;

pub fn impl_reflective_trait(ast: DeriveInput) -> TokenStream {
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

mod identifiable;
mod updatable;

use identifiable::impl_reflective_trait;
use proc_macro::TokenStream;
use syn::DeriveInput;
use updatable::impl_updatable_trait;

#[proc_macro_derive(Identifiable)]
pub fn reflective_derive_macro(item: TokenStream) -> TokenStream {
    // parse
    let ast: DeriveInput = syn::parse(item).unwrap();

    // generate
    impl_reflective_trait(ast)
}

#[proc_macro_derive(Updatable)]
pub fn derive_updatable(item: TokenStream) -> TokenStream {
    // parse
    let ast: DeriveInput = syn::parse(item).unwrap();

    // generate
    impl_updatable_trait(ast)
}

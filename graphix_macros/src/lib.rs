use proc_macro::TokenStream;

mod entity;

#[proc_macro_derive(Entity, attributes(graphix))]
pub fn entity(input: TokenStream) -> TokenStream {
    entity::derive_entity(input).into()
}
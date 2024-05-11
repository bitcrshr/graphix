use proc_macro2::TokenStream;
use std::env;
use syn::DeriveInput;

mod macros;
mod sql;

#[proc_macro_derive(Entity, attributes(graphix))]
pub fn entity(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast: DeriveInput = syn::parse_macro_input!(input as DeriveInput);

    let toks = macros::entity::entity_inner(&ast).unwrap_or_else(|err| err.to_compile_error());
    debug_print_generated(&ast, &toks);
    toks.into()
}

fn debug_print_generated(ast: &DeriveInput, toks: &TokenStream) {
    let debug = env::var("GRAPHIX_DEBUG");
    if let Ok(s) = debug {
        if s == "1" {
            println!("{}", toks);
        }

        if ast.ident == s {
            println!("{}", toks);
        }
    }
}

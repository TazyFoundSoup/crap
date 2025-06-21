use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Crap, attributes(crap))]
pub fn derive_crap(input: TokenStream) -> TokenStream {
    // Parse the input tokens into a syntax tree
    let input = parse_macro_input!(input as DeriveInput);

    // this is just debug for now
    println!("Deriving Crap for: {}", input.ident);

}
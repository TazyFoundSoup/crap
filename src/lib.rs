// This file is part of the crap command line parser.
// Licensed under the MIT License (MIT).
// Read LICENSE for details.

extern crate proc_macro;
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput, Meta, Lit};
use quote::quote;

struct Command {
    long: String,
    short: Option<char>,
    desc: String,
}

#[proc_macro_derive(Crap)]
pub fn crap_derive(input: TokenStream) -> TokenStream {
    // Parse tokens
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    // Get the enum passed to the macro
    // In this case, it will be an enum with variants respresenting commands.
    // They will also have attributes to specify other command details such as:
    //     long  - we don't have to worry because the long name is the variant name, which can be read without
    //             any attributes.
    //     short - a single character for the command
    //     desc  - a description of the command which will be used to generate a custom help message.
    // More coming soon...
    if let syn::Data::Enum(ref cmd_enum) = input.data {
        // Get all the variants of the enum and process them into a vector of the Command struct.
        let commands: Vec<Command>;
        for variant in &cmd_enum.variants {
            if variant.attrs.iter().any(|attr| attr.path().is_ident("command")) {
                if let Ok(Meta::List(meta_list)) = attr.parse_meta() {
                    for nested in meta_list.nested {
                        if let NestedMeta::Meta(Meta::NameValue(nv)) = nested {
                        // Now we can check each name-key pair in the attribute
                        // If the key is "short", we can get the value for later use
                        if nv.path.is_ident("short") {
                            if let Lit::Str(lit) = &nv.lit {
                                let short = lit.value();
                            }
                        }
                        
                    }
                    }
                }
            }
        }
    }

    TokenStream::from(expanded)
}
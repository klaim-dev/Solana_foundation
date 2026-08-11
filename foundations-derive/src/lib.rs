use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(HasName)]
pub fn has_name(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;
    let literal = name.to_string();

    let tokens = quote! {
        impl HasName for #name {
            fn name() -> &'static str {
                #literal
            }
        }
    };
    tokens.into()
}
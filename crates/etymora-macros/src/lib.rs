use proc_macro::TokenStream;

use quote::{quote, ToTokens};
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(WikTsvParser)]
pub fn tsv_parser(tokens: TokenStream) -> TokenStream {
    // let input = parse_macro_input!(tokens as DeriveInput);

    "fn add() { (|a, b| a + b) }".parse().unwrap()
}

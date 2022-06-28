use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr};

#[proc_macro]
pub fn static_array_from_file(input: TokenStream) -> TokenStream {
    input
}

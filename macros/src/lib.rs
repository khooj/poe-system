use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    token::Comma,
    Expr, Ident, LitStr, Token,
};

struct StaticMacro {
    array_name: Ident,
    filename: String,
}

impl Parse for StaticMacro {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name: Ident = input.parse()?;
        let _: Comma = input.parse()?;
        let filename: LitStr = input.parse()?;

        Ok(StaticMacro {
            array_name: name,
            filename: filename.value(),
        })
    }
}

#[proc_macro]
pub fn static_array_from_file(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as StaticMacro);
    let name = input.array_name;

    let tokens = quote! {
        static #name: &[&'static str] = &[];
    };

    tokens.into()
}

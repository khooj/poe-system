use std::{
    env, fs,
    io::{BufRead, BufReader},
};

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    token::Comma,
    Ident, LitStr,
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
    let filename = input.filename;

    let mut c = env::current_dir().unwrap();
    c.push(&filename);
    let f = fs::OpenOptions::new().read(true).open(&c);
    if f.is_err() {
        let e = format!("cant open file: {}", c.to_string_lossy());
        return quote!(::std::compile_error!(#e);).into();
    }

    let mut buf = BufReader::new(f.unwrap());
    let mut sv = vec![];
    let mut s = String::new();
    while let Ok(sz) = buf.read_line(&mut s) {
        if sz == 0 {
            break;
        }
        sv.push(s.clone());
        s.clear();
    }

    let tokens = quote! {
        static #name: &[&'static str] = &[ #(#sv),* ];
    };

    tokens.into()
}

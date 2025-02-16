use std::{
    env, fs,
    io::{BufRead, BufReader},
};

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, Ident, LitStr,
};

struct StaticMacro {
    filename: String,
}

impl Parse for StaticMacro {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let filename: LitStr = input.parse()?;

        Ok(StaticMacro {
            filename: filename.value(),
        })
    }
}

#[proc_macro]
pub fn static_array_from_file(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as StaticMacro);
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
        sv.push(s.clone().trim().to_owned());
        s.clear();
    }

    let tokens = quote! {
        { &[ #(#sv),* ] }
    };

    tokens.into()
}

struct GenSetMethod {
    name: Ident,
}

impl Parse for GenSetMethod {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let name: Ident = input.parse()?;

        Ok(GenSetMethod { name })
    }
}

#[proc_macro]
pub fn gen_min_max_method(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as GenSetMethod);
    let funcname = format_ident!("set_{}", input.name);
    let name = syn::LitStr::new(&input.name.to_string(), input.name.span());
    let tokens = quote! {
        fn #funcname(mut self, min: Option<i32>, max: Option<i32>) -> Self {
            let v = json!({
                "min": min,
                "max": max,
            });
            let m = self.filters.entry(#name.to_string()).or_default();
            *m = v;
            self
        }
    };
    tokens.into()
}

#[proc_macro]
pub fn gen_option_method(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as GenSetMethod);
    let funcname = format_ident!("set_{}", input.name);
    let name = syn::LitStr::new(&input.name.to_string(), input.name.span());
    let tokens = quote! {
        fn #funcname(mut self, val: bool) -> Self {
            let v = json!({
                "option": val,
            });
            let m = self.filters.entry(#name.to_string()).or_default();
            *m = v;
            self
        }
    };
    tokens.into()
}

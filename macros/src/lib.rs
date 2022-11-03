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
        sv.push(s.clone().trim().to_owned());
        s.clear();
    }

    let tokens = quote! {
        pub static #name: &[&'static str] = &[ #(#sv),* ];
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
    let funcname = format!("set_{}", input.name);
    let funcname = Ident::new(&funcname, input.name.span());
    let name = syn::LitStr::new(&input.name.to_string(), input.name.span());
    let tokens = quote! {
        fn #funcname(&mut self, min: Option<i32>, max: Option<i32>) {
            let v = json!({
                "min": min,
                "max": max,
            });
            let m = self.filters.entry(#name.to_string()).or_default();
            *m = v;
        }
    };
    tokens.into()
}

#[proc_macro]
pub fn gen_option_method(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as GenSetMethod);
    let funcname = format!("set_{}", input.name);
    let funcname = Ident::new(&funcname, input.name.span());
    let name = syn::LitStr::new(&input.name.to_string(), input.name.span());
    let tokens = quote! {
        fn #funcname(&mut self, val: bool) {
            let v = json!({
                "option": val,
            });
            let m = self.filters.entry(#name.to_string()).or_default();
            *m = v;
        }
    };
    tokens.into()
}

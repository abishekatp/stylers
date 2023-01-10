#![feature(proc_macro_span)]
use proc_macro::TokenStream;
mod builder;

#[proc_macro]
pub fn style(ts: TokenStream) -> TokenStream {   
    let _class_name = builder::build_style(ts.into());
    "1+2".parse().unwrap()
}



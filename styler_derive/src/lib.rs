#![feature(proc_macro_span)]
use proc_macro::TokenStream;
use quote::quote;
use std::collections::hash_map::RandomState;
use std::hash::{BuildHasher, Hasher};
mod builder;

#[proc_macro]
pub fn style(ts: TokenStream) -> TokenStream {
    let random_class = rand_class();
    builder::build_style(ts, &random_class);
    let expanded = quote! {
        let class = #random_class;
    };
    TokenStream::from(expanded)
}

fn rand_class() -> String {
    let hash = RandomState::new().build_hasher().finish().to_string();
    let k = &hash[0..6];
    format!(".l-{}", k.to_string())
}

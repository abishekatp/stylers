#![feature(proc_macro_span)]
use proc_macro::TokenStream;
mod builder;

#[proc_macro]
pub fn style(ts: TokenStream) -> TokenStream {
    // println!("{:#?}",ts);
    let ts = builder::get_style_string(ts.into());
    println!("============================================================");
    println!("{}",ts);
    println!("============================================================");
    //todo: use quote here.
    
    "1+2".parse().unwrap()
}



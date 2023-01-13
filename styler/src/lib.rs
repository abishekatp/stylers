#![feature(proc_macro_span)]
use proc_macro::TokenStream;
use proc_macro2;
use quote::quote;

use std::collections::hash_map::RandomState;
use std::hash::{BuildHasher, Hasher};

use std::fs::{File, OpenOptions};
use std::io::{ErrorKind, Write};
use styler_core::build_style;

#[proc_macro]
pub fn style(ts: TokenStream) -> TokenStream {
    let random_class = rand_class();
    let (style, _sel_map) = build_style(proc_macro2::TokenStream::from(ts), &random_class);
    // dbg!(&sel_map);
    let random_class = random_class[1..].to_string();
    let expanded = quote! {
        let class_name = #random_class;
    };
    // dbg!(&style);
    write_to_file(&style);
    TokenStream::from(expanded)
    // let call_site = proc_macro::Span::call_site();
    // dbg!(&call_site);
    // println!("{}",call_site.source_text().unwrap());
    // println!("{:?}",call_site.source_file());
}

//this macro will return the style string. Note:created for testing purpose only.
#[proc_macro]
pub fn style_str(ts: TokenStream) -> TokenStream {
    let random_class = String::from(".test");
    let (style, _sel_map) = build_style(proc_macro2::TokenStream::from(ts), &random_class);
    let expanded = quote! {
        #style
    };
    TokenStream::from(expanded)
}

fn rand_class() -> String {
    let hash = RandomState::new().build_hasher().finish().to_string();
    let k = &hash[0..6];
    format!(".l-{}", k.to_string())
}

//append if file exists or write it into the new file
fn write_to_file(data: &str) {
    let file_name = "main.css";
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(file_name)
        .unwrap_or_else(|err| {
            if err.kind() == ErrorKind::NotFound {
                File::create(file_name).unwrap_or_else(|err| {
                    panic!("Problem creating the file: {:?}", err);
                })
            } else {
                panic!("Problem opening the file: {:?}", err);
            }
        });
    let _ = file
        .write_all(data.as_bytes())
        .expect("Problem writing to file");
}
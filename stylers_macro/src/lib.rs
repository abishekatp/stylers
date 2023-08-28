//! This crate provides style macro for scoped css in rust web frameworks which follows component like architecture e.g Leptos.
use proc_macro::TokenStream;
use quote::quote;

use std::collections::hash_map::RandomState;
use std::hash::{BuildHasher, Hasher};
use stylers_core::rand_class_from_seed;
use stylers_core::{from_str, from_ts};

/// style macro take any valid css as input and returns a unique class name.
/// For examples see: <https://github.com/abishekatp/stylers>
#[proc_macro]
pub fn style(ts: TokenStream) -> TokenStream {
    let strval = ts.to_string();
    let random_class = rand_class_from_seed(strval);
    let random_class = random_class[1..].to_string();
    let expanded = quote! {
        #random_class
    };
    TokenStream::from(expanded)
}

/// style_sheet macro take css file path as a string input and returns a unique class name.
/// For examples see: <https://github.com/abishekatp/stylers>
#[proc_macro]
pub fn style_sheet(ts: TokenStream) -> TokenStream {
    let file_path = ts.to_string();
    let file_path = file_path.trim_matches('"');
    let css_content = std::fs::read_to_string(&file_path).expect("Expected to read file");
    let random_class = rand_class_from_seed(css_content.to_string());
    let random_class = random_class[1..].to_string();
    let expanded = quote! {
        #random_class
    };
    TokenStream::from(expanded)
}

/// style_str macro any valid css as input and returns a tuple (unique_class_name,style_string).
/// note: this macro does not require a component name like style macro
/// For examples see: <https://github.com/abishekatp/stylers>
#[proc_macro]
pub fn style_str(ts: TokenStream) -> TokenStream {
    let random_class = rand_class();
    let (style, _sel_map) = from_ts(ts.into(), &random_class, true);
    let random_class = random_class[1..].to_string();
    let expanded = quote! {
        (#random_class,#style)
    };
    TokenStream::from(expanded)
}

/// style_sheet_str macro take css file path as a string input and returns a tuple (unique_class_name,style_string).
/// For examples see: <https://github.com/abishekatp/stylers>
#[proc_macro]
pub fn style_sheet_str(ts: TokenStream) -> TokenStream {
    let file_path = ts.to_string();
    let file_path = file_path.trim_matches('"');
    let css_content = std::fs::read_to_string(&file_path).expect("Expected to read file");
    let random_class = rand_class();
    let style = from_str(&css_content, &random_class);
    let random_class = random_class[1..].to_string();
    let expanded = quote! {
        (#random_class,#style)
    };
    TokenStream::from(expanded)
}

// This style_sheet_test macro will return the style string.
// Note:created for testing purpose only.
#[proc_macro]
pub fn style_sheet_test(ts: TokenStream) -> TokenStream {
    let file_path = ts.to_string();
    let file_path = file_path.trim_matches('"');
    let css_content = std::fs::read_to_string(&file_path).expect("Expected to read file");
    let random_class = String::from(".test");
    let style = from_str(&css_content, &random_class);
    let expanded = quote! {
        #style
    };
    TokenStream::from(expanded)
}

// This style_test macro will return the style string.
// Note:created for testing purpose only.
#[proc_macro]
pub fn style_test(ts: TokenStream) -> TokenStream {
    let random_class = String::from(".test");
    let (style, _sel_map) = from_ts(ts.into(), &random_class, true);
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

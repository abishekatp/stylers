//! This crate provides style macro for scoped css in rust web frameworks which follows component like architecture e.g Leptos.
use proc_macro::TokenStream;
use quote::quote;

use stylers_core::Class;
use stylers_core::{from_str, from_ts};

/// style macro take any valid css as input and returns a unique class name.
/// For examples see: <https://github.com/abishekatp/stylers>
#[proc_macro]
pub fn style(ts: TokenStream) -> TokenStream {
    let strval = ts.to_string();
    let class = Class::rand_class_from_seed(strval);
    let class = class.as_name();
    let expanded = quote! {
        #class
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
    let class = Class::rand_class_from_seed(css_content.to_string());
    let class = class.as_name();
    let expanded = quote! {
        #class
    };
    TokenStream::from(expanded)
}

/// style_str macro any valid css as input and returns a tuple (unique_class_name,style_string).
/// note: this macro does not require a component name like style macro
/// For examples see: <https://github.com/abishekatp/stylers>
#[proc_macro]
pub fn style_str(ts: TokenStream) -> TokenStream {
    let class = Class::random();
    let (style, _sel_map) = from_ts(ts.into(), &class, true);
    let class = class.as_name();
    let expanded = quote! {
        (#class,#style)
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
    let class = Class::random();
    let style = from_str(&css_content, &class);
    let class = class.as_name();
    let expanded = quote! {
        (#class,#style)
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
    let class = Class::new(String::from("test"));
    let style = from_str(&css_content, &class);
    let expanded = quote! {
        #style
    };
    TokenStream::from(expanded)
}

// This style_test macro will return the style string.
// Note:created for testing purpose only.
#[proc_macro]
pub fn style_test(ts: TokenStream) -> TokenStream {
    let class = Class::new(String::from("test"));
    let (style, _sel_map) = from_ts(ts.into(), &class, true);
    let expanded = quote! {
        #style
    };
    TokenStream::from(expanded)
}

//! This create as of now only exposes one function named build_style.
//! The main focus of this function is to provide scoped css for Rust components(for the framework which provides component like architecture e.g leptos).
//! This function can be used parse the style sheet in rust.
mod css_at_rule;
mod css_style_declar;
mod css_style_rule;
mod css_style_sheet;
mod utils;
use proc_macro2::TokenStream;
use std::collections::HashMap;

pub(crate) use crate::style::css_at_rule::CSSAtRule;
pub(crate) use crate::style::css_style_declar::CSSStyleDeclaration;
pub(crate) use crate::style::css_style_rule::CSSStyleRule;
pub(crate) use crate::style::css_style_sheet::{CSSRule, CSSStyleSheet};

/// This function will build the whole style text as rust TokenStream.
/// This function will take two arguments.
/// ts: TokenStream which is token stream of text content of whole style sheet.
/// random_class: &String is random class to be appended for each selector.
/// This function will return tuple with two fields (style string, map of unique keys of selectors.)
/// style string: is the parsed style sheet as a string
pub fn build_style_from_ts(
    ts: TokenStream,
    random_class: &String,
    is_proc_macro: bool,
) -> (String, HashMap<String, ()>) {
    let mut style = String::new();
    let (style_sheet, sel_map) = CSSStyleSheet::new(ts, random_class, is_proc_macro);
    style_sheet.css_rules.iter().for_each(|rule| match rule {
        CSSRule::AtRule(at_rule) => style.push_str(&at_rule.css_text()),
        CSSRule::StyleRule(style_rule) => style.push_str(&style_rule.css_text()),
    });

    (style, sel_map)
}

//todo: This test will only work when Span is available outside proceduaral macro crate.
//https://docs.rs/proc-macro2/latest/proc_macro2/struct.Span.html#method.unwrap
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use quote::quote;

//     #[test]
//     fn simple_tag() {
//         let input = quote!{
//             div {
//                 border: 1px solid black;
//                 margin: 25px 50px 75px 100px;
//                 background-color: lightblue;
//             }
//         };
//         let (style,_) = build_style(input.into(), &"sty".to_string());
//         assert_eq!(style,"div.sty {border: 1px solid black;margin: 25px 50px 75px 100px;background-color: lightblue;}");
//     }
// }

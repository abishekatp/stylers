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

pub(crate) fn build_style(ts: TokenStream, random_class: &str) -> (String, HashMap<String, ()>) {
    let mut style = String::new();
    let (style_sheet, sel_map) = CSSStyleSheet::new(ts, random_class);
    style_sheet.css_rules.iter().for_each(|rule| match rule {
        CSSRule::AtRule(at_rule) => style.push_str(&at_rule.css_text()),
        CSSRule::StyleRule(style_rule) => style.push_str(&style_rule.css_text()),
    });

    (style, sel_map)
}

#[cfg(test)]
mod tests {
    use super::*;
    use quote::quote;

    // TODO: Span is only available outside procedural macro crate. workaround?
    // https://docs.rs/proc-macro2/latest/proc_macro2/struct.Span.html#method.unwrap
    #[test]
    #[ignore]
    fn simple_tag() {
        let input = quote! {
            div {
                border: 1px solid black;
                margin: 25px 50px 75px 100px;
                background-color: lightblue;
            }
        };

        let (style, _) = build_style(input, "sty");
        assert_eq!(style,"div.sty {border: 1px solid black;margin: 25px 50px 75px 100px;background-color: lightblue;}");
    }
}

#![feature(proc_macro_span)]
#![feature(extend_one)]
mod css_at_rule;
mod css_style_declar;
mod css_style_rule;
mod css_style_sheet;
mod utils;

use css_style_sheet::{CSSRule, CSSStyleSheet};
use proc_macro2::{TokenStream, TokenTree};
use std::collections::HashMap;
//this function will build the whole style. This will return style string, component name, map of unique keys of selectors.
pub fn build_style(
    ts: TokenStream,
    random_class: &String,
) -> (String, String, HashMap<String, ()>) {
    let mut ts_iter = ts.into_iter();

    //first two tokens are for component name and comma.
    let TokenTree::Literal(comp_name) = ts_iter.next().expect("Expected value of type token tree") else {
        panic!(r#"Expected component name at the start like style!("component_name", your css comes here)"#)
    };
    let comp_name = comp_name.to_string().trim_matches('"').to_string();

    let TokenTree::Punct(comma) = ts_iter.next().expect("Expected value of type token tree") else {
        panic!("Expected comma(,) after component name");   
    };
    if comma.as_char() != ',' {
        panic!("Expected comma(,) after component name")
    }

    let mut style = String::new();
    let (style_sheet, sel_map) = CSSStyleSheet::new(ts_iter.collect(), random_class);
    style_sheet.css_rules.iter().for_each(|rule| match rule {
        CSSRule::AtRule(at_rule) => style.push_str(&at_rule.css_text()),
        CSSRule::StyleRule(style_rule) => style.push_str(&style_rule.css_text()),
    });

    (style, comp_name, sel_map)
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

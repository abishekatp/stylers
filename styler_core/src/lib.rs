#![feature(proc_macro_span)]
mod css_style_sheet;
mod css_rule;
mod css_style;
mod utils;

use proc_macro2::{TokenStream, TokenTree};
use std::collections::HashMap;
use css_style_sheet::CSSStyleSheet;
//this function will build the whole style. This will return style string, component name, map of unique keys.
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
    let (style_sheet,sel_map) = CSSStyleSheet::parse(ts_iter.collect(),random_class.clone());
    style_sheet.css_rules.iter().for_each(|rule|{
        style.push_str(&rule.css_text())
    });

    (style, comp_name,sel_map)
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
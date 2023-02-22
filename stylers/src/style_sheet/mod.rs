mod css_at_rule;
mod css_style_declar;
mod css_style_rule;
mod css_style_sheet;

use crate::style_sheet::css_style_sheet::{CSSRule, CSSStyleSheet};
use std::collections::HashMap;

/// This function will build the whole style text as rust TokenStream.
/// This function will take two arguments.
/// ts: TokenStream which is token stream of text content of whole style sheet.
/// random_class: &String is random class to be appended for each selector.
/// This function will return tuple with two fields (style string, component name, map of unique keys of selectors.)
/// style string: is the parsed style sheet as a string
/// component name: is the name of the component passed by
pub fn build_style(style_str: &str, random_class: &String) -> (String, HashMap<String, ()>) {
    let mut style = String::new();
    let (style_sheet, sel_map) = CSSStyleSheet::new(style_str, random_class);
    style_sheet.css_rules.iter().for_each(|rule| match rule {
        CSSRule::AtRule(at_rule) => style.push_str(&at_rule.css_text()),
        CSSRule::StyleRule(style_rule) => style.push_str(&style_rule.css_text()),
    });

    (style, sel_map)
}

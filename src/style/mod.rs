use proc_macro2::TokenTree;
use std::collections::HashSet;

mod css_at_rule;
mod css_style_declar;
mod css_style_rule;
mod css_style_sheet;
mod utils;

pub(crate) use crate::style::css_at_rule::AtRule;
pub(crate) use crate::style::css_style_declar::StyleDeclaration;
pub(crate) use crate::style::css_style_rule::StyleRule;
pub(crate) use crate::style::css_style_sheet::{Rule, StyleSheet};
use crate::Class;

pub(crate) fn build_style(
    token_stream: impl Iterator<Item = TokenTree>,
    class: &Class,
) -> (String, HashSet<String>) {
    let mut style = String::new();

    let (style_sheet, selectors) = StyleSheet::new(token_stream, class);

    style_sheet.rules.iter().for_each(|rule| match rule {
        Rule::AtRule(at_rule) => style.push_str(&at_rule.css_text()),
        Rule::StyleRule(style_rule) => style.push_str(&style_rule.css_text()),
    });

    (style, selectors)
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

        let class = Class::new("test".into());
        let (style, _) = build_style(input.into_iter(), &class);
        assert_eq!(style,"div.test {border: 1px solid black;margin: 25px 50px 75px 100px;background-color: lightblue;}");
    }
}

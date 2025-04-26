//! This create as of now only exposes one function named build_style.
//! The main focus of this function is to provide scoped css for Rust components(for the framework which provides component like architecture e.g leptos).
//! This function can be used parse the style sheet in rust.
mod css_at_rule;
mod css_style_declar;
mod css_style_rule;
mod css_style_sheet;
mod utils;
use proc_macro2::TokenTree;
use std::collections::HashSet;

use crate::Class;
pub(crate) use crate::style::css_at_rule::AtRule;
pub(crate) use crate::style::css_style_declar::StyleDeclaration;
pub(crate) use crate::style::css_style_rule::StyleRule;
pub(crate) use crate::style::css_style_sheet::{Rule, StyleSheet};

/// This function will build the whole style text as rust TokenStream.
/// This function will take two arguments.
/// ts: TokenStream which is token stream of text content of whole style sheet.
/// random_class: &String is random class to be appended for each selector.
/// This function will return tuple with two fields (style string, map of unique keys of selectors.)
/// style string: is the parsed style sheet as a string
pub fn build_style_from_ts(
  token_stream: impl Iterator<Item = TokenTree>,
  class: &Class,
  is_proc_macro: bool,
) -> (String, HashSet<String>) {
  let mut style = String::new();

  let (style_sheet, sel_map) = StyleSheet::new(token_stream, class, is_proc_macro);

  tracing::trace!(?style_sheet, ?sel_map);

  style_sheet.rules.iter().for_each(|rule| match rule {
    Rule::AtRule(at_rule) => style.push_str(&at_rule.css_text()),
    Rule::StyleRule(style_rule) => style.push_str(&style_rule.css_text()),
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

    let class = Class::new("test".into());
    let (style, _) = build_style_from_ts(input.into_iter(), &class, true);
    assert_eq!(
      style,
      "div.test {border: 1px solid black;margin: 25px 50px 75px 100px;background-color: lightblue;}"
    );
  }
}

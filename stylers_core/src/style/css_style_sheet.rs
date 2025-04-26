use proc_macro2::{Delimiter, TokenStream, TokenTree};
use std::collections::HashSet;

use crate::Class;
use crate::style::css_at_rule::AtRule;
use crate::style::css_style_rule::StyleRule;

// Rule is enum which will have two kinds style-rule and at-rule(which begins with @)
/// Ressource: <https://developer.mozilla.org/en-US/docs/Web/API/CSSRule>
#[derive(Debug)]
pub(crate) enum Rule {
  StyleRule(StyleRule),
  AtRule(AtRule),
}

/// Ressources: <https://developer.mozilla.org/en-US/docs/Web/API/StyleSheet> and <https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleSheet>
#[derive(Debug, Default)]
pub(crate) struct StyleSheet {
  pub(crate) rules: Vec<Rule>,
}

impl StyleSheet {
  pub(crate) fn new(
    token_stream: impl IntoIterator<Item = TokenTree>,
    class: &Class,
    is_proc_macro: bool,
  ) -> (StyleSheet, HashSet<String>) {
    let mut css_style_sheet = StyleSheet::default();

    let mut css_rule_tt = TokenStream::new();
    let mut sel_map = HashSet::new();

    let mut is_at_rule = false;
    let mut count = 0;

    for tt in token_stream {
      count += 1;

      css_rule_tt.extend(Some(tt.clone()));

      match tt {
        TokenTree::Group(t) => {
          if t.delimiter() == Delimiter::Brace {
            count = 0;
            if is_at_rule {
              let (at_rule, new_map) = AtRule::new(css_rule_tt, class, is_proc_macro);
              css_style_sheet.rules.push(Rule::AtRule(at_rule));
              sel_map.extend(new_map);
              is_at_rule = false;
            } else {
              let (style_rule, new_map) = StyleRule::new(css_rule_tt, class, is_proc_macro);
              css_style_sheet.rules.push(Rule::StyleRule(style_rule));
              sel_map.extend(new_map);
            }
            css_rule_tt = TokenStream::new();
          }
        }
        TokenTree::Punct(p) => {
          let ch = p.as_char();
          if ch == '@' && count == 1 {
            is_at_rule = true;
          }
          //in regular at-rule css rule ends with semicolon without any style declaration.
          if is_at_rule && ch == ';' {
            let (at_rule, new_map) = AtRule::new(css_rule_tt, class, is_proc_macro);
            css_style_sheet.rules.push(Rule::AtRule(at_rule));
            sel_map.extend(new_map);
            is_at_rule = false;
            css_rule_tt = TokenStream::new();
          }
        }
        _ => continue,
      }
    }

    (css_style_sheet, sel_map)
  }
}

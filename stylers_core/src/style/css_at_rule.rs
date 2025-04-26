use proc_macro2::{Delimiter, TokenStream, TokenTree};
use std::collections::HashSet;

use crate::Class;
use crate::style::css_style_sheet::{Rule, StyleSheet};
use crate::style::utils::{add_spaces, parse_group};

/// ref: https://developer.mozilla.org/en-US/docs/Web/CSS/At-rule
#[derive(Debug)]
pub(crate) struct AtRule {
  //nested at-rule may contain one or more css rule block inside it.
  pub(crate) rules: Vec<Rule>,
  pub(crate) at_rules: Vec<String>,
}

impl AtRule {
  // This method will parse the at-rule tokenstream and return teh AtRule
  // HashMap will contain all unique selectors which may be nested inside at-rule.
  pub(crate) fn new(
    token_stream: TokenStream,
    class: &Class,
    is_proc_macro: bool,
  ) -> (AtRule, HashSet<String>) {
    let mut css_at_rule = AtRule {
      rules: vec![],
      at_rules: vec![],
    };
    css_at_rule.parse(token_stream, class, is_proc_macro);

    (css_at_rule, HashSet::new())
  }

  // This css_text method will give the whole at-rule as single string value.
  // Note that we the calling function will be responsible for passing token stream of single at-rule at a time.
  pub(crate) fn css_text(&self) -> String {
    let mut text = String::new();
    //when we call parse method recursively it pushes at rule in order from inner most to outer most.
    self.at_rules.iter().rev().for_each(|r| {
      text.push_str(r);
      text.push('{');
    });
    //here we add the css_rules which are nested inside of at-rules one by one.
    if !self.rules.is_empty() {
      for css_rule in self.rules.iter() {
        match css_rule {
          Rule::StyleRule(style_rule) => text.push_str(&style_rule.css_text()),
          Rule::AtRule(at_rule) => text.push_str(&at_rule.css_text()),
        }
      }
      for _ in 0..self.at_rules.len() {
        text.push('}');
      }
    }
    //in case of regular at_rule remove all extra open braces added in the previous step.
    let text = text.trim_matches('{');
    text.to_string()
  }

  // This parse method will parse the at-rule tokn stream.
  // Note: this is recursive function it will handle nested at-rules.
  fn parse(
    &mut self,
    token_stream: TokenStream,
    class: &Class,
    is_proc_macro: bool,
  ) -> HashSet<String> {
    let mut at_rule = String::new();
    let mut selectors = HashSet::new();

    let mut pre_line = 0;
    let mut pre_col = 0;

    for tt in token_stream {
      match tt {
        TokenTree::Group(t) => {
          //only if the delimiter is brace it will be either style-rule or at-rule definition
          if t.delimiter() == Delimiter::Brace {
            let mut new_ts = t.stream().into_iter().take(1);
            let mut is_at_rule = false;
            if let Some(TokenTree::Punct(at)) = new_ts.next() {
              if at.as_char() == '@' {
                is_at_rule = true;
              }
            }

            if at_rule.contains("@page")
              || at_rule.contains("@font-face")
              || at_rule.contains("keyframes")
              || at_rule.contains("@counter-style")
              || at_rule.contains("@font-feature-values")
              || at_rule.contains("@property")
            {
              // At-rules will not contain any nested css-rules. so we just parse that group as a string.
              at_rule.push_str(&parse_group(t, is_proc_macro));
            } else if is_at_rule {
              // If there is another inner at-rule
              self.parse(t.stream(), class, is_proc_macro);
            } else {
              // Each at-rule may contain one or more css rules nested inside of it.
              let (mut style_sheet, new_map) = StyleSheet::new(t.stream(), class, is_proc_macro);
              self.rules.append(&mut style_sheet.rules);
              selectors = new_map;
            }
            self.at_rules.push(at_rule);
            at_rule = String::new();
          } else {
            add_spaces(
              &mut at_rule,
              t.span(),
              &mut pre_line,
              &mut pre_col,
              is_proc_macro,
            );
            at_rule.push_str(&parse_group(t, is_proc_macro));
          }
        }
        TokenTree::Ident(t) => {
          add_spaces(
            &mut at_rule,
            t.span(),
            &mut pre_line,
            &mut pre_col,
            is_proc_macro,
          );
          at_rule.push_str(&t.to_string());
        }
        TokenTree::Literal(t) => {
          add_spaces(
            &mut at_rule,
            t.span(),
            &mut pre_line,
            &mut pre_col,
            is_proc_macro,
          );
          at_rule.push_str(&t.to_string());
        }
        TokenTree::Punct(t) => {
          let ch = t.as_char();
          add_spaces(
            &mut at_rule,
            t.span(),
            &mut pre_line,
            &mut pre_col,
            is_proc_macro,
          );
          at_rule.push(ch);
          // Regular at-rule ends with semicolon. there won't be any style declaration for this.
          if ch == ';' {
            self.at_rules.push(at_rule.clone());
          }
        }
      }
    }

    selectors
  }
}

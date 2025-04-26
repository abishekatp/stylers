use proc_macro2::{Delimiter, TokenStream, TokenTree};
use std::collections::HashSet;

use crate::Class;
use crate::style::css_style_declar::StyleDeclaration;
use crate::style::utils::{add_spaces, parse_group};

/// StyleRule is one kind of Rule which contains a selector text and a style declaration.
/// Ressource: <https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleRule>
#[derive(Debug, Default)]
pub(crate) struct StyleRule {
  pub(crate) selector_text: String,
  pub(crate) style: StyleDeclaration,
}

impl StyleRule {
  // This function will take the token stream of one StyleRule and parse it.
  // Note that we the calling function will be responsible for passing token stream of single style-rule at a time.
  pub(crate) fn new(
    ts: TokenStream,
    class: &Class,
    is_proc_macro: bool,
  ) -> (StyleRule, HashSet<String>) {
    let mut css_style_rule = StyleRule::default();
    let sel_map = css_style_rule.parse(ts, class, is_proc_macro);

    (css_style_rule, sel_map)
  }

  // This css_text method will give the whole style-rule as single string value.
  pub(crate) fn css_text(&self) -> String {
    let mut text = self.selector_text.clone();
    text.push_str(&self.style.style_css_text());
    text
  }

  // parse method will extract the selector part of the style-rule and parse that selector using parse_selector method.
  fn parse(&mut self, ts: TokenStream, class: &Class, is_proc_macro: bool) -> HashSet<String> {
    let mut pre_col: usize = 0;
    let mut pre_line: usize = 0;
    //selector will just store current selector of the style rule.
    let mut selector = String::new();
    let mut sel_map = HashSet::new();

    for tt in ts {
      match tt {
        TokenTree::Group(t) => {
          //only if the delimiter is brace it will be style definition.
          if t.delimiter() == Delimiter::Brace {
            sel_map = self.parse_selector(&selector, class);
            self.style = StyleDeclaration::new(t, is_proc_macro);
          } else {
            add_spaces(
              &mut selector,
              t.span(),
              &mut pre_line,
              &mut pre_col,
              is_proc_macro,
            );
            selector.push_str(&parse_group(t, is_proc_macro));
          }
        }
        TokenTree::Ident(t) => {
          add_spaces(
            &mut selector,
            t.span(),
            &mut pre_line,
            &mut pre_col,
            is_proc_macro,
          );
          selector.push_str(&t.to_string());
        }
        TokenTree::Literal(t) => {
          add_spaces(
            &mut selector,
            t.span(),
            &mut pre_line,
            &mut pre_col,
            is_proc_macro,
          );
          selector.push_str(t.to_string().trim_matches('"'));
        }
        TokenTree::Punct(t) => {
          let ch = t.as_char();
          //only when ch is dot or hash we need space information. because space will mean direct child.
          //colon also need space info because it may be custom directive like :deep(p)
          if ch == '.' || ch == '#' || ch == ':' {
            add_spaces(
              &mut selector,
              t.span(),
              &mut pre_line,
              &mut pre_col,
              is_proc_macro,
            );
          } else {
            let end = t.span().end();
            pre_col = end.column;
            pre_line = end.line;
            if is_proc_macro {
              let end = t.span().unwrap().end();
              pre_col = end.column();
              pre_line = end.line();
            }
          }
          selector.push(t.as_char());
        }
      }
    }
    sel_map
  }

  //parse_selector method will parse the all parts of selector and add random class to them
  pub(crate) fn parse_selector(&mut self, selector_text: &str, class: &Class) -> HashSet<String> {
    let mut sel_map: HashSet<String> = HashSet::new();
    let mut source = String::new();
    let sel_len = selector_text.len();
    let mut is_punct_start = false;
    let mut is_pseudo_class_start = false;
    let mut is_bracket_open = false;
    let mut is_deep_directive = false;
    let mut is_deep_directive_open = false;
    let mut was_deep_directive_open = false;
    let mut temp = String::new();
    let mut i = 0;
    for c in selector_text.chars() {
      i += 1;
      //when reading external files in h2,h1{} selector will be splitted into multiple lines
      if c == '\n' {
        continue;
      }

      //ignore all white spaces after :deep directive.
      if was_deep_directive_open {
        if c.is_whitespace() {
          source.push(' ');
          continue;
        } else {
          source.push(c);
          was_deep_directive_open = false;
          continue;
        }
      }

      //ignore everything between square brackets.
      //todo:handle the case when brackets inside attribute.
      if is_bracket_open {
        if c == ']' {
          is_bracket_open = false;
          source.push(c);

          if !is_deep_directive_open {
            source.push_str(&class.as_selector());
          }

          temp.push(c);
          sel_map.insert(temp.clone());
          temp = String::new();
        } else {
          source.push(c);
          temp.push(c);
        }
        continue;
      }
      if c == '[' {
        is_bracket_open = true;
        source.push(c);
        temp.push(c);
        continue;
      }

      // if current char is colon and previous char is space means that is custom :deep directive.
      // then just use whatever values inside :deep() directive.
      if is_deep_directive_open && c != ')' {
        source.push(c);
        continue;
      }
      if is_deep_directive_open && c == ')' {
        is_deep_directive = false;
        is_deep_directive_open = false;
        was_deep_directive_open = true;
        continue;
      }
      if is_deep_directive && c == '(' {
        is_deep_directive_open = true;
        continue;
      }
      if is_deep_directive && c != '(' {
        continue;
      }
      if c == ':' {
        if let Some(sub) = selector_text.get(i..i + 4) {
          if sub == "deep" {
            is_deep_directive = true;
            continue;
          }
        }
      }

      //ignore everything until we reach to whitespace or end of the line after encountering pseudo class selector(:).
      if is_pseudo_class_start {
        if c == ' ' || i == sel_len {
          source.push(c);
          is_pseudo_class_start = false;

          if c != ' ' {
            temp.push(c);
          }
          sel_map.insert(temp.clone());
          temp = String::new();
        } else {
          source.push(c);
          temp.push(c);
        }
        continue;
      }
      if c == ':' {
        is_pseudo_class_start = true;
        source.push_str(&class.as_selector());
        source.push(c);
        temp.push(c);
        continue;
      }

      //this condition ignores the unwanted white space after comma, >, +, ~ punctuations.
      if is_punct_start {
        // this will remove newline charactors following comma(,) in selectors.
        if c.is_whitespace() {
          continue;
        } else {
          is_punct_start = false;
        }
      }
      if c == ',' || c == '+' || c == '~' || c == '>' || c == '|' {
        source.push_str(&class.as_selector());
        source.push(c);
        is_punct_start = true;

        sel_map.insert(temp.clone());
        temp = String::new();
        continue;
      }

      //check for universal selector.
      if c == '*' {
        source.push_str(&class.as_selector());
        sel_map.insert("*".to_string());
        continue;
      }

      //append random class if we reach end of the line.
      if i == sel_len {
        source.push(c);
        source.push_str(&class.as_selector());

        temp.push(c);
        sel_map.insert(temp.clone());
        temp = String::new();
        continue;
      }

      //check for direct child selector
      if c == ' ' {
        source.push_str(&class.as_selector());
        source.push(' ');

        sel_map.insert(temp.clone());
        temp = String::new();
      } else {
        source.push(c);
        temp.push(c);
      }
    }

    //if :root pseudo element is used then no need to add random class.
    if source.contains(":root") {
      source = String::from(":root");
    }
    self.selector_text = source;
    sel_map
  }
}

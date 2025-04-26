use crate::Class;
use crate::style::AtRule;
use crate::style::StyleRule;
use crate::style::{Rule, StyleSheet};

impl StyleSheet {
  // This function will take the whole stylesheet content as string and return CSSStyleSheet structure
  pub(crate) fn from_str(style_str: &str, class: &Class) -> StyleSheet {
    //removing all the comments in the css content.
    let mut style_str = style_str.to_string();
    while let Some((first, last)) = style_str.split_once("/*") {
      let mut temp = String::new();
      temp.push_str(first);
      let (_, end) = last
        .split_once("*/")
        .expect("Expecting to split the comment");
      temp.push_str(end);
      style_str = temp;
    }

    let mut css_style_sheet = StyleSheet { rules: vec![] };
    let mut is_at_rule = false;
    let mut style = String::new();
    let mut no_of_openings = 0;
    let mut no_of_closings = 0;
    for ch in style_str.chars() {
      //trimming the style because empty spaces at the beginning are not significant.
      if style.trim_start().is_empty() && ch == '@' {
        is_at_rule = true;
      }
      if ch == '{' {
        no_of_openings += 1;
      }
      if ch == '}' {
        no_of_closings += 1;
      }
      style.push(ch);

      // ending with semicolon means at rule without style declaration
      if ch == ';' && is_at_rule && no_of_openings == 0 {
        //to omit empty whitespaces.
        style = style.trim().to_string();
        let at_rule = AtRule::from_str(&style, class);
        css_style_sheet.rules.push(Rule::AtRule(at_rule));
        style = String::new();
        is_at_rule = false
      } else if ch == '}' && no_of_openings != 0 && no_of_openings == no_of_closings {
        //this else condition handle one block of at_rule or style rule from the whole style sheet content.
        //to omit empty whitespaces.
        style = style.trim().to_string();
        if is_at_rule {
          let at_rule = AtRule::from_str(&style, class);
          css_style_sheet.rules.push(Rule::AtRule(at_rule));
        } else {
          let style_rule = StyleRule::from_str(&style, class);
          css_style_sheet.rules.push(Rule::StyleRule(style_rule));
        }
        no_of_openings = 0;
        no_of_closings = 0;
        style = String::new();
        is_at_rule = false;
      }
    }

    css_style_sheet
  }
}

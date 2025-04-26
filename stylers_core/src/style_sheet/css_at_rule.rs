use crate::Class;
use crate::style::AtRule;

use crate::style::StyleSheet;

impl AtRule {
  // This method will parse the at-rule block string and return the AtRule
  // note: this is string version of the parse method in AtRule struct.
  pub(crate) fn from_str(at_block: &str, class: &Class) -> AtRule {
    let mut css_at_rule = AtRule {
      rules: vec![],
      at_rules: vec![],
    };
    css_at_rule.parse_from_str(at_block, class);

    css_at_rule
  }

  // This parse method will parse the at-rule block string.
  // note: this is string version of the parse method in AtRule struct.
  fn parse_from_str(&mut self, at_block: &str, class: &Class) {
    if at_block.trim().ends_with(';') {
      self.at_rules.push(parse_at_rule_declaration(at_block));
    } else {
      let mut at_block = at_block;
      loop {
        let (at_rule, declaration) = at_block.split_once('{').expect("Expecting At rule");
        //removing extra white spaces and extra closing braces at the end.
        let mut declaration = declaration.trim();
        let (first, _) = declaration
          .rsplit_once('}')
          .expect("Expecting to remove extra closing braces");
        declaration = first;

        //for some cases keyframes comes with prefix @-webkit-keyframes
        if at_rule.contains("@page")
          || at_rule.contains("@font-face")
          || at_rule.contains("keyframes")
          || at_rule.contains("@counter-style")
          || at_rule.contains("@font-feature-values")
          || at_rule.contains("@property")
        {
          let mut at_rule = at_rule.to_string();
          at_rule.push('{');
          at_rule.push_str(&parse_at_rule_declaration(declaration));
          at_rule.push('}');
          self.at_rules.push(at_rule.to_string());
          break;
        } else if declaration.starts_with('@') {
          self.at_rules.push(at_rule.to_string());
          at_block = declaration;
          continue;
        } else {
          self.at_rules.push(at_rule.to_string());
          let style_sheet = StyleSheet::from_str(declaration, class);
          self.rules = style_sheet.rules;
          break;
        }
      }
    }
    self.at_rules.reverse();
  }
}

//Some at rules don't contain another style rule inside them for those rule we can directly parse the string
fn parse_at_rule_declaration(at_rule_declar: &str) -> String {
  let mut parts: Vec<&str> = at_rule_declar.split('\n').collect();
  parts = parts.iter().map(|item| item.trim()).collect();
  parts.join("")
}

mod css_at_rule;
mod css_style_declar;
mod css_style_rule;
mod css_style_sheet;

use crate::{
    style::{Rule, StyleSheet},
    Class,
};

/// This function will build the whole style text as the String.
/// This build_style is string version of the build_style method from style macro.
pub(crate) fn build_style(style_str: &str, class: &Class) -> String {
    let mut style = String::new();
    let style_sheet = StyleSheet::from_str(style_str, class);
    style_sheet.rules.iter().for_each(|rule| match rule {
        Rule::AtRule(at_rule) => style.push_str(&at_rule.css_text()),
        Rule::StyleRule(style_rule) => style.push_str(&style_rule.css_text()),
    });

    style
}

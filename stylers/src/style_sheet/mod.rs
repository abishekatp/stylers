mod css_at_rule;
mod css_style_declar;
mod css_style_rule;
mod css_style_sheet;

use crate::style::{CSSRule, CSSStyleSheet};

/// This function will build the whole style text as the String.
/// This build_style is string version of the build_style method from style macro.
pub(crate) fn build_style(style_str: &str, random_class: &String) -> String {
    let mut style = String::new();
    let style_sheet = CSSStyleSheet::from_str(style_str, random_class);
    style_sheet.css_rules.iter().for_each(|rule| match rule {
        CSSRule::AtRule(at_rule) => style.push_str(&at_rule.css_text()),
        CSSRule::StyleRule(style_rule) => style.push_str(&style_rule.css_text()),
    });

    style
}

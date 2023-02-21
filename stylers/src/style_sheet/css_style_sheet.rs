use std::collections::HashMap;

use crate::style_sheet::css_at_rule::CSSAtRule;
use crate::style_sheet::css_style_rule::CSSStyleRule;

//ref: https://developer.mozilla.org/en-US/docs/Web/API/CSSRule
// CSSRule is enum which will have two kinds style-rule and at-rule(which begins with @)
#[derive(Debug)]
pub enum CSSRule {
    StyleRule(CSSStyleRule),
    AtRule(CSSAtRule),
}

//ref: https://developer.mozilla.org/en-US/docs/Web/API/StyleSheet
//ref: https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleSheet
//CSSStyleSheet is representation of single style sheet.
// It will contain list of CSSRule.
pub struct CSSStyleSheet {
    pub css_rules: Vec<CSSRule>,
}

impl CSSStyleSheet {
    //This function will take the whole stylesheet content as token stream and return CSSStyleSheet structure
    pub fn new(style_str: String, random_class: &str) -> (CSSStyleSheet, HashMap<String, ()>) {
        let mut css_style_sheet = CSSStyleSheet { css_rules: vec![] };
        let mut is_at_rule = false;
        let sel_map = HashMap::new();
        let mut style = String::new();
        let mut no_of_openings = 0;
        let mut no_of_closings = 0;
        for ch in style_str.chars() {
            //trimming the style because empty spaces at the beginning are not significant.
            if style.trim_start().len() <= 0 && ch == '@' {
                is_at_rule = true;
            }
            if ch == '{' {
                no_of_openings += 1;
            }
            if ch == '}' {
                no_of_closings += 1;
            }
            style.push(ch);
            if ch == ';' && is_at_rule && no_of_openings == 0 {
                //to omit empty whitespaces.
                style = style.trim().to_string();
                let at_rule = CSSAtRule::new(style, random_class);
                css_style_sheet.css_rules.push(CSSRule::AtRule(at_rule));
                style = String::new();
                is_at_rule = false
            } else if ch == '}' && no_of_openings != 0 && no_of_openings == no_of_closings {
                //to omit empty whitespaces.
                style = style.trim().to_string();
                if is_at_rule {
                    let at_rule = CSSAtRule::new(style, random_class);
                    css_style_sheet.css_rules.push(CSSRule::AtRule(at_rule));
                } else {
                    let at_rule = CSSStyleRule::new(style, random_class);
                    css_style_sheet.css_rules.push(CSSRule::StyleRule(at_rule));
                }
                no_of_openings = 0;
                no_of_closings = 0;
                style = String::new();
                is_at_rule = false;
            }
        }

        (css_style_sheet, sel_map)
    }
}

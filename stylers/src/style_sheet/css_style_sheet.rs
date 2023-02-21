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
        let mut count = 0;
        let mut sel_map = HashMap::new();
        let mut style_rule = String::new();
        let mut no_of_openings = 0;
        let mut no_of_closings = 0;
        for ch in style_str.chars() {
            if style_rule.len() <= 0 && ch == '@' {
                is_at_rule = true;
            }
            style_rule.push(ch);
            if is_at_rule && no_of_openings == 0 && ch == ';' {
                //pass the rule to CSSAtRule
                style_rule = String::new();
            } else if no_of_openings != 0 && no_of_openings == no_of_closings {
                if is_at_rule {
                    //pass to the CSSAtRule
                } else {
                    //pass to the CSSStyleRule
                }
                no_of_openings = 0;
                no_of_closings = 0;
                style_rule = String::new();
            }
        }

        (css_style_sheet, sel_map)
    }
}

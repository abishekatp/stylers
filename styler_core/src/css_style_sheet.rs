use proc_macro2::{Delimiter, TokenStream, TokenTree};
use std::collections::HashMap;

use crate::css_at_rule::CSSAtRule;
use crate::css_style_rule::{CSSRule, CSSStyleRule};

//ref: https://developer.mozilla.org/en-US/docs/Web/API/StyleSheet
//ref: https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleSheet
pub struct CSSStyleSheet {
    pub css_rules: Vec<Box<dyn CSSRule>>,
}

impl CSSStyleSheet {
    pub fn new(ts: TokenStream, random_class: &str) -> (CSSStyleSheet, HashMap<String, ()>) {
        let mut css_style_sheet = CSSStyleSheet { css_rules: vec![] };
        let mut ts_iter = ts.into_iter();
        let mut css_rule_tt = TokenStream::new();
        let mut is_at_rule = false;
        let mut count = 0;
        let mut sel_map = HashMap::new();
        loop {
            if let Some(tt) = ts_iter.next() {
                count += 1;
                css_rule_tt.extend_one(tt.clone());
                match tt {
                    TokenTree::Group(t) => {
                        if t.delimiter() == Delimiter::Brace {
                            count = 0;
                            if is_at_rule {
                                let (at_rule, new_map) = CSSAtRule::new(css_rule_tt, random_class);
                                css_style_sheet.css_rules.push(Box::new(at_rule));
                                sel_map.extend(new_map.into_iter());
                                is_at_rule = false;
                            } else {
                                let (style_rule, new_map) =
                                    CSSStyleRule::new(css_rule_tt, random_class);
                                css_style_sheet.css_rules.push(Box::new(style_rule));
                                sel_map.extend(new_map.into_iter());
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
                            let (at_rule, new_map) = CSSAtRule::new(css_rule_tt, random_class);
                            css_style_sheet.css_rules.push(Box::new(at_rule));
                            sel_map.extend(new_map.into_iter());
                            is_at_rule = false;
                            css_rule_tt = TokenStream::new();
                        }
                    }
                    _ => continue,
                }
            } else {
                break;
            }
        }

        (css_style_sheet, sel_map)
    }
}

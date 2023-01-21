use std::collections::HashMap;

use crate::css_style_rule::CSSStyleRule;
use crate::css_style_sheet::{CSSRule, CSSStyleSheet};
use crate::utils::{add_spaces, parse_group};
use proc_macro2::{Delimiter, TokenStream, TokenTree};

//ref: https://developer.mozilla.org/en-US/docs/Web/CSS/At-rule
#[derive(Debug)]
pub struct CSSAtRule {
    //only nested at-rules will contain style_rule.
    css_rules: Vec<CSSRule>,
    //todo: parse the style argument in parse and create this style_map.
    // style_map: HashMap<String,String>,
    at_rules: Vec<String>,
}

impl CSSAtRule {
    pub fn new(ts: TokenStream, random_class: &str) -> (CSSAtRule, HashMap<String, ()>) {
        let mut css_at_rule = CSSAtRule {
            css_rules: vec![],
            at_rules: vec![],
        };
        css_at_rule.parse(ts, random_class);

        (css_at_rule, HashMap::new())
    }

    pub fn css_text(&self) -> String {
        let mut text = String::new();
        //when we call parse method recursively it pushes at rule in order from inner most to outer most.
        self.at_rules.iter().rev().for_each(|r| {
            text.push_str(r);
            text.push('{');
        });
        if self.css_rules.len() > 0 {
            for css_rule in self.css_rules.iter() {
                match css_rule {
                    CSSRule::StyleRule(style_rule) => text.push_str(&style_rule.css_text()),
                    CSSRule::AtRule(at_rule) => text.push_str(&at_rule.css_text()),
                }
            }
            for _ in 0..self.at_rules.len() {
                text.push('}');
            }
        }
        //in case of regular at_rule remove all open braces added.
        let text = text.trim_matches('{');
        text.to_string()
    }

    fn parse(&mut self, ts: TokenStream, random_class: &str) -> HashMap<String, ()> {
        let mut at_rule = String::new();
        let mut pre_line = 0;
        let mut pre_col = 0;
        let mut ts_iter = ts.into_iter();
        let mut sel_map = HashMap::new();

        loop {
            match ts_iter.next() {
                Some(tt) => {
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
                                if is_at_rule {
                                    //if there is another inner at-rule
                                    self.parse(t.stream(), random_class);
                                } else {
                                    if at_rule.contains(&"@support".to_string()) {
                                        //@support rule can contain multiple rules inside it.
                                        let (mut style_sheet, new_map) =
                                            CSSStyleSheet::new(t.stream(), random_class);
                                        self.css_rules.append(&mut style_sheet.css_rules);
                                        sel_map = new_map;
                                    } else {
                                        let (style_rule, new_map) =
                                            CSSStyleRule::new(t.stream(), random_class);
                                        self.css_rules.push(CSSRule::StyleRule(style_rule));
                                        sel_map = new_map;
                                    }
                                }
                                self.at_rules.push(at_rule);
                                at_rule = String::new();
                            } else {
                                add_spaces(&mut at_rule, t.span(), &mut pre_line, &mut pre_col);
                                at_rule.push_str(&parse_group(t));
                            }
                        }
                        TokenTree::Ident(t) => {
                            add_spaces(&mut at_rule, t.span(), &mut pre_line, &mut pre_col);
                            at_rule.push_str(&t.to_string());
                        }
                        TokenTree::Literal(t) => {
                            add_spaces(&mut at_rule, t.span(), &mut pre_line, &mut pre_col);
                            at_rule.push_str(&t.to_string());
                        }
                        TokenTree::Punct(t) => {
                            let ch = t.as_char();
                            add_spaces(&mut at_rule, t.span(), &mut pre_line, &mut pre_col);
                            at_rule.push(ch);
                            //regular at rule ends with semicolon.
                            if ch == ';' {
                                self.at_rules.push(at_rule.clone());
                            }
                        }
                    }
                }
                None => break,
            }
        }

        sel_map
    }
}

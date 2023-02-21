use proc_macro2::{Delimiter, TokenStream, TokenTree};
use std::collections::HashMap;

use crate::style::css_style_sheet::{CSSRule, CSSStyleSheet};
use crate::style::utils::{add_spaces, parse_group};

// ref: https://developer.mozilla.org/en-US/docs/Web/CSS/At-rule
// CSSAtRule is one kind of CSSRule. It will have two parts
// at-rule may contain nested at-rules. style-rule will be the inner most nesting of nested at-rule
// some ar-rules like @support may contain multiple style-rules nested inside.
// So we store them in the css_rules list.
#[derive(Debug)]
pub struct CSSAtRule {
    //nested at-rule may contain one or more css rule block inside it.
    css_rules: Vec<CSSRule>,
    at_rules: Vec<String>,
}

impl CSSAtRule {
    // This method will parse the at-rule tokenstream and return teh CSSAtRule
    // HashMap will contain all unique selectors which may be nested inside at-rule.
    pub fn new(ts: TokenStream, random_class: &str) -> (CSSAtRule, HashMap<String, ()>) {
        let mut css_at_rule = CSSAtRule {
            css_rules: vec![],
            at_rules: vec![],
        };
        css_at_rule.parse(ts, random_class);

        (css_at_rule, HashMap::new())
    }

    // This css_text method will give the whole at-rule as single string value.
    // Note that we the calling function will be responsible for passing token stream of single at-rule at a time.
    pub fn css_text(&self) -> String {
        let mut text = String::new();
        //when we call parse method recursively it pushes at rule in order from inner most to outer most.
        self.at_rules.iter().rev().for_each(|r| {
            text.push_str(r);
            text.push('{');
        });
        //here we add the css_rules which are nested inside of at-rules one by one.
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
        //in case of regular at_rule remove all extra open braces added in the previous step.
        let text = text.trim_matches('{');
        text.to_string()
    }

    // This parse method will parse the at-rule tokn stream.
    // Note: this is recursive function it will handle nested at-rules.
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
                                //@font-feature-values at-rule does not need inner at-rules to be parsed
                                if is_at_rule && !at_rule.contains("@font-feature-values") {
                                    //if there is another inner at-rule
                                    self.parse(t.stream(), random_class);
                                } else {
                                    if at_rule.contains("@page")
                                        || at_rule.contains("@font-face")
                                        || at_rule.contains("keyframes")
                                        || at_rule.contains("@counter-style")
                                        || at_rule.contains("@font-feature-values")
                                        || at_rule.contains("@property")
                                    {
                                        //these at-rules will not contain any nested css-rules. so we just parse that group as a string.
                                        at_rule.push_str(&parse_group(t));
                                    } else {
                                        //each at-rule may contain one or more css rules nested inside of it.
                                        //it is like another small style sheet inside of it. So we use CSSStyleSheet here.
                                        let (mut style_sheet, new_map) =
                                            CSSStyleSheet::new(t.stream(), random_class);
                                        self.css_rules.append(&mut style_sheet.css_rules);
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
                            //regular at rule ends with semicolon. there won't be any style declaration for this.
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

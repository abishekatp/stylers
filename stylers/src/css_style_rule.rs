use proc_macro2::{Delimiter, TokenStream, TokenTree};
use std::collections::HashMap;

use crate::css_style_declar::CSSStyleDeclaration;
use crate::utils::{add_spaces, parse_group};

// ref: https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleRule
// CSSStyleRule is one kind of CSSRule which will contain two parts.
// One is selector text and another one is style declaration for that selector.
#[derive(Debug)]
pub struct CSSStyleRule {
    selector_text: String,
    style: CSSStyleDeclaration,
}

impl CSSStyleRule {
    // This function will take the token stream of one CSSStyleRule and parse it.
    // Note that we the calling function will be responsible for passing token stream of single style-rule at a time.
    pub fn new(ts: TokenStream, random_class: &str) -> (CSSStyleRule, HashMap<String, ()>) {
        let mut css_style_rule = CSSStyleRule {
            selector_text: String::new(),
            style: CSSStyleDeclaration::empty(),
        };
        let sel_map = css_style_rule.parse(ts, random_class);

        (css_style_rule, sel_map)
    }

    // This css_text method will give the whole style-rule as single string value.
    pub fn css_text(&self) -> String {
        let mut text = self.selector_text.clone();
        text.push_str(&self.style.style_css_text());
        text
    }

    // parse method will extract the selector part of the style-rule and parse that selector using parse_selector method.
    fn parse(&mut self, ts: TokenStream, random_class: &str) -> HashMap<String, ()> {
        let mut pre_col: usize = 0;
        let mut pre_line: usize = 0;
        //selector will just store current selector of the style rule.
        let mut selector = String::new();
        let mut ts_iter = ts.into_iter();
        let mut sel_map = HashMap::new();
        loop {
            match ts_iter.next() {
                Some(tt) => {
                    match tt {
                        TokenTree::Group(t) => {
                            //only if the delimiter is brace it will be style definition.
                            if t.delimiter() == Delimiter::Brace {
                                sel_map = self.parse_selector(&selector, random_class);
                                self.style = CSSStyleDeclaration::new(t);
                            } else {
                                add_spaces(&mut selector, t.span(), &mut pre_line, &mut pre_col);
                                selector.push_str(&parse_group(t));
                            }
                        }
                        TokenTree::Ident(t) => {
                            add_spaces(&mut selector, t.span(), &mut pre_line, &mut pre_col);
                            selector.push_str(&t.to_string());
                        }
                        TokenTree::Literal(t) => {
                            add_spaces(&mut selector, t.span(), &mut pre_line, &mut pre_col);
                            selector.push_str(t.to_string().trim_matches('"'));
                        }
                        TokenTree::Punct(t) => {
                            let ch = t.as_char();
                            //only when ch is dot or hash we need space information. because space will mean direct child.
                            if ch == '.' || ch == '#' {
                                add_spaces(&mut selector, t.span(), &mut pre_line, &mut pre_col);
                            } else {
                                let end = t.span().unwrap().end();
                                pre_col = end.column;
                                pre_line = end.line;
                            }
                            selector.push(t.as_char());
                        }
                    }
                }
                None => break,
            }
        }
        sel_map
    }

    //parse_selector method will parse the all parts of selector and add random class to them
    fn parse_selector(&mut self, selector_text: &str, random_class: &str) -> HashMap<String, ()> {
        let mut sel_map: HashMap<String, ()> = HashMap::new();
        let mut source = String::new();
        let sel_len = selector_text.len();
        let mut is_punct_start = false;
        let mut is_pseudo_class_start = false;
        let mut is_bracket_open = false;
        let mut temp = String::new();
        let mut i = 0;
        for c in selector_text.chars() {
            i += 1;

            //ignore everything between square brackets.
            //todo:handle the case when brackets inside attribute.
            if is_bracket_open {
                if c == ']' {
                    is_bracket_open = false;
                    source.push(c);
                    source.push_str(random_class);

                    temp.push(c);
                    sel_map.insert(temp.clone(), ());
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

            //ignore everything until we reach to whitespace or end of the line after encountering pseudo class selector(:).
            if is_pseudo_class_start {
                if c == ' ' || i == sel_len {
                    source.push(c);
                    is_pseudo_class_start = false;

                    if c != ' ' {
                        temp.push(c);
                    }
                    sel_map.insert(temp.clone(), ());
                    temp = String::new();
                } else {
                    source.push(c);
                    temp.push(c);
                }
                continue;
            }
            if c == ':' {
                is_pseudo_class_start = true;
                source.push_str(random_class);
                source.push(c);
                temp.push(c);
                continue;
            }

            //this condition ignores the unwanted white space after comma, >, +, ~ punctuations.
            if is_punct_start && c == ' ' {
                is_punct_start = false;
                continue;
            }
            if c == ',' || c == '+' || c == '~' || c == '>' || c == '|' {
                source.push_str(random_class);
                source.push(c);
                is_punct_start = true;

                sel_map.insert(temp.clone(), ());
                temp = String::new();
                continue;
            }

            //check for universal selector.
            if c == '*' {
                source.push_str(random_class);
                sel_map.insert("*".to_string(), ());
                continue;
            }

            //append random class if we reach end of the line.
            if i == sel_len {
                source.push(c);
                source.push_str(random_class);

                temp.push(c);
                sel_map.insert(temp.clone(), ());
                temp = String::new();
                continue;
            }

            //check for direct child selector
            if c == ' ' {
                source.push_str(random_class);
                source.push(' ');

                sel_map.insert(temp.clone(), ());
                temp = String::new();
            } else {
                source.push(c);
                temp.push(c);
            }
        }
        self.selector_text = source;
        sel_map
    }
}

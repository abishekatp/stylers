use std::collections::HashMap;
use proc_macro2::{TokenStream, TokenTree, Delimiter};

use crate::css_rule::{CSSRule,CSSStyleRule};
use crate::utils::{add_spaces,parse_group};

//ref: https://developer.mozilla.org/en-US/docs/Web/API/StyleSheet
//ref: https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleSheet
pub struct CSSStyleSheet{
    pub css_rules: Vec<Box<dyn CSSRule>>
}

impl CSSStyleSheet {
    pub fn parse(ts:TokenStream,random_class: String)->(CSSStyleSheet,HashMap<String, ()>){
        let mut css_style_sheet = CSSStyleSheet{css_rules: vec![]};
        let mut pre_col: usize = 0;
        let mut pre_line: usize = 0;
        //selector will just store current selector for each style
        let mut selector = String::new();
        let mut sel_map: HashMap<String, ()> = HashMap::new();

        ts.into_iter().for_each(|tt| {
            match tt {
                TokenTree::Group(t) => {
                    //only if the delimiter is brace it will be style definition
                    if t.delimiter() == Delimiter::Brace {
                        let style_rule = CSSStyleRule::parse(&selector,t,&random_class,&mut sel_map);
                        css_style_sheet.css_rules.push(Box::new(style_rule));  
                         
                        selector = String::new();      
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
                    //only in these two cases we need space information
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
        });

        (css_style_sheet,sel_map)
    }
}
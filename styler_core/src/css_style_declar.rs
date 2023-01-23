use crate::utils::{add_spaces, parse_group};
use proc_macro2::{Group, TokenTree};

//ref: https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleDeclaration
//CSSStyleDeclaration is actual style declaration for each selectors
#[derive(Debug)]
pub struct CSSStyleDeclaration {
    //e.g {color:red;}
    style_css_text: String,
}

impl CSSStyleDeclaration {
    pub fn empty() -> CSSStyleDeclaration {
        CSSStyleDeclaration {
            style_css_text: "".to_string(),
        }
    }
    pub fn new(group: Group) -> CSSStyleDeclaration {
        let mut css_style_declar = CSSStyleDeclaration {
            style_css_text: "".to_string(),
        };
        css_style_declar.parse(group);
        css_style_declar
    }
    pub fn style_css_text(&self) -> String {
        self.style_css_text.clone()
    }

    pub fn parse(&mut self, group: Group) {
        let mut body = String::new();
        let mut pre_col: usize = 0;
        let mut pre_line: usize = 0;
        let mut is_property_start = false;
        body.push('{');
        group.stream().into_iter().for_each(|tt| match tt {
            TokenTree::Group(t) => {
                add_spaces(&mut body, t.span(), &mut pre_line, &mut pre_col);
                body.push_str(&parse_group(t));
            }
            TokenTree::Ident(t) => {
                add_spaces(&mut body, t.span(), &mut pre_line, &mut pre_col);
                body.push_str(&t.to_string());
            }
            TokenTree::Literal(t) => {
                add_spaces(&mut body, t.span(), &mut pre_line, &mut pre_col);
                //we are trimming r because in some cases like "\1g34" is not valid rust syntax.
                //in those places user have to use r"\1g34".
                body.push_str(t.to_string().trim_start_matches('r'));
            }
            TokenTree::Punct(t) => {
                let ch = t.as_char();
                //this will check if user adde the semicolon or not.
                //since we are validating using colon actual line which is missing semicolon will be pre_line-1.
                if is_property_start && ch == ':' {
                    panic!("Missing simicolon in line {}", pre_line - 1)
                } else if ch == ':' {
                    is_property_start = true;
                }
                if ch == ';' {
                    is_property_start = false;
                }
                add_spaces(&mut body, t.span(), &mut pre_line, &mut pre_col);
                body.push(ch);
            }
        });
        body.push('}');
        self.style_css_text.push_str(&body);
    }
}

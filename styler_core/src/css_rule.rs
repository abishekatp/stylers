use crate::css_style::CSSStyleDeclaration;
use proc_macro2::Group;
use std::collections::HashMap;

//ref: https://developer.mozilla.org/en-US/docs/Web/API/CSSRule
pub trait CSSRule {
    //e.g div{color:red;}
    fn css_text(&self) -> String;
}

//ref: https://developer.mozilla.org/en-US/docs/Web/API/CSSStyleRule
#[derive(Debug)]
pub struct CSSStyleRule {
    selector_text: String,
    style: CSSStyleDeclaration,
    //todo: parse the style argument in parse and create this style_map.
    // style_map: HashMap<String,String>,
}

impl CSSRule for CSSStyleRule {
    fn css_text(&self) -> String {
        let mut text = self.selector_text.clone();
        text.push_str(&self.style.style_css_text());
        text
    }
}

impl CSSStyleRule {
    pub fn parse(
        selector_text: &str,
        group: Group,
        random_class: &str,
        sel_map: &mut HashMap<String, ()>,
    ) -> CSSStyleRule {
        let mut source = String::new();
        let sel_len = selector_text.len();
        let mut is_punct_start = false;
        let mut is_pseudo_class_start = false;
        let mut is_bracket_open = false;
        let mut is_event_start = false;
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

            //ignore everything until we reach to whitespace after encountering event selector(@).
            if is_event_start {
                if c == ' ' {
                    is_event_start = false;
                    source.push(' ');
                } else {
                    source.push(c);
                }
                continue;
            }
            if c == '@' {
                is_event_start = true;
                source.push(c);
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
            if c == ',' || c == '+' || c == '~' || c == '>' {
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

        CSSStyleRule {
            selector_text: source,
            style: CSSStyleDeclaration::parse(group),
        }
    }
}

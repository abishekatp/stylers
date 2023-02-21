use std::collections::HashMap;

use crate::style_sheet::css_style_declar::CSSStyleDeclaration;

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
    pub fn new(style_block: String, random_class: &str) -> CSSStyleRule {
        let mut css_style_rule = CSSStyleRule {
            selector_text: String::new(),
            style: CSSStyleDeclaration::empty(),
        };
        css_style_rule.parse(style_block, random_class);

        css_style_rule
    }

    // This css_text method will give the whole style-rule as single string value.
    pub fn css_text(&self) -> String {
        let mut text = self.selector_text.clone();
        text.push_str(&self.style.style_css_text());
        text
    }

    // parse method will extract the selector part of the style-rule and parse that selector using parse_selector method.
    fn parse(&mut self, style_block: String, random_class: &str) {
        //selector will just store current selector of the style rule.
        let (selector_text, body) = style_block.split_once('{').expect("Expecting selector");
        let selector_text = selector_text.trim();
        let _ = self.parse_selector(selector_text, random_class);
        let mut style_declar = String::from("{");
        style_declar.push_str(&body);
        self.style = CSSStyleDeclaration::new(style_declar);
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
            //when reading external files in h2,h1{} selector will be splitted into multiple lines
            if c == '\n' {
                continue;
            }
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

        //if :root pseudo element is used then no need to add random class.
        if source.contains(":root") {
            source = String::from(":root");
        }
        self.selector_text = source;
        sel_map
    }
}

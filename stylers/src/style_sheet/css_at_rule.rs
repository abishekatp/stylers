use crate::style_sheet::css_style_sheet::CSSRule;

use super::css_style_sheet::CSSStyleSheet;

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
    // This method will parse the at-rule block and return the CSSAtRule
    pub fn new(at_block: String, random_class: &str) -> CSSAtRule {
        let mut css_at_rule = CSSAtRule {
            css_rules: vec![],
            at_rules: vec![],
        };
        css_at_rule.parse(at_block, random_class);

        css_at_rule
    }

    // This css_text method will give the whole at-rule as single string value.
    // Note that we the calling function will be responsible for passing token stream of single at-rule at a time.
    pub fn css_text(&self) -> String {
        let mut text = String::new();
        //when we call parse method recursively it pushes at rule in order from inner most to outer most.
        self.at_rules.iter().for_each(|r| {
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

    // This parse method will parse the at-rule block.
    // Note: this is recursive function it will handle nested at-rules.
    fn parse(&mut self, at_block: String, random_class: &str) {
        if at_block.trim().ends_with(';') {
            self.at_rules.push(at_block);
        } else {
            let mut at_block = at_block;
            loop {
                let (at_rule, declaration) = at_block.split_once('{').expect("Expecting At rule");
                self.at_rules.push(at_rule.to_string());
                let mut declaration = declaration.trim();
                if declaration.starts_with('@') {
                    at_block = declaration.to_string();
                    continue;
                } else {
                    for _ in 0..self.at_rules.len() {
                        let (first, _) = declaration
                            .rsplit_once('}')
                            .expect("Expecting to remove extra closing braces");
                        declaration = first;
                    }
                    let (style_sheet, _) =
                        CSSStyleSheet::new(declaration.to_string(), random_class);
                    self.css_rules = style_sheet.css_rules;
                    break;
                }
            }
        }
    }
}

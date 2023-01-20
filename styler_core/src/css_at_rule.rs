use proc_macro2::Group;

use crate::css_style_declar::CSSStyleDeclaration;
use crate::css_style_rule::CSSRule;

//ref: https://developer.mozilla.org/en-US/docs/Web/CSS/At-rule
#[derive(Debug)]
pub struct CSSAtRule {
    selector_text: String,
    //some times there will be no style declaration for at rules.
    style: Option<CSSStyleDeclaration>,
    //todo: parse the style argument in parse and create this style_map.
    // style_map: HashMap<String,String>,
}

impl CSSRule for CSSAtRule {
    fn css_text(&self) -> String {
        let mut text = self.selector_text.clone();
        if let Some(st) = &self.style{
            text.push_str(&st.style_css_text());
        }
        text
    }
}

impl CSSAtRule {
    //todo: check how to add random classes to these at rules.
    pub fn parse_regular(selector_text: &str,random_class: &str) -> Box<dyn CSSRule> {
        let selector_text = selector_text.trim().trim_end_matches(';');
        let mut sel = parse(selector_text, random_class);
        sel.push(';');
        Box::new(CSSAtRule {
            selector_text: sel,
            style: None,
        })
    }
    pub fn parse_nested(selector_text: &str,random_class: &str,group: Group) -> Box<dyn CSSRule> {
        Box::new(CSSAtRule {
            selector_text: parse(selector_text, random_class),
            style: Some(CSSStyleDeclaration::parse(group)),
        })
    }
}

fn parse(selector_text: &str,random_class: &str)->String{
    let (at_rule, identifiers) = selector_text.split_once(' ').expect(&format!("Error in {}",selector_text));
    let mut idents = String::from(at_rule);
    idents.push(' ');
    identifiers.split(',').for_each(|ident|{
        idents.push_str(ident.trim());
        idents.push_str(random_class);
        idents.push(',');
    });
    idents.trim_end_matches(',').to_string()
}

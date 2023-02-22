use crate::style::CSSStyleDeclaration;
use crate::style::CSSStyleRule;

impl CSSStyleRule {
    // This function will take the style block string and parse it.
    // this function will only parse single stryle rule block
    pub(crate) fn from_str(style_block: &str, random_class: &str) -> CSSStyleRule {
        let mut css_style_rule = CSSStyleRule {
            selector_text: String::new(),
            style: CSSStyleDeclaration::empty(),
        };
        css_style_rule.parse_str(style_block, random_class);

        css_style_rule
    }

    // parse method will extract the selector part of the style-rule and parse that selector using parse_selector method.
    fn parse_str(&mut self, style_block: &str, random_class: &str) {
        //selector will just store current selector of the style rule.
        let (selector_text, body) = style_block.split_once('{').expect("Expecting selector");
        let selector_text = selector_text.trim();
        let _ = self.parse_selector(selector_text, random_class);
        let mut style_declar = String::from("{");
        style_declar.push_str(&body);
        self.style = CSSStyleDeclaration::from_str(style_declar);
    }
}

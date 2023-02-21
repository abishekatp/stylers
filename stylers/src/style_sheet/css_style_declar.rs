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
    pub fn new(style_declar: String) -> CSSStyleDeclaration {
        let mut css_style_declar = CSSStyleDeclaration {
            style_css_text: "".to_string(),
        };
        css_style_declar.parse(style_declar);
        css_style_declar
    }
    pub fn style_css_text(&self) -> String {
        self.style_css_text.clone()
    }

    pub fn parse(&mut self, group: String) {
        let mut body = String::new();

        let mut property = String::new();
        let mut is_property_start = false;
        //if raw_str then we should not remove the double quotes
        let mut raw_str = false;

        let mut pre_col: usize = 0;
        let mut pre_line: usize = 0;
    }
}

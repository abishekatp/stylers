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

    pub fn parse(&mut self, style_delar: String) {
        //todo: what if newline is inside content property
        let mut declarations: Vec<&str> = style_delar.split('\n').collect();
        declarations = declarations.iter().map(|item| item.trim()).collect();
        self.style_css_text = declarations.join("");
    }
}

use crate::style::StyleDeclaration;

impl StyleDeclaration {
  // note: this is string version of the new method in StyleDeclaration struct.
  pub(crate) fn from_str(style_declar: String) -> StyleDeclaration {
    let mut css_style_declar = StyleDeclaration {
      style_css_text: "".to_string(),
    };
    css_style_declar.parse_from_str(style_declar);
    css_style_declar
  }

  // note: this is string version of the parse method in StyleDeclaration struct.
  fn parse_from_str(&mut self, style_delar: String) {
    //todo: what if newline is inside content property
    let mut declarations: Vec<&str> = style_delar.split('\n').collect();
    declarations = declarations.iter().map(|item| item.trim()).collect();
    self.style_css_text = declarations.join("");
  }
}

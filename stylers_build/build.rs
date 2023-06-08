use glob::glob;
use std::{env::current_dir, fs};
use syn::parse::{Parse, ParseStream, Result};
use syn::Item;
use syn::{Ident, LitStr, Token};

// Getting debug output from a build.rs is not easy. Using a warning as a work-around,
// but that only allows single line output.
// https://github.com/rust-lang/cargo/issues/985#issuecomment-1071667472
// For full output use:
// less ./target/debug/build/*/output
macro_rules! p {
    ($($tokens: tt)*) => {
        println!("cargo:warning={}", format!($($tokens)*))
    }
}

fn main() {
    p!("Running build script...");
    p!("Working dir: {:?}", current_dir().unwrap());

    let pattern = format!("{}/src/**/*.rs", current_dir().unwrap().to_str().unwrap());
    for file in glob(&pattern).unwrap() {
        let file = file.unwrap();
        let content = fs::read_to_string(file).expect("Failed to read file");
        let ast = syn::parse_file(&content).unwrap();

        for item in ast.items {
            if let Item::Macro(item_macro) = item {
                let path = &item_macro.mac.path;
                p!("path: {:?}", path.segments);
            }
        }
    }
}

#[derive(Debug)]
struct StyleMacroCall {
    style_var_name: String,
    style_content: String,
}

impl Parse for StyleMacroCall {
    fn parse(input: ParseStream) -> Result<Self> {
        let css_var_name: Ident = input.parse()?;
        input.parse::<Token![,]>()?;
        let css_content: LitStr = input.parse()?;

        Ok(StyleMacroCall {
            style_var_name: css_var_name.to_string(),
            style_content: css_content.value(),
        })
    }
}

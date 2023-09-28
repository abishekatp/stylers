mod parser;

use glob::glob;
use parser::parse_item;
use std::fs::File;
use std::io::{self, Write};
use std::{env::current_dir, fs};

pub use stylers_macro::style;
pub use stylers_macro::style_sheet;
pub use stylers_macro::style_sheet_str;
pub use stylers_macro::style_str;

macro_rules! p {
    ($($tokens: tt)*) => {
        println!("cargo:warning={}", format!($($tokens)*))
    }
}

pub fn build(output_path: Option<String>) {
    let pattern = format!("{}/src/**/*.rs", current_dir().unwrap().to_str().unwrap());
    let mut output_css = String::from("");
    p!(
        "{}",
        "===============================Stylers debug output start==============================="
    );
    for file in glob(&pattern).unwrap() {
        let file = file.unwrap();
        let content = fs::read_to_string(file).expect("Failed to read file");
        let ast = syn::parse_file(&content).unwrap();
        // check the each item in the *.rs file
        for item in ast.items {
            let Some(css) = parse_item(item) else {
                continue;
            };
            output_css += &css;
        }
    }

    write_css(output_path, &output_css)
        .unwrap_or_else(|e| p!("Problem creating output file: {}", e.to_string()));
    p!(
        "{}",
        "===============================Stylers debug output end==============================="
    );
}

const OUTPUT_DIR: &str = "./target";
/// Writes the styles in its own file and appends itself to the main.css file
fn write_css(output_path: Option<String>, content: &str) -> io::Result<()> {
    let mut out_path = String::from("./target/stylers_out.css");
    if let Some(path) = output_path {
        out_path = path;
    }

    fs::create_dir_all(&OUTPUT_DIR)?;

    let mut buffer = File::create(out_path)?;
    buffer.write_all(content.as_bytes())?;
    buffer.flush()?;

    Ok(())
}

use glob::glob;
use std::fs::File;
use std::io::Write;
use std::{borrow::Borrow, env::current_dir, fs};
use stylers_core::rand_class_from_seed;
use stylers_core::{from_str, from_ts};
use syn::{Expr, Item, Stmt};

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
            // check if the item is of type Function.
            if let Item::Fn(fn_def) = item {
                let _componet_name = &fn_def.sig.ident;
                // check each statement in the function
                for stmt in fn_def.block.stmts {
                    // check if any of the statment is of the form `let any_valid_variable = style!{}`
                    if let Stmt::Local(let_bin) = stmt {
                        if let Some(init) = let_bin.init {
                            if let Expr::Macro(expr_mac) = init.expr.borrow() {
                                if let Some(path_seg) = expr_mac.mac.path.segments.last() {
                                    let macro_name = path_seg.ident.clone().to_string();
                                    // p!("macro_name:{:?}", macro_name);

                                    if macro_name == String::from("style") {
                                        let ts = expr_mac.mac.tokens.clone();
                                        let class_name = rand_class_from_seed(ts.to_string());
                                        let (scoped_css, _) = from_ts(ts, &class_name, false);
                                        output_css += &scoped_css;
                                    }

                                    if macro_name == String::from("style_sheet") {
                                        let ts = expr_mac.mac.tokens.clone();
                                        let file_path = ts.to_string();
                                        let file_path = file_path.trim_matches('"');
                                        let css_content = std::fs::read_to_string(&file_path)
                                            .expect("Expected to read file");

                                        let random_class =
                                            rand_class_from_seed(css_content.to_string());
                                        let style = from_str(&css_content, &random_class);
                                        output_css += &style;
                                    }
                                }
                            }
                        }
                    }
                    //todo: other than let statements cover that other way style! macro can instantiated.
                }
            }
        }
    }

    let mut out_path = String::from("./target/stylers_out.css");
    if let Some(path) = output_path {
        out_path = path;
    }
    let mut file = File::create(out_path).expect("Problem creating main.css file");
    file.write_all(output_css.as_bytes())
        .expect("Error writing to the file");

    p!(
        "{}",
        "===============================Stylers debug output end==============================="
    );
}

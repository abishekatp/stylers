use glob::glob;
use std::{borrow::Borrow, env::current_dir, fs};
use stylers::style_str;
use syn::{Expr, Item, Stmt};

// use std::collections::hash_map::RandomState;
// use std::hash::{BuildHasher, Hasher};

macro_rules! p {
    ($($tokens: tt)*) => {
        println!("cargo:warning={}", format!($($tokens)*))
    }
}

pub fn build() {
    let pattern = format!("{}/src/**/*.rs", current_dir().unwrap().to_str().unwrap());
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
            // check if the item is of type Function. E.g fn Hello(cx: Scope, name: &'static str) -> impl IntoView {}
            if let Item::Fn(fn_def) = item {
                let componet_name = &fn_def.sig.ident;
                p!("componet_name:{:?}", componet_name);
                // check each statement in the function
                for stmt in fn_def.block.stmts {
                    // check if any of the statment is of the form let any_valid_variable = style!{}
                    if let Stmt::Local(let_bin) = stmt {
                        if let Some(init) = let_bin.init {
                            if let Expr::Macro(expr_mac) = init.expr.borrow() {
                                if let Some(path_seg) = expr_mac.mac.path.segments.last() {
                                    let macro_name = path_seg.ident.clone().to_string();
                                    // checking if the macro is the style macro
                                    if macro_name == String::from("style_build") {
                                        let _ts = expr_mac.mac.tokens.clone();
                                        // todo: how to pass this token stream at compile time for this any_macro!.
                                        // todo: One way we have to pass actual rust token stream.
                                        // todo: The other way we can pass the string that keeps the space information to the macro
                                        // let (scoped_css, _class_name) = any_macro!(ts);
                                        // p!("class_name:{}", class_name);
                                        p!("macro_name:{}", macro_name);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    p!(
        "{}",
        "===============================Stylers debug output end==============================="
    );
}

// fn rand_class() -> String {
//     let hash = RandomState::new().build_hasher().finish().to_string();
//     let k = &hash[0..6];
//     format!(".l-{}", k.to_string())
// }

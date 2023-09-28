use proc_macro2::TokenStream;
use rstml::node::{Node, NodeAttribute};
use std::borrow::Borrow;
use stylers_core::Class;
use stylers_core::{from_str, from_ts};
use syn::{Expr, Item, Macro, Stmt};

macro_rules! p {
    ($($tokens: tt)*) => {
        println!("cargo:warning={}", format!($($tokens)*))
    }
}

pub fn parse_item(item: Item) -> Option<String> {
    let mut output_css = String::from("");

    match item {
        // check if the item is of type Function.
        Item::Fn(fn_def) => {
            let _componet_name = &fn_def.sig.ident;
            // check each statement in the function
            for stmt in fn_def.block.stmts {
                // check if any of the statment is of the form `let any_valid_variable = style!{}`
                let Some(css) = parse_stmt(stmt) else {
                    continue;
                };
                output_css += &css;
            }
        }
        _ => (),
    }

    Some(output_css)
}

fn parse_stmt(stmt: Stmt) -> Option<String> {
    let mut output_css = String::from("");
    match stmt {
        Stmt::Local(let_bin) => {
            let Some(init) = let_bin.init else {
                return None;
            };
            let Some(css) = parse_expr(init.expr.borrow()) else {
                return None;
            };
            output_css += &css;
        }
        Stmt::Macro(expr_mac) => {
            let _ = parse_macro(&expr_mac.mac);
            return None;
        }
        // Stmt::Expr(expr, _) => {
        //     let _ = parse_expr(&expr);
        //     return None;
        // }
        _ => (),
    }

    Some(output_css)
}

fn parse_expr(expr: &Expr) -> Option<String> {
    let mut output_css = String::from("");

    match expr {
        Expr::Macro(expr_mac) => {
            let Some(css) = &parse_macro(&expr_mac.mac) else {
                return None;
            };
            output_css += &css
        }
        _ => (),
    }

    Some(output_css)
}

fn parse_macro(mac: &Macro) -> Option<String> {
    let mut output_css = String::from("");
    let Some(path_seg) = mac.path.segments.last() else {
        return None;
    };
    let macro_name = path_seg.ident.clone().to_string();
    // p!("macro_name:{:?}", macro_name);

    if macro_name == String::from("style") {
        let ts = mac.tokens.clone();
        let class = Class::rand_class_from_seed(ts.to_string());
        let token_stream = TokenStream::from(ts).into_iter();
        let (scoped_css, _) = from_ts(token_stream, &class, false);
        output_css += &scoped_css;
    }

    if macro_name == String::from("style_sheet") {
        let ts = mac.tokens.clone();
        let file_path = ts.to_string();
        let file_path = file_path.trim_matches('"');
        let css_content = std::fs::read_to_string(&file_path).expect("Expected to read file");

        let class = Class::rand_class_from_seed(css_content.to_string());
        let style = from_str(&css_content, &class);
        output_css += &style;
    }

    if macro_name == String::from("view") {
        // let ts = mac.tokens.clone();
        // let Ok(nodes) = rstml::parse2(ts) else {
        //     p!("{}", "error parsing the view! macro");
        //     return None;
        // };
        // for node in nodes {
        //     let Some(css) = parse_node(node) else {
        //         continue;
        //     };
        //     output_css += &css;
        // }
    }

    Some(output_css)
}

fn parse_node(node: Node) -> Option<String> {
    let mut output_css = String::from("");

    match node {
        Node::Element(element) => {
            for attr in element.attributes() {
                let Some(css) = parse_attr(attr) else {
                    continue;
                };
                output_css += &css;
            }
            for node in element.children {
                let Some(css) = parse_node(node) else {
                    continue;
                };
                output_css += &css;
            }
        }
        _ => (),
    }

    Some(output_css)
}

fn parse_attr(attr: &NodeAttribute) -> Option<String> {
    let mut output_css = String::from("");

    match attr {
        NodeAttribute::Attribute(keyed_attr) => {
            if keyed_attr.key.to_string() == String::from("class") {
                let Some(expr) = keyed_attr.value() else {
                    return None;
                };
                let Some(css) = parse_expr(expr) else {
                    return None;
                };
                output_css += &css;
            }
        }
        _ => (),
    }

    Some(output_css)
}

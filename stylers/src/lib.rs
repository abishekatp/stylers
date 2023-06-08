//! This crate provides style macro for scoped css in rust web frameworks which follows component like architecture e.g Leptos.
use proc_macro::TokenStream;
use proc_macro2::{self, TokenTree};
use quote::quote;

use std::collections::hash_map::RandomState;
use std::fs::{self, File, OpenOptions};
use std::hash::{BuildHasher, Hasher};
use std::io::Write;
use stylers_core::{build_style_from_str, build_style_from_ts};

/// style macro take any valid css as input and returns a unique class name.
/// The first two Tokens of the token stream must be component name and comma punctuation.
/// This function will create css file named same as component name in the css folder of the root directory of the project.
/// For examples see: <https://github.com/abishekatp/stylers>
#[proc_macro]
pub fn style(ts: TokenStream) -> TokenStream {
    let (comp_name, ts) = get_component_name(ts);
    let random_class = rand_class();
    let (style, _sel_map) = build_style_from_ts(ts, &random_class);
    let random_class = random_class[1..].to_string();
    let expanded = quote! {
        #random_class
    };
    write_to_file(&style, &comp_name);
    TokenStream::from(expanded)
}

/// style_sheet macro take css file path as a string input and returns a unique class name.
/// This function will create css file in the css folder of the root directoy of the project.
/// CSS file will be named same as css file name passed as the input.
/// For examples see: <https://github.com/abishekatp/stylers>
#[proc_macro]
pub fn style_sheet(ts: TokenStream) -> TokenStream {
    let file_path = ts.to_string();
    let file_path = file_path.trim_matches('"');
    let css_content = std::fs::read_to_string(&file_path).expect("Expected to read file");
    let random_class = rand_class();
    let style = build_style_from_str(&css_content, &random_class);
    let random_class = random_class[1..].to_string();
    let expanded = quote! {
        #random_class
    };
    let file_name = file_path
        .split("/")
        .last()
        .expect("Expecting file name from file path");
    let file_name = file_name.split(".css").next().expect("Expecting file name");
    write_to_file(&style, file_name);
    TokenStream::from(expanded)
}

/// style_str macro any valid css as input and returns a tuple (unique_class_name,style_string).
/// note: this macro does not require a component name like style macro
/// For examples see: <https://github.com/abishekatp/stylers>
#[proc_macro]
pub fn style_str(ts: TokenStream) -> TokenStream {
    let random_class = rand_class();
    let (style, _sel_map) = build_style_from_ts(ts.into(), &random_class);
    let random_class = random_class[1..].to_string();
    let expanded = quote! {
        (#random_class,#style)
    };
    TokenStream::from(expanded)
}

/// style_sheet_str macro take css file path as a string input and returns a tuple (unique_class_name,style_string).
/// For examples see: <https://github.com/abishekatp/stylers>
#[proc_macro]
pub fn style_sheet_str(ts: TokenStream) -> TokenStream {
    let file_path = ts.to_string();
    let file_path = file_path.trim_matches('"');
    let css_content = std::fs::read_to_string(&file_path).expect("Expected to read file");
    let random_class = rand_class();
    let style = build_style_from_str(&css_content, &random_class);
    let random_class = random_class[1..].to_string();
    let expanded = quote! {
        (#random_class,#style)
    };
    TokenStream::from(expanded)
}

///This style_sheet_test macro will return the style string. Note:created for testing purpose only.
#[proc_macro]
pub fn style_sheet_test(ts: TokenStream) -> TokenStream {
    let file_path = ts.to_string();
    let file_path = file_path.trim_matches('"');
    let css_content = std::fs::read_to_string(&file_path).expect("Expected to read file");
    let random_class = String::from(".test");
    let style = build_style_from_str(&css_content, &random_class);
    let expanded = quote! {
        #style
    };
    TokenStream::from(expanded)
}

///This style_test macro will return the style string. Note:created for testing purpose only.
#[proc_macro]
pub fn style_test(ts: TokenStream) -> TokenStream {
    let (_comp_name, ts) = get_component_name(ts);
    let random_class = String::from(".test");
    let (style, _sel_map) = build_style_from_ts(ts.into(), &random_class);
    let expanded = quote! {
        #style
    };
    TokenStream::from(expanded)
}

fn rand_class() -> String {
    let hash = RandomState::new().build_hasher().finish().to_string();
    let k = &hash[0..6];
    format!(".l-{}", k.to_string())
}

//append if file exists or write it into the new file
fn write_to_file(data: &str, file_name: &str) {
    let dir_path = String::from("./target/stylers/css");
    let mut file_path = String::from("./target/stylers/css/");
    file_path.push_str(&file_name.to_lowercase());
    file_path.push_str(".css");

    fs::create_dir_all(&dir_path)
        .expect("Problem creating css directory in the root directory of the project.");
    let mut buffer = File::create(file_path).expect("Problem creating css file");
    let _ = buffer.write_all(data.as_bytes());
    buffer.flush().expect("Problem closing css file");

    cat(&dir_path)
}

fn cat(dir: &str) {
    let out_path = "./target/stylers/main.css";
    let _ = File::create(out_path).expect("Problem creating main.css file");
    let mut buffer = OpenOptions::new()
        .append(true)
        .open(out_path)
        .expect("Problem opening main.css file");

    let files = fs::read_dir(dir).expect("Problem reading css directory");
    for file in files {
        let data = fs::read_to_string(
            file.expect("Problem getting css file path inside css dir")
                .path(),
        )
        .expect("Problem reading css file in css dir");
        let _ = buffer.write(data.as_bytes());
    }
    buffer.flush().expect("Problem closing main.css file");
}

//first two tokens are for component name and comma. we extract those info in this function
fn get_component_name(ts: TokenStream) -> (String, proc_macro2::TokenStream) {
    let mut ts_iter = proc_macro2::TokenStream::from(ts).into_iter();
    let TokenTree::Literal(comp_name) = ts_iter.next().expect("Expected value of type token tree") else {
        panic!(r#"Expected component name at the start like style!("component_name", your css comes here)"#)
    };
    let comp_name = comp_name.to_string().trim_matches('"').to_string();

    let TokenTree::Punct(comma) = ts_iter.next().expect("Expected value of type token tree") else {
        panic!("Expected comma(,) after component name");   
    };
    if comma.as_char() != ',' {
        panic!("Expected comma(,) after component name")
    }
    (comp_name, ts_iter.collect())
}

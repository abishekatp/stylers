#![feature(proc_macro_span)]
use proc_macro::TokenStream;
use proc_macro2;
use quote::quote;

use std::collections::hash_map::RandomState;
use std::hash::{BuildHasher, Hasher};

use std::fs::{self, File, OpenOptions};
use std::io::Write;
use styler_core::build_style;

#[proc_macro]
pub fn style(ts: TokenStream) -> TokenStream {
    let random_class = rand_class();
    let (style, comp_name, _sel_map) =
        build_style(proc_macro2::TokenStream::from(ts), &random_class);
    // dbg!(&sel_map);
    let random_class = random_class[1..].to_string();
    let expanded = quote! {
        const __STYLER_CLASS_NAME:&str = #random_class;
    };
    // dbg!(&style);
    write_to_file(&style, &comp_name);
    TokenStream::from(expanded)
    // let call_site = proc_macro::Span::call_site();
    // dbg!(&call_site);
    // println!("{}",call_site.source_text().unwrap());
    // println!("{:?}",call_site.source_file());
}

//this macro will return the style string. Note:created for testing purpose only.
#[proc_macro]
pub fn style_str(ts: TokenStream) -> TokenStream {
    let random_class = String::from(".test");
    let (style, _comp_name, _sel_map) =
        build_style(proc_macro2::TokenStream::from(ts), &random_class);
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
    let dir_path = String::from("./css");
    let mut file_path = String::from("./css/");
    file_path.push_str(&file_name.to_lowercase());
    file_path.push_str(".css");

    fs::create_dir_all(&dir_path).unwrap();
    let mut buffer = File::create(file_path).expect("Problem creating css file");
    let _ = buffer.write_all(data.as_bytes());
    buffer.flush().expect("Problem closing css file");

    cat(&dir_path)
}

fn cat(dir: &str) {
    let out_path = "./main.css";
    let _ = File::create(out_path).expect("Problem creating css file");
    let mut buffer = OpenOptions::new()
        .append(true)
        .open(out_path)
        .expect("Problem opening css file");

    let files = fs::read_dir(dir).unwrap();
    for file in files {
        let data = fs::read_to_string(file.unwrap().path()).unwrap();
        let _ = buffer.write(data.as_bytes());
    }
    buffer.flush().expect("Problem closing css file");
}

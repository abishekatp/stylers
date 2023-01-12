use proc_macro::{Delimiter, Group, TokenStream, TokenTree};
use std::{collections::HashMap, vec};

//todo: try to convert this to proc_macro2 types to use outside proc_macro crate
//this function will build the whole style and write it into the main.css file
pub fn build_style(ts: TokenStream, random_class: &String)-> (String, HashMap<String, ()>){
    println!("{:#?}",ts);
    let mut pre_col: usize = 0;
    let mut pre_line: usize = 0;
    let mut style = String::new();
    //selector will just store current selector for each style
    let mut selector = String::new();
    let mut sel_map :HashMap<String, ()>= HashMap::new();

    ts.into_iter().for_each(|tt| {
        match tt {
            TokenTree::Group(t) => {
                //only if the delimiter is brace it will be style definition
                if t.delimiter()==Delimiter::Brace{
                    append_selector(&mut style, &selector, &random_class,&mut sel_map);
                    selector = String::new();
                    add_spaces(&mut style,t.span(), &mut pre_line, &mut pre_col);
                    style.push_str(&parse_body(t));
                    //todo:remove this.
                    style.push('\n');
                }else{
                    add_spaces(&mut selector,t.span(), &mut pre_line, &mut pre_col);
                    selector.push_str(&parse_body(t));
                }
            }
            TokenTree::Ident(t) => {
                add_spaces(&mut selector,t.span(), &mut pre_line, &mut pre_col);
                selector.push_str(&t.to_string());
            }
            TokenTree::Literal(t) => {
                add_spaces(&mut selector,t.span(), &mut pre_line, &mut pre_col);
                selector.push_str(t.to_string().trim_matches('"'));
            }
            TokenTree::Punct(t) => {
                add_spaces(&mut selector,t.span(), &mut pre_line, &mut pre_col);
                selector.push(t.as_char());
            }
        }
    });
    // dbg!(&style);
    // _write_to_file(style);
    (style, sel_map)
}

//parse each css selector body. this function recursively calls itself
fn parse_body(group: Group) -> String {
    let mut body = String::new();
    let mut pre_col: usize = 0;
    let mut pre_line: usize = 0;
    let mut closing = ' ';
    match group.delimiter() {
        Delimiter::Brace => {
            body.push('{');
            closing = '}';
        }
        Delimiter::Parenthesis => {
            body.push('(');
            closing = ')';
        }
        Delimiter::Bracket => {
            body.push('[');
            closing = ']';
        }
        _ => (),
    }
    group.stream().into_iter().for_each(|tt| match tt {
        TokenTree::Group(t) => {
            add_spaces(&mut body,t.span(), &mut pre_line, &mut pre_col);
            body.push_str(&parse_body(t));
        }
        TokenTree::Ident(t) => {
            add_spaces(&mut body,t.span(), &mut pre_line, &mut pre_col);
            body.push_str(&t.to_string());
        }
        TokenTree::Literal(t) => {
            add_spaces(&mut body,t.span(), &mut pre_line, &mut pre_col);
            body.push_str(&t.to_string());
        }
        TokenTree::Punct(t) => {
            add_spaces(&mut body,t.span(), &mut pre_line, &mut pre_col);
            body.push(t.as_char());
        }
    });
    body.push(closing);
    body
}

//check if spaces needed to be appended
fn add_spaces(source: &mut String,span: proc_macro::Span, pre_line: &mut usize, pre_col: &mut usize){
    let start = span.start();
    let end = span.end();
    let cur_col = start.column;
    let cur_line = start.line;
    if *pre_line == cur_line && cur_col > *pre_col {
        source.push(' ');
    }
    *pre_col = end.column;
    *pre_line = end.line;
}

//build selector
fn append_selector(source:&mut String,selector: &str,random_class:&str,sel_map:&mut HashMap<String,()>){
    let selectors:Vec<&str> = selector.split(' ').collect();
    selectors.into_iter().for_each(|t|{
        sel_map.insert(t.to_string(), ());

        source.push(' ');
        let is_pseudo_class=t.contains(':');
        let contains_comma=t.contains(',');
        let mut sels = vec![t];
        if contains_comma{
            sels = t.split(',').collect();
        }
        let sels_len = sels.len();
        let mut i = 0;
        //this code will handle commas and pseudo classes in the css selectors
        for s in sels{
            if is_pseudo_class{
                let (pre,suf) = s.split_once(':').expect(&format!("Pseudo class error at {}",selector));
                source.push_str(pre);
                source.push_str(random_class);
                source.push(':');
                source.push_str(suf);
            }else{
                source.push_str(s);
                source.push_str(random_class);
            }
            if sels_len>1 && i!=sels_len-1{
                source.push(',')
            }
            i+=1;
        }
    });
}
#![feature(proc_macro_span)]
use proc_macro2::{Delimiter, Group, TokenStream, TokenTree};
use std::collections::HashMap;

//this function will build the whole style and write it into the main.css file
pub fn build_style(ts: TokenStream, random_class: &String) -> (String, HashMap<String, ()>) {
    // println!("{:#?}",ts);
    let mut pre_col: usize = 0;
    let mut pre_line: usize = 0;
    let mut style = String::new();
    //selector will just store current selector for each style
    let mut selector = String::new();
    let mut sel_map: HashMap<String, ()> = HashMap::new();

    ts.into_iter().for_each(|tt| {
        match tt {
            TokenTree::Group(t) => {
                //only if the delimiter is brace it will be style definition
                if t.delimiter() == Delimiter::Brace {
                    append_selector(&mut style, &selector, &random_class, &mut sel_map);
                    selector = String::new();
                    add_spaces(&mut style, t.span(), &mut pre_line, &mut pre_col);
                    style.push_str(&parse_body(t));
                    //todo:remove this.
                    style.push('\n');
                } else {
                    add_spaces(&mut selector, t.span(), &mut pre_line, &mut pre_col);
                    selector.push_str(&parse_body(t));
                }
            }
            TokenTree::Ident(t) => {
                add_spaces(&mut selector, t.span(), &mut pre_line, &mut pre_col);
                selector.push_str(&t.to_string());
            }
            TokenTree::Literal(t) => {
                add_spaces(&mut selector, t.span(), &mut pre_line, &mut pre_col);
                selector.push_str(t.to_string().trim_matches('"'));
            }
            TokenTree::Punct(t) => {
                add_spaces(&mut selector, t.span(), &mut pre_line, &mut pre_col);
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
            add_spaces(&mut body, t.span(), &mut pre_line, &mut pre_col);
            body.push_str(&parse_body(t));
        }
        TokenTree::Ident(t) => {
            add_spaces(&mut body, t.span(), &mut pre_line, &mut pre_col);
            body.push_str(&t.to_string());
        }
        TokenTree::Literal(t) => {
            add_spaces(&mut body, t.span(), &mut pre_line, &mut pre_col);
            body.push_str(&t.to_string());
        }
        TokenTree::Punct(t) => {
            add_spaces(&mut body, t.span(), &mut pre_line, &mut pre_col);
            body.push(t.as_char());
        }
    });
    body.push(closing);
    body
}

//check if spaces needed to be appended
fn add_spaces(
    source: &mut String,
    span: proc_macro2::Span,
    pre_line: &mut usize,
    pre_col: &mut usize,
) {
    let start = span.unwrap().start();
    let end = span.unwrap().end();
    let cur_col = start.column;
    let cur_line = start.line;
    if *pre_line == cur_line && cur_col > *pre_col {
        source.push(' ');
    }
    *pre_col = end.column;
    *pre_line = end.line;
}

//build selector
fn append_selector(
    source: &mut String,
    selector: &str,
    random_class: &str,
    sel_map: &mut HashMap<String, ()>,
) {
    dbg!(&selector);

    //to handle commad separated selectors
    let separators: Vec<&str> = selector.split_inclusive(&[',', '>', '+', '~']).collect();
    let separators_len = separators.len();
    let mut i = 0;
    for s1 in separators {
        let mut s1 = s1.trim();
        let mut separator = "";
        //handles two cases first when there are no separators,
        //second when last selector does not have any separator.
        if separators_len > 1 && i != separators_len - 1 {
            let len = s1.len();
            separator = &s1[len - 1..];
            s1 = &s1[..len - 1];
        }
        // to handle indirect child selector
        let indirect_childs: Vec<&str> = s1.trim().split(' ').collect();
        let indirect_len = indirect_childs.len();
        let mut j = 0;
        for s3 in indirect_childs {
            let s3 = s3.trim();
            sel_map.insert(s3.to_string(), ());
            let is_pseudo_class = s3.contains(':');
            if s3 == "*" {
                //to handle universal selector
                source.push_str(random_class);
            } else if matches!(s3, "@keyframes" | "@-webkit-keyframes") {
                source.push_str(s3);
            } else if is_pseudo_class {
                //to handle pseudo classes
                let (pre, suf) = s3
                    .split_once(':')
                    .expect(&format!("Pseudo class error at {}", selector));
                source.push_str(pre);
                source.push_str(random_class);
                source.push(':');
                source.push_str(suf);
            } else {
                //general case
                source.push_str(s3);
                source.push_str(random_class);
            }
            if indirect_len - 1 != j {
                source.push(' ');
            }
            j += 1;
        } //indirect childs
        source.push_str(separator);
        i += 1;
    } //separators
}

//todo: This test will only work when Span is available outside proceduaral macro crate.
//https://docs.rs/proc-macro2/latest/proc_macro2/struct.Span.html#method.unwrap
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use quote::quote;

//     #[test]
//     fn simple_tag() {
//         let input = quote!{
//             div {
//                 border: 1px solid black;
//                 margin: 25px 50px 75px 100px;
//                 background-color: lightblue;
//             }
//         };
//         let (style,_) = build_style(input.into(), &"sty".to_string());
//         assert_eq!(style,"div.sty {border: 1px solid black;margin: 25px 50px 75px 100px;background-color: lightblue;}");
//     }
// }

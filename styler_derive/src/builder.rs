use proc_macro::{Delimiter, Group, TokenStream, TokenTree};
use std::fs::{File, OpenOptions};
use std::io::{ErrorKind, Write};

//todo: try to convert this to proc_macro2 types to use outside proc_macro crate
//this function will build the whole style and write it into the main.css file
pub fn build_style(ts: TokenStream, random_class: &String) {
    let mut pre_col: usize = 0;
    let mut pre_line: usize = 0;
    let mut style = String::new();
    //each componenet will have one unique random class

    ts.into_iter().for_each(|tt| {
        match tt {
            TokenTree::Group(t) => {
                style.push_str(&parse_line(t.span(), &mut pre_line, &mut pre_col));
                style.push_str(&parse_body(t));
                style.push('\n');
            }
            TokenTree::Ident(t) => {
                style.push_str(&parse_line(t.span(), &mut pre_line, &mut pre_col));
                style.push_str(&t.to_string());
                //attaching random class to each selector
                style.push_str(&random_class);
            }
            TokenTree::Literal(t) => {
                let start = t.span().start();
                panic!(
                    "Literal strings are not allowed in selectors line:{},col:{}",
                    start.line, start.column
                );
            }
            TokenTree::Punct(t) => {
                let start = t.span().start();
                let end = t.span().end();
                let cur_col = start.column;
                let cur_line = start.line;
                if pre_line == cur_line && cur_col > pre_col {
                    style.push(' ');
                } else if pre_line == cur_line {
                    //remove the class we have added in previous step.
                    //we might have added generated class for joint selector(ex. div.class1)
                    //only if the current selector and previous selector are in same line
                    let len = style.len();
                    if len > 9 {
                        style = String::from(&style[0..len - 9]);
                    }
                }
                style.push(t.as_char());
                pre_col = end.column;
                pre_line = end.line;
            }
        }
    });
    write_to_file(style);
}

//parse each each css selector body. this function recursively calls itself
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
            body.push_str(&parse_line(t.span(), &mut pre_line, &mut pre_col));
            body.push_str(&parse_body(t));
        }
        TokenTree::Ident(t) => {
            body.push_str(&parse_line(t.span(), &mut pre_line, &mut pre_col));
            body.push_str(&t.to_string());
        }
        TokenTree::Literal(t) => {
            body.push_str(&parse_line(t.span(), &mut pre_line, &mut pre_col));
            body.push_str(&t.to_string());
        }
        TokenTree::Punct(t) => {
            body.push_str(&parse_line(t.span(), &mut pre_line, &mut pre_col));
            body.push(t.as_char());
        }
    });
    body.push(closing);
    body
}

//parse each css line and add neccessary spaces
fn parse_line(span: proc_macro::Span, pre_line: &mut usize, pre_col: &mut usize) -> String {
    let mut temp = String::new();
    let start = span.start();
    let end = span.end();
    let cur_col = start.column;
    let cur_line = start.line;
    if *pre_line == cur_line && cur_col > *pre_col {
        temp.push(' ');
    }
    *pre_col = end.column;
    *pre_line = end.line;
    temp
}

//append if file exists or write it into the new file
fn write_to_file(data: String) {
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open("out.css")
        .unwrap_or_else(|err| {
            if err.kind() == ErrorKind::NotFound {
                File::create("main.css").unwrap_or_else(|err| {
                    panic!("Problem creating the file: {:?}", err);
                })
            } else {
                panic!("Problem opening the file: {:?}", err);
            }
        });
    let _ = file
        .write_all(data.as_bytes())
        .expect("Problem writing to file");
}

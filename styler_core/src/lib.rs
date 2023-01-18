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
                let ch = t.as_char();
                //only in these two cases we need space information
                if ch == '.' || ch == '#' {
                    add_spaces(&mut selector, t.span(), &mut pre_line, &mut pre_col);
                } else {
                    let end = t.span().unwrap().end();
                    pre_col = end.column;
                    pre_line = end.line;
                }
                selector.push(t.as_char());
            }
        }
    });
    dbg!(&sel_map);
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

fn append_selector(
    source: &mut String,
    selector: &str,
    random_class: &str,
    sel_map: &mut HashMap<String, ()>,
) {
    // dbg!(&selector);
    let sel_len = selector.len();
    let mut ignore_spaces = false;
    let mut ignore_untill_space = false;
    let mut ignore_untill_close = false;
    let mut ignore_untill_event_end = false;
    let mut temp = String::new();
    let mut i = 0;
    for c in selector.chars() {
        i += 1;

        //ignore everything between square brackets.
        if ignore_untill_close && c != ']' {
            source.push(c);
            temp.push(c);
            continue;
        }
        if ignore_untill_close && c == ']' {
            source.push(c);
            ignore_untill_close = false;
            source.push_str(random_class);

            temp.push(c);
            sel_map.insert(temp.clone(), ());
            temp = String::new();
            continue;
        }
        if c == '[' {
            source.push(c);
            ignore_untill_close = true;
            temp.push(c);
            continue;
        }

        //ignore everything until we reach to whitespace.
        if ignore_untill_event_end && c != ' ' {
            source.push(c);
            continue;
        }
        if ignore_untill_event_end && c == ' ' {
            ignore_untill_event_end = false;
            source.push(' ');
            continue;
        }
        //check for event selector
        if c == '@' {
            ignore_untill_event_end = true;
            source.push(c);
            continue;
        }

        //ignore everything until we reach to whitespace or end of the line.
        if ignore_untill_space && (c == ' ' || i == sel_len) {
            //this condition will should be true when either we reach space or end of line.
            source.push(c);
            ignore_untill_space = false;

            if c != ' ' {
                temp.push(c);
            }
            sel_map.insert(temp.clone(), ());
            temp = String::new();
            continue;
        }
        if ignore_untill_space && c != ' ' {
            source.push(c);
            temp.push(c);
            continue;
        }
        //check for pseudo class selector
        if c == ':' {
            ignore_untill_space = true;
            source.push_str(random_class);
            source.push(c);
            temp.push(c);
            continue;
        }

        //check for child or sibiling selectors
        //this condition ignores the unwanted white space after comma, >, +, ~ punctuations.
        if c == ' ' && ignore_spaces {
            ignore_spaces = false;
            continue;
        }
        if c == ',' || c == '+' || c == '~' || c == '>' {
            source.push_str(random_class);
            source.push(c);
            ignore_spaces = true;

            sel_map.insert(temp.clone(), ());
            temp = String::new();
            continue;
        }

        //check for universal selector.
        if c == '*' {
            source.push_str(random_class);
            sel_map.insert("*".to_string(), ());
            continue;
        }

        //append random class if we reach end of the line.
        if i == sel_len {
            source.push(c);
            source.push_str(random_class);

            temp.push(c);
            sel_map.insert(temp.clone(), ());
            temp = String::new();
            continue;
        }

        //check for direct child selector
        if c != ' ' {
            source.push(c);
            temp.push(c);
            continue;
        }
        if c == ' ' {
            source.push_str(random_class);
            source.push(' ');

            sel_map.insert(temp.clone(), ());
            temp = String::new();
        }
    }
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

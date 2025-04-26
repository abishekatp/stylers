use proc_macro2::{Delimiter, Group, TokenTree};

// This parse_group function will parse the TokenTree::Group and return a string.
// This function will add at most one whitespace even if there are many whitespaces in actual tokenstream.
pub(crate) fn parse_group(group: Group, is_proc_macro: bool) -> String {
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
      add_spaces(
        &mut body,
        t.span(),
        &mut pre_line,
        &mut pre_col,
        is_proc_macro,
      );
      body.push_str(&parse_group(t, is_proc_macro));
    }
    TokenTree::Ident(t) => {
      add_spaces(
        &mut body,
        t.span(),
        &mut pre_line,
        &mut pre_col,
        is_proc_macro,
      );
      body.push_str(&t.to_string());
    }
    TokenTree::Literal(t) => {
      add_spaces(
        &mut body,
        t.span(),
        &mut pre_line,
        &mut pre_col,
        is_proc_macro,
      );
      //we are trimming r because in some cases like "\1g34" is not valid rust syntax.
      //in those places user have to use r"\1g34".
      body.push_str(t.to_string().trim_start_matches('r').trim_matches('#'));
    }
    TokenTree::Punct(t) => {
      add_spaces(
        &mut body,
        t.span(),
        &mut pre_line,
        &mut pre_col,
        is_proc_macro,
      );
      body.push(t.as_char());
    }
  });
  body.push(closing);
  body
}

//check if spaces needed to be appended
//note: this function also reset the pre_line and pre_col to the cureent token's end line and column
//note: this function convert proc_macro2::Span to proc_macro::Span
// https://docs.rs/proc-macro2/latest/proc_macro2/struct.Span.html#method.start
pub(crate) fn add_spaces(
  source: &mut String,
  span: proc_macro2::Span,
  pre_line: &mut usize,
  pre_col: &mut usize,
  is_proc_macro: bool,
) {
  let mut start_col = span.start().column;
  let mut start_line = span.start().line;
  let mut end_col = span.end().column;
  let mut end_line = span.end().line;
  if is_proc_macro {
    start_col = span.unwrap().start().column();
    start_line = span.unwrap().start().line();
    end_col = span.unwrap().end().column();
    end_line = span.unwrap().end().line();
  }
  let cur_col = start_col;
  let cur_line = start_line;
  if *pre_line == cur_line && cur_col > *pre_col {
    source.push(' ');
  }
  *pre_col = end_col;
  *pre_line = end_line;
}

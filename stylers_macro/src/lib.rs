//! This crate provides style macro for scoped css in rust web frameworks which follows component like architecture e.g Leptos.
#![feature(proc_macro_span)]
#![warn(clippy::panic, clippy::unwrap_used, clippy::expect_used, clippy::cargo)]

use std::fs;
use std::path::Path;

use litrs::StringLit;
use proc_macro2::{self, TokenStream, TokenTree};
use quote::quote;

use stylers_core::Class;
use stylers_core::{from_str, from_ts};

/// style macro take any valid css as input and returns a unique class name.
/// For examples see: <https://github.com/abishekatp/stylers>
#[proc_macro]
pub fn style(ts: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let strval = ts.to_string();
  let class = Class::rand_class_from_seed(strval);
  let class = class.as_name();
  let expanded = quote! {
      #class
  };
  proc_macro::TokenStream::from(expanded)
}

/// style_sheet macro take css file path as a string input and returns a unique class name.
/// For examples see: <https://github.com/abishekatp/stylers>
#[proc_macro]
pub fn style_sheet(ts: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let file_path = ts.to_string();
  let file_path = file_path.trim_matches('"');
  let css_content = std::fs::read_to_string(file_path).expect("Expected to read file");
  let class = Class::rand_class_from_seed(css_content.to_string());
  let class = class.as_name();
  let expanded = quote! {
      #class
  };
  proc_macro::TokenStream::from(expanded)
}

#[proc_macro]
pub fn style_str(token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let token_stream = TokenStream::from(token_stream).into_iter();
  let expanded = style_str_(token_stream).unwrap_or_else(|err| quote! { compile_error!(#err) });
  proc_macro::TokenStream::from(expanded)
}

fn style_str_(token_stream: impl Iterator<Item = TokenTree>) -> Result<TokenStream, String> {
  let class = Class::random();
  let (style, _selectors) = from_ts(token_stream, &class, true);

  Ok(quote! { (#class, #style) })
}

#[proc_macro]
pub fn style_sheet_str(token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let token_stream = TokenStream::from(token_stream).into_iter();
  let expanded =
    style_sheet_str_(token_stream).unwrap_or_else(|err| quote! { compile_error!(#err) });
  proc_macro::TokenStream::from(expanded)
}

fn style_sheet_str_(token_stream: impl Iterator<Item = TokenTree>) -> Result<TokenStream, String> {
  let tokens = &token_stream.collect::<Vec<_>>();
  let &[TokenTree::Literal(path_literal)] = &tokens.as_slice() else {
    return Err("Expected only a string literal".to_string());
  };

  let path = StringLit::try_from(path_literal)
    .map_err(|err| format!("Expected a string literal: {}", err))?;
  let path = Path::new(path.value());

  let style_sheet_content = fs::read_to_string(path).map_err(|_| "Expected to read file")?;

  let class = Class::random();
  let style = from_str(&style_sheet_content, &class);

  Ok(quote! { (#class, #style) })
}

#[doc(hidden)]
#[proc_macro]
pub fn style_sheet_test(token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let token_stream = TokenStream::from(token_stream).into_iter();
  let expanded =
    style_sheet_test_(token_stream).unwrap_or_else(|err| quote! { compile_error!(#err) });
  proc_macro::TokenStream::from(expanded)
}

fn style_sheet_test_(token_stream: impl Iterator<Item = TokenTree>) -> Result<TokenStream, String> {
  let tokens = &token_stream.collect::<Vec<_>>();
  let &[TokenTree::Literal(path_literal)] = &tokens.as_slice() else {
    return Err("Expected only a string literal".to_string());
  };

  let path = StringLit::try_from(path_literal)
    .map_err(|err| format!("Expected a string literal: {}", err))?;
  let path = Path::new(path.value());

  let style_sheet_content = fs::read_to_string(path).map_err(|_| "Expected to read file")?;

  let class = Class::new("test".into());
  let style = from_str(&style_sheet_content, &class);

  Ok(quote! { #style })
}

#[doc(hidden)]
#[proc_macro]
pub fn style_test(token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let token_stream = TokenStream::from(token_stream).into_iter();
  let expanded = style_test_(token_stream).unwrap_or_else(|err| quote! { compile_error!(#err) });
  proc_macro::TokenStream::from(expanded)
}

fn style_test_(token_stream: impl Iterator<Item = TokenTree>) -> Result<TokenStream, String> {
  let token_stream = token_stream.into_iter();

  let class = Class::new("test".into());
  let (style, _selectors) = from_ts(token_stream, &class, true);

  Ok(quote! { #style })
}

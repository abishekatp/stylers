//! This crate provides style macro for scoped css in rust web frameworks which follows component like architecture e.g Leptos.
//!
//! See <https://github.com/abishekatp/stylers/tree/main/examples> for examples.

#![feature(proc_macro_span)]
#![warn(clippy::panic, clippy::unwrap_used, clippy::expect_used, clippy::cargo)]

mod style;
mod style_sheet;

use litrs::StringLit;
use proc_macro2::{self, TokenStream, TokenTree};
use quote::{quote, ToTokens};

use std::collections::hash_map::RandomState;
use std::fs::{self, File, OpenOptions};
use std::hash::{BuildHasher, Hasher};
use std::io::{self, Write};
use std::path::Path;

use crate::style::build_style;

#[proc_macro]
pub fn style(token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let token_stream = TokenStream::from(token_stream).into_iter();
    let expanded = style_(token_stream).unwrap_or_else(|err| quote! { compile_error!(#err) });
    proc_macro::TokenStream::from(expanded)
}

fn style_(mut token_stream: impl Iterator<Item = TokenTree>) -> Result<TokenStream, String> {
    let component_name = extract_component_name(&mut token_stream)?;

    let class = Class::random();
    let (style, _sel_map) = build_style(token_stream, &class);

    write_css(&component_name, &style).map_err(|err| format!("Failed to write css: {}", err))?;
    aggregate_css().map_err(|err| format!("Failed to write css: {}", err))?;

    Ok(quote! { #class })
}

#[proc_macro]
pub fn style_sheet(token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let token_stream = TokenStream::from(token_stream).into_iter();
    let expanded = style_sheet_(token_stream).unwrap_or_else(|err| quote! { compile_error!(#err) });
    proc_macro::TokenStream::from(expanded)
}

fn style_sheet_(token_stream: impl Iterator<Item = TokenTree>) -> Result<TokenStream, String> {
    let tokens = &token_stream.collect::<Vec<_>>();
    let &[TokenTree::Literal(path_literal)] = &tokens.as_slice() else {
        return Err("Expected only string literal".to_string());
    };

    let path = StringLit::try_from(path_literal)
        .map_err(|err| format!("Expected a string literal: {}", err))?;
    let path = Path::new(path.value());

    let style_sheet_content = fs::read_to_string(path).map_err(|_| "Expected to read file")?;

    let class = Class::random();
    let style = style_sheet::build_style(&style_sheet_content, &class);

    let filename = path
        .file_name()
        .ok_or("Path is suppose to point to a file, not a folder".to_string())?
        .to_string_lossy()
        .into_owned();
    let filename = filename
        .strip_suffix(".css")
        .ok_or("The file you are trying to load is not a `.css` one".to_string())?;

    write_css(filename, &style).map_err(|err| format!("Failed to write css: {}", err))?;
    aggregate_css().map_err(|err| format!("Failed to write css: {}", err))?;

    Ok(quote! { #class })
}

#[proc_macro]
pub fn style_str(token_stream: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let token_stream = TokenStream::from(token_stream).into_iter();
    let expanded = style_str_(token_stream).unwrap_or_else(|err| quote! { compile_error!(#err) });
    proc_macro::TokenStream::from(expanded)
}

fn style_str_(token_stream: impl Iterator<Item = TokenTree>) -> Result<TokenStream, String> {
    let class = Class::random();
    let (style, _selectors) = build_style(token_stream, &class);

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
    let style = style_sheet::build_style(&style_sheet_content, &class);

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
    let style = style_sheet::build_style(&style_sheet_content, &class);

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
    let mut token_stream = token_stream.into_iter();

    let _component_name = extract_component_name(&mut token_stream)?;

    let class = Class::new("test".into());
    let (style, _selectors) = build_style(token_stream, &class);

    Ok(quote! { #style })
}

#[derive(Debug)]
struct Class(String);

impl Class {
    fn new(class: String) -> Self {
        Self(class)
    }

    fn random() -> Self {
        let hash = RandomState::new().build_hasher().finish();

        Self(format!("l-{}", &hash.to_string()[0..6]))
    }

    fn as_name(&self) -> &str {
        &self.0
    }

    fn as_selector(&self) -> String {
        format!(".{}", self.0)
    }
}

impl ToTokens for Class {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let class = self.as_name();
        tokens.extend(quote! { #class })
    }
}

const OUTPUT_DIR: &str = "./target/stylers";

/// Writes the styles in its own file and appends itself to the main.css file
fn write_css(filename: &str, content: &str) -> io::Result<()> {
    let output_dir = Path::new(OUTPUT_DIR).join("css");

    let filename = Path::new(&filename.to_lowercase()).with_extension(".css");
    let filepath = output_dir.join(filename);

    fs::create_dir_all(&output_dir)?;

    let mut buffer = File::create(filepath)?;
    buffer.write_all(content.as_bytes())?;
    buffer.flush()?;

    Ok(())
}

// TODO: this may scale poorly, maybe find a way to aggregate the css files only once per compilation.
/// Aggregates the output from all CSS files into a single main.css file
fn aggregate_css() -> io::Result<()> {
    let output_dir = Path::new(OUTPUT_DIR);
    let output_file = output_dir.join("main.css");

    File::create(&output_file)?;
    let mut buffer = OpenOptions::new().append(true).open(&output_file)?;

    let files = fs::read_dir(output_dir.join("css"))?;

    for file in files.filter_map(|file| file.ok()) {
        let data = fs::read_to_string(file.path())?;
        buffer.write_all(data.as_bytes())?;
    }

    buffer.flush()?;

    Ok(())
}

/// Eats the first two tokens of the macro input, checks is this is a component name and a comma.
fn extract_component_name(stream: &mut impl Iterator<Item = TokenTree>) -> Result<String, String> {
    let Some(TokenTree::Literal(name_literal)) = stream.next() else {
        return Err(String::from(
            r#"Expected component name at the start like style!("component_name", your css comes here)"#,
        ));
    };

    let component_name = StringLit::try_from(name_literal).map_err(|err| err.to_string())?;

    let Some(TokenTree::Punct(punctuation)) = stream.next() else {
        return Err("Expected comma `,` after component name".to_string());
    };

    if punctuation.as_char() != ',' {
        return Err("Expected comma `,` after component name".to_string());
    }

    Ok(component_name.value().to_owned())
}

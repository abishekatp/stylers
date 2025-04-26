#![feature(extend_one)]
#![feature(proc_macro_span)]
mod style;
mod style_sheet;

use proc_macro2::TokenStream;
use quote::{ToTokens, quote};
use rand::prelude::*;
use rand_chacha::ChaCha8Rng;
use std::collections::hash_map::RandomState;
use std::hash::{BuildHasher, Hasher};

pub use style::build_style_from_ts as from_ts;
pub use style_sheet::build_style_from_str as from_str;

//ref: https://rust-random.github.io/book/guide-seeding.html

#[derive(Debug)]
pub struct Class(String);

impl Class {
  pub fn new(class: String) -> Self {
    Self(class)
  }

  pub fn random() -> Self {
    let hash = RandomState::new().build_hasher().finish();

    Self(format!("l-{}", &hash.to_string()[0..6]))
  }

  pub fn rand_class_from_seed(content: String) -> Self {
    let mut no_of_chars = 0;
    for ch in content.chars() {
      if !ch.is_whitespace() && !ch.is_whitespace() {
        no_of_chars += 1;
      }
    }
    let mut rng = ChaCha8Rng::seed_from_u64(no_of_chars);
    let hash = rng.r#gen::<i32>();
    Self(format!("l-{}", &hash.to_string()[0..6]))
  }

  pub fn as_name(&self) -> &str {
    &self.0
  }

  pub fn as_selector(&self) -> String {
    format!(".{}", self.0)
  }
}

impl ToTokens for Class {
  fn to_tokens(&self, tokens: &mut TokenStream) {
    let class = self.as_name();
    tokens.extend(quote! { #class })
  }
}

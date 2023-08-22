#![feature(extend_one)]
mod style;
mod style_sheet;

use rand::prelude::*;
use rand_chacha::ChaCha8Rng;

pub use style::build_style_from_ts as from_ts;
pub use style_sheet::build_style_from_str as from_str;

//ref: https://rust-random.github.io/book/guide-seeding.html
pub fn rand_class_from_seed(content: String) -> String {
    let mut no_of_chars = 0;
    for ch in content.chars() {
        if !ch.is_whitespace() && !ch.is_whitespace() {
            no_of_chars += 1;
        }
    }
    let mut rng = ChaCha8Rng::seed_from_u64(no_of_chars);
    let hash = rng.gen::<i32>();
    format!(".l-{}", hash.to_string())
}

#![feature(proc_macro_span)]
#![feature(extend_one)]
mod style;
mod style_sheet;

pub use style::build_style_from_ts as from_ts;
pub use style_sheet::build_style_from_str as from_str;

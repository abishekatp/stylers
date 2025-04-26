//! ## Feature flags
#![doc = document_features::document_features!()]
//!
//! ## Usage
//! An example build.rs
//! ```rust
#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/examples/build.rs"))]
//! ```

pub use stylers_macro::style;
pub use stylers_macro::style_sheet;
pub use stylers_macro::style_sheet_str;
pub use stylers_macro::style_str;

#[cfg(feature = "build")]
pub use build::*;
#[cfg(feature = "build")]
mod build;

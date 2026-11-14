//! # Hanconv
//!
//! Convert between Chinese characters variants.
//!
//! ## Getting Started
//!
//! Add `hanconv` as a dependency in your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! hanconv = "0.5"
//! ```
//!
//! ## Example
//!
//! ```rust
//! let result = hanconv::s2t("简繁转换");
//! assert_eq!(result, "簡繁轉換");
//! ```

mod conv;
mod dict;
mod map;
mod trie;

pub use conv::{
    Conversion::{self, *},
    Converter, hk2s, hk2t, jp2t, s2hk, s2t, s2tw, s2twp, t2hk, t2jp, t2s, t2tw, tw2s, tw2sp, tw2t,
};
pub use dict::{
    Dictionary::{self, *},
    TextDictionary,
};
pub use trie::{Node, Trie};

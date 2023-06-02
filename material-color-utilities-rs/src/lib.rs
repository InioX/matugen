//! Utilities for creating Material 3 colour schemes
//!
//! It has optional Serde serialisation support, using the `serde` feature.
//!
//! ```toml
//! material-color-utilities-rs = {version = "0", features=["serde"]}
//! ```

pub mod blend;
pub mod htc;
pub mod palettes;
pub mod quantize;
pub mod scheme;
pub mod score;
pub mod util;

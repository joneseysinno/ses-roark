//! Beams with wide flanges; shear lag (Roark 8.12).
//!
//! Shear-lag effective-width ratios depend on span, flange width, and load
//! distribution and are taken from published charts; callers supply them.
//! Compression flanges thin enough to buckle use von Kármán's effective width.

mod compression_flange_effective_width;
mod effective_width;

pub use compression_flange_effective_width::compression_flange_effective_width;
pub use effective_width::effective_width;

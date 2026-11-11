//! Shared assertions and fixtures for the Chapter 8 tests.

mod assert_close;
mod assert_matches_expected;
mod end_residuals;
mod euler_load;
mod mixed_loads;
mod portal;
mod rotated_rectangle;
mod uniform_simple_beam;

pub use assert_close::assert_close;
pub use assert_matches_expected::assert_matches_expected;
pub use end_residuals::end_residuals;
pub use euler_load::euler_load;
pub use mixed_loads::mixed_loads;
pub use portal::portal;
pub use rotated_rectangle::rotated_rectangle;
pub use uniform_simple_beam::uniform_simple_beam;

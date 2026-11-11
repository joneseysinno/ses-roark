//! Ultimate strength and plastic design of straight beams (Roark 8.15, 8.16;
//! Table 8.13 collapse loads computed).
//!
//! Collapse load factors use the mechanism (upper-bound) method: for each
//! span the factor is minimized over the position of the interior hinge, with
//! end hinges wherever the end is fixed or continuous.

mod collapse_load_factor;
mod continuous_collapse_load_factor;
mod external_work;
mod mechanism_load_factor;
mod shape_factor;

pub use collapse_load_factor::collapse_load_factor;
pub use continuous_collapse_load_factor::continuous_collapse_load_factor;
pub use shape_factor::shape_factor;

//! Independent textbook closed forms for Table 8.1 cases 1 and 2 (uniform),
//! written without the engine. Deflections are converted to y-up.

mod case_1a;
mod case_1b;
mod case_1c;
mod case_1d;
mod case_1e;
mod case_1f;
mod case_2a;
mod case_2b;
mod case_2c;
mod case_2d;
mod case_2e;
mod case_2f;
mod expected;

pub use case_1a::case_1a;
pub use case_1b::case_1b;
pub use case_1c::case_1c;
pub use case_1d::case_1d;
pub use case_1e::case_1e;
pub use case_1f::case_1f;
pub use case_2a::case_2a;
pub use case_2b::case_2b;
pub use case_2c::case_2c;
pub use case_2d::case_2d;
pub use case_2e::case_2e;
pub use case_2f::case_2f;
pub use expected::Expected;

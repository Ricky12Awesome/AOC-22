pub use std::collections::HashMap;
pub use std::collections::HashSet;
pub use std::fmt::Debug;
pub use std::str::Chars;

#[macro_use]
pub mod day;
pub mod days;

#[macro_use]
pub mod prelude {
  pub use ::itertools::*;

  pub use crate::day::*;
}

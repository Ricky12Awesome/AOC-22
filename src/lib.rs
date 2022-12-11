#![feature(iter_array_chunks)]
#![feature(entry_insert)]
#![feature(int_roundings)]

pub use std::{
  collections::{HashMap, HashSet},
  fmt::Debug,
  str::Chars,
};

#[macro_use]
pub mod day;
pub mod days;
pub mod misc;

#[macro_use]
pub mod prelude {
  pub use ::std::{cell::RefCell, collections::HashSet, fmt::Debug, ops::Index, str::FromStr};

  pub use ::itertools::*;
  pub use ::parse_display::FromStr;

  pub use crate::{day::*, misc::*};
}

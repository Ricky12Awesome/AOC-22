#![feature(iter_array_chunks)]
#![feature(entry_insert)]
#![feature(int_roundings)]
#![feature(exact_size_is_empty)]
#![feature(array_windows)]

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
  pub use ::std::{
    cell::RefCell,
    collections::{BTreeMap, BTreeSet, HashMap, HashSet},
    fmt::Debug,
    ops::Index,
    str::FromStr,
  };

  pub use ::itertools::*;
  pub use ::parse_display::FromStr;
  pub use ::regex::Regex;
  pub use ::serde::{Deserialize, Serialize};

  pub use crate::{day::*, misc::*};
}

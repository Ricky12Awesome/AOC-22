#![allow(incomplete_features)]
#![feature(iter_array_chunks)]
#![feature(entry_insert)]
#![feature(int_roundings)]
#![feature(exact_size_is_empty)]
#![feature(array_windows)]
#![feature(slice_take)]
#![feature(generic_const_exprs)]
#![feature(build_hasher_simple_hash_one)]

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

  pub use ::arrayvec::ArrayVec;
  pub use ::indexmap::{IndexMap, IndexSet};
  pub use ::rayon::prelude::*;
  pub use ::itertools::*;
  pub use ::parse_display::{FromStr, Display};
  pub use ::regex::Regex;
  pub use ::serde::{Deserialize, Serialize};

  pub use crate::{day::*, misc::*};
}

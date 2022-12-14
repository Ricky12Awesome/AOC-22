use itertools::{
  FoldWhile::{Continue, Done},
  Itertools,
};
use std::{
  fmt::{Display, Formatter},
  time::{Duration, Instant},
};

pub fn parse_u32(str: &str) -> u32 {
  str.parse().unwrap()
}

pub fn parse_u128(str: &str) -> u128 {
  str.parse().unwrap()
}

pub fn parse_u64(str: &str) -> u64 {
  str.parse().unwrap()
}

pub fn parse_usize(str: &str) -> usize {
  str.parse().unwrap()
}

pub fn char_to_u32(c: char) -> u32 {
  c.to_digit(10).unwrap()
}

pub fn char_to_u8(c: char) -> u8 {
  c.to_digit(10).unwrap() as u8
}

pub fn char_to_usize(c: char) -> usize {
  c.to_digit(10).unwrap() as usize
}

pub fn char_to_isize(c: char) -> isize {
  c.to_digit(10).unwrap() as isize
}

pub struct Timer(Instant);

impl Timer {
  pub fn start() -> Self {
    Timer(Instant::now())
  }

  pub fn elapsed(&self) -> Duration {
    self.0.elapsed()
  }
}

impl Display for Timer {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    std::fmt::Debug::fmt(&self.0.elapsed(), f)
  }
}

pub trait CountInclusive: Iterator {
  fn count_inclusive(&mut self, mut predicate: impl FnMut(Self::Item) -> bool) -> usize
  where
    Self: Sized,
  {
    self
      .fold_while(0, |acc, value| {
        if predicate(value) {
          Continue(acc + 1)
        } else {
          Done(acc + 1)
        }
      })
      .into_inner()
  }
}

impl <I: Iterator> CountInclusive for I {

}

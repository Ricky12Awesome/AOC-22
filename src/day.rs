use std::fmt::{Display, Formatter};
use std::str::FromStr;
use clap::Parser;

#[macro_export]
macro_rules! answer {
  ($answer:expr) => {
    Some($answer)
  };
  ($part:expr, $p1:expr, $p2:expr) => {
    match $part {
      Part::One => (answer!($p1), None),
      Part::Two => (None, answer!($p2)),
      Part::Both => (answer!($p1), answer!($p2)),
    }
  };
}

#[macro_export]
macro_rules! input_path {
  ($name:literal) => {
    concat!("../../input/Day", stringify!($name), ".txt")
  };
}

#[macro_export]
macro_rules! day {
  ($name:literal, $p1:expr, $p2:expr, |$part:ident, $input:ident| -> $T:tt$(, $T2:tt)? $solve:block) => {
    use $crate::prelude::*;

    pub fn solve($part: Part, $input: &str) -> Answer<$T$(, $T2)?> $solve

    #[allow(unused)]
    pub const INPUT: &'static str = include_str!(input_path!($name));

    // generate_tests_for_day!($p1, $p2);
  };
}

#[macro_export]
macro_rules! generate_tests_for_day {
  ($p1:expr, $p2:expr) => {
    #[cfg(test)]
    #[test]
    pub fn both() {
      let (part1, part2) = solve(Part::Both, INPUT);
      let p1 = $p1;
      let p2 = $p2;

      if let Some(answer) = &part1 {
        println!("Part 1: {answer}");
      }

      if let Some(answer) = &part2 {
        println!("Part 2: {answer}");
      }

      if let Some(_) = p1 {
        assert_eq!(part1, p1);
      }

      if let Some(_) = p2 {
        assert_eq!(part2, p2);
      }
    }

    #[cfg(test)]
    #[test]
    pub fn part1() {
      let (part1, _) = solve(Part::One, INPUT);
      let p1 = $p1;

      if let Some(answer) = &part1 {
        println!("Part 1: {answer}");
      }

      if let Some(_) = p1 {
        assert_eq!(part1, p1);
      }
    }

    #[cfg(test)]
    #[test]
    pub fn part2() {
      let (_, part2) = solve(Part::Two, INPUT);
      let p2 = $p2;

      if let Some(_) = p2 {
        assert_eq!(part2, p2);
      }

      if let Some(answer) = &part2 {
        println!("Part 2: {answer}");
      }
    }
  };
}

#[derive(Default, Debug, Copy, Clone)]
pub enum Part {
  One,
  Two,

  #[default]
  Both,
}

impl FromStr for Part {
  type Err = String;

  fn from_str(str: &str) -> Result<Self, Self::Err> {
    match str {
      "1" | "one" | "One" => Ok(Part::One),
      "2" | "two" | "Two" => Ok(Part::Two),
      "3" | "both" | "Both" => Ok(Part::Both),
      _ => Err(String::from("")),
    }
  }
}

impl Display for Part {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::One => write!(f, "1"),
      Self::Two => write!(f, "2"),
      Self::Both => write!(f, "Both"),
    }
  }
}

pub type Answer<P1, P2 = P1> = (Option<P1>, Option<P2>);

#[derive(Debug, Parser)]
pub struct DayArgs {
  #[clap(long, short)]
  pub part: Option<Part>,

  #[clap(long, short)]
  pub input: Option<String>,

  #[clap(long, short)]
  pub benchmark: bool,

  #[clap(long, short)]
  pub warmup: bool,

  #[clap(long, short)]
  pub time: Option<u64>
}

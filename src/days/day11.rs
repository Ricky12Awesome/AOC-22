#![allow(unused)]

use std::{cell::RefCell, ops::Index, str::FromStr};
// day!(Day11, Some(0), Some(0));
day!(Day11);

#[derive(Debug)]
struct Monkey {
  index: usize,
  inspected: u128,
  items: Vec<u128>,
  operation: Operation,
  test: Test,
}

#[derive(Debug, FromStr, Copy, Clone)]
#[rustfmt::skip]
enum Operation {
  #[display("+ old")] PlusOld,
  #[display("+ {0}")] PlusNew(u128),
  #[display("* old")] TimesOld,
  #[display("* {0}")] TimesNew(u128),
}

impl Operation {
  fn calc(self, value: u128) -> u128 {
    match self {
      Operation::PlusOld => value + value,
      Operation::PlusNew(n) => value + n,
      Operation::TimesOld => value * value,
      Operation::TimesNew(n) => value * n,
    }
  }
}

#[derive(Debug, Copy, Clone)]
struct Test {
  by: u128,
  if_true: usize,
  if_false: usize,
}

impl FromStr for Monkey {
  type Err = anyhow::Error;

  fn from_str(str: &str) -> Result<Self, Self::Err> {
    let mut lines = str.lines();

    Ok(Self {
      index: lines.next().unwrap().index(..1).parse()?,
      inspected: 0,
      items: lines
        .next()
        .unwrap()
        .index(18..)
        .split(", ")
        .map(parse_u128)
        .collect(),
      operation: lines.next().unwrap().index(23..).parse()?,
      test: Test {
        by: lines.next().unwrap().index(21..).parse()?,
        if_true: lines.next().unwrap().index(29..).parse()?,
        if_false: lines.next().unwrap().index(30..).parse()?,
      },
    })
  }
}

impl<'a> From<&'a str> for Monkey {
  fn from(str: &'a str) -> Self {
    str.parse::<Self>().unwrap()
  }
}

impl Test {
  fn is_divisible(self, n: u128) -> usize {
    if n % self.by == 0 {
      self.if_true
    } else {
      self.if_false
    }
  }
}

impl Monkey {
  fn throw_items(&mut self) -> impl Iterator<Item = (usize, u128)> + '_ {
    self
      .items
      .drain(..)
      .map(|lvl| self.operation.calc(lvl))
      .inspect(|_| self.inspected += 1)
      .map(|lvl| lvl.div_floor(3))
      .map(|lvl| (self.test.is_divisible(lvl), lvl))
  }
}

impl Day11 {
  pub fn day(part: Part) -> Answer<u128> {
    let mut monkeys = Self::INPUT
      .split("Monkey ")
      .filter(|s| !s.is_empty())
      .map(Monkey::from)
      .map(RefCell::new)
      .collect_vec();

    for _ in 0..20 {
      for mut monkey in monkeys.iter().map(RefCell::borrow_mut) {
        for (to, item) in monkey.throw_items() {
          monkeys[to].borrow_mut().items.push(item)
        }
      }
    }

    let (a, b) = monkeys
      .iter()
      .map(RefCell::borrow)
      .map(|m| m.inspected)
      .sorted_unstable()
      .rev()
      .take(2)
      .collect_tuple::<(_, _)>()
      .unwrap();

    let part1 = || a * b;
    let part2 = || 0;

    answer!(part, part1, part2)
  }
}

use std::cell::RefCell;

use regex::Regex;

day!(
  Day05,
  Some(String::from("BWNCQRMDB")),
  Some(String::from("NHWZCBNBF"))
);
// day!(Day05);

type Crate = char;

#[derive(Debug, Default)]
struct Crates {
  crates: Vec<Crate>,
  last: usize,
}

impl Crates {
  fn top(&self) -> Crate {
    self.crates[self.last - 1]
  }

  fn free(&mut self, amount: usize) -> &mut [Crate] {
    if self.crates.len() < self.last + amount + 1 {
      self
        .crates
        .extend_from_slice(&vec![Crate::default(); amount + 1]);
    }

    &mut self.crates[self.last..][..amount]
  }

  fn last(&mut self, amount: usize) -> &mut [Crate] {
    &mut self.crates[self.last - amount..self.last]
  }

  fn swap(&mut self, amount: usize, other: &mut Crates, reverse: bool) {
    let take = self.last(amount);
    let free = other.free(amount);

    if reverse {
      take.reverse();
    }
    take.swap_with_slice(free);

    self.last -= amount;
    other.last += amount;
  }
}

#[derive(Debug, Default)]
struct Stacks {
  stacks: Vec<RefCell<Crates>>,
  reverse: bool,
}

impl Stacks {
  fn add(&mut self, stack: usize, crate_: Crate) {
    if self.stacks.len() <= stack {
      self.stacks.push(RefCell::default())
    }

    let mut stack = self.stacks[stack].borrow_mut();

    stack.crates.push(crate_);
    stack.last += 1;
  }

  fn move_crates(&mut self, amount: usize, a: usize, b: usize) {
    let mut a = self.stacks[a - 1].borrow_mut();
    let mut b = self.stacks[b - 1].borrow_mut();

    a.swap(amount, &mut b, self.reverse)
  }

  fn top(&self) -> Vec<Crate> {
    self
      .stacks
      .iter()
      .map(|crate_| crate_.borrow().top())
      .collect_vec()
  }

  fn parse(reverse: bool, input: &str) -> Self {
    let mut stacks = Stacks::default();
    let mut lines = input.lines();

    stacks.reverse = reverse;

    let regex = Regex::new(r"( \[|\[|]|] )").unwrap();

    let mut crates = lines
      .take_while_ref(|line| !line.starts_with(" 1"))
      .map(|line| regex.replace_all(line, ""))
      .map(|line| line.replace("  ", " "))
      .map(|line| line.replace("  ", " "))
      .collect_vec();

    crates.reverse();

    for crate_ in crates {
      for (i, c) in crate_.char_indices() {
        if c != ' ' {
          stacks.add(i, c)
        }
      }
    }

    let regex = Regex::new("[a-z] ?").unwrap();

    for line in lines
      .filter(|line| line.starts_with("move"))
      .map(|line| regex.replace_all(line, ""))
    {
      let (amount, a, b) = line
        .split(' ')
        .map(parse_usize)
        .collect_tuple::<(_, _, _)>()
        .unwrap();

      stacks.move_crates(amount, a, b);
    }

    stacks
  }
}

impl Day05 {
  pub fn day(part: Part) -> Answer<String> {
    let parse = |reverse| {
      Stacks::parse(reverse, Self::INPUT)
        .top()
        .iter()
        .collect::<String>()
    };

    let part1 = || parse(true);
    let part2 = || parse(false);

    answer!(part, part1, part2)
  }
}

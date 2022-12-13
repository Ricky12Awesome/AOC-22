day!(
  05,
  Some(String::from("BWNCQRMDB")),
  Some(String::from("NHWZCBNBF")),
  |part, input| -> String {
    let parse = |reverse| Stacks::parse(reverse, input).top().collect::<_>();

    answer!(part, parse(true), parse(false))
  }
);

type Crate = char;

#[derive(Debug, Default)]
struct Crates {
  crates: Vec<Crate>,
  last: usize,
}

impl Crates {
  fn new() -> Self {
    Crates {
      crates: Vec::with_capacity(50),
      last: 0,
    }
  }

  fn top(&self) -> Crate {
    self.crates[self.last - 1]
  }

  fn free(&mut self, amount: usize) -> &mut [Crate] {
    if self.crates.len() < self.last + amount + 1 {
      let slice = vec![Crate::default(); amount + 1];
      self.crates.extend_from_slice(&slice);
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
  fn new(reverse: bool) -> Self {
    Self {
      stacks: Vec::with_capacity(25),
      reverse,
    }
  }

  fn add(&mut self, stack: usize, crate_: Crate) {
    if self.stacks.len() <= stack {
      self.stacks.push(RefCell::new(Crates::new()))
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

  fn top(&self) -> impl Iterator<Item = Crate> + '_ {
    self.stacks.iter().map(|crate_| crate_.borrow().top())
  }

  fn parse(reverse: bool, input: &str) -> Self {
    let mut stacks = Stacks::new(reverse);
    let mut lines = input.lines();

    let moves_regex = Regex::new("[a-z] ?").unwrap();
    let crates_regex = Regex::new(r"( \[|\[|]|] )").unwrap();

    let mut crates = lines
      .take_while_ref(|line| !line.starts_with(" 1"))
      .map(|line| crates_regex.replace_all(line, ""))
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

    let moves = lines
      .filter(|line| line.starts_with("move"))
      .map(|line| moves_regex.replace_all(line, ""))
      .map(|line| {
        line
          .split(' ')
          .map(parse_usize)
          .collect_tuple::<(_, _, _)>()
          .unwrap()
      });

    for (amount, a, b) in moves {
      stacks.move_crates(amount, a, b);
    }

    stacks
  }
}

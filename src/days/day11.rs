day!(11, Some(111210), Some(15447387620), |part, input| -> u64 {
  let part1 = || _solve::<20>(input, false);
  let part2 = || _solve::<10000>(input, true);

  answer!(part, part1, part2)
});

#[derive(Debug)]
struct Monkey {
  inspected: u64,
  items: Vec<u64>,
  operation: Operation,
  by: u64,
  if_true: usize,
  if_false: usize,
}

#[derive(Debug, FromStr, Copy, Clone)]
#[rustfmt::skip]
enum Operation {
  #[display("+ old")] PlusOld,
  #[display("+ {0}")] PlusNew(u64),
  #[display("* old")] TimesOld,
  #[display("* {0}")] TimesNew(u64),
}

impl FromStr for Monkey {
  type Err = anyhow::Error;

  fn from_str(str: &str) -> Result<Self, Self::Err> {
    let mut lines = str.lines().skip(1);

    Ok(Self {
      inspected: 0,
      items: lines
        .next()
        .unwrap()
        .index(18..)
        .split(", ")
        .map(parse_u64)
        .collect(),
      operation: lines.next().unwrap().index(23..).parse()?,
      by: lines.next().unwrap().index(21..).parse()?,
      if_true: lines.next().unwrap().index(29..).parse()?,
      if_false: lines.next().unwrap().index(30..).parse()?,
    })
  }
}

impl<'a> From<&'a str> for Monkey {
  fn from(str: &'a str) -> Self {
    str.parse::<Self>().unwrap()
  }
}

impl Monkey {
  fn inspect_items(&mut self, factor: Option<u64>) -> impl Iterator<Item = (usize, u64)> + '_ {
    let to_monkey = |n| {
      if n % self.by == 0 {
        self.if_true
      } else {
        self.if_false
      }
    };

    let calc = |value| match self.operation {
      Operation::PlusOld => value + value,
      Operation::PlusNew(n) => value + n,
      Operation::TimesOld => value * value,
      Operation::TimesNew(n) => value * n,
    };

    self.inspected += self.items.len() as u64;

    self
      .items
      .drain(..)
      .map(calc)
      .map(move |lvl| factor.map(|n| lvl % n).unwrap_or_else(|| lvl / 3))
      .map(move |lvl| (to_monkey(lvl), lvl))
  }
}

fn _solve<const ROUNDS: usize>(input: &str, by_product: bool) -> u64 {
  let monkeys = input
    .split("Monkey ")
    .filter(|s| !s.is_empty())
    .map(Monkey::from)
    .map(RefCell::new)
    .collect_vec();

  let factor = by_product.then(|| {
    monkeys
      .iter()
      .map(RefCell::borrow)
      .map(|m| m.by)
      .product::<u64>()
  });

  for _ in 0..ROUNDS {
    for mut monkey in monkeys.iter().map(RefCell::borrow_mut) {
      for (to, item) in monkey.inspect_items(factor) {
        monkeys[to].borrow_mut().items.push(item)
      }
    }
  }

  monkeys
    .iter()
    .map(RefCell::borrow)
    .map(|m| m.inspected)
    .sorted_unstable()
    .rev()
    .take(2)
    .product()
}

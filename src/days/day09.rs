#[rustfmt::skip]
day!(09, Some(6018), Some(2619), |part, input| -> usize {
  answer!(
    part,
    Rope::<2>::solve(input),
    Rope::<10>::solve(input)
  )
});

#[derive(Debug)]
struct Rope<const SIZE: usize> {
  visited: HashSet<(i32, i32)>,
  knots: [(i32, i32); SIZE],
}

impl<const SIZE: usize> Rope<SIZE> {
  fn iteration(mut self, (direction, steps): (&str, i32)) -> Self {
    for _ in 0..steps {
      match direction {
        "U" => self.knots[0].1 += 1,
        "D" => self.knots[0].1 -= 1,
        "L" => self.knots[0].0 -= 1,
        "R" => self.knots[0].0 += 1,
        _ => unreachable!(),
      }

      for i in 1..self.knots.len() {
        let target = self.knots[i - 1];
        let follow = &mut self.knots[i];

        if !((target.0 - follow.0).abs() <= 1 && (target.1 - follow.1).abs() <= 1) {
          follow.0 += (target.0 - follow.0).signum();
          follow.1 += (target.1 - follow.1).signum();
        }
      }

      self.visited.insert(self.knots[self.knots.len() - 1]);
    }

    self
  }

  fn solve(input: &str) -> usize {
    let rope = Self {
      visited: HashSet::with_capacity(10000),
      knots: [(0, 0); SIZE],
    };

    input
      .lines()
      .map(|line| (&line[..1], line[2..].parse::<i32>().unwrap()))
      .fold(rope, Rope::iteration)
      .visited
      .len()
  }
}

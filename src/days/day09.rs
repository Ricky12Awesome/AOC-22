#[rustfmt::skip]
day!(09, Some(6018), Some(2619), |part, input| -> usize {
  answer!(
    part,
    Rope::<2>::solve(input),
    Rope::<10>::solve(input)
  )
});

#[derive(Debug, FromStr, Copy, Clone)]
#[display("{0} {1}")]
struct Line(char, i32);

#[derive(Debug)]
struct Rope<const SIZE: usize> {
  visited: HashSet<(i32, i32)>,
  knots: [(i32, i32); SIZE],
}

impl<const SIZE: usize> Rope<SIZE> {
  fn iteration(mut self, Line(direction, steps): Line) -> Self {
    for _ in 0..steps {
      match direction {
        'U' => self.knots[0].1 += 1,
        'D' => self.knots[0].1 -= 1,
        'L' => self.knots[0].0 -= 1,
        'R' => self.knots[0].0 += 1,
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
      visited: HashSet::from_iter([(0, 0)].into_iter()),
      knots: [(0, 0); SIZE],
    };

    input
      .lines()
      .filter_map(|line| line.parse::<Line>().ok())
      .fold(rope, Rope::iteration)
      .visited
      .len()
  }
}

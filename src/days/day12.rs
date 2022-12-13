day!(12, Some(394), Some(388), |part, input| -> usize {
  answer!(part, Day::solve(input, false), Day::solve(input, true))
});

#[derive(Debug, Default)]
struct Day {
  map: HashMap<(i32, i32), i32>,
  start: (i32, i32),
  end: (i32, i32),
  anywhere: bool,
}

impl Day {
  fn new(anywhere: bool) -> Self {
    Self {
      anywhere,
      ..Self::default()
    }
  }

  fn fold_iteration(mut self, ((x, y), c): ((usize, usize), char)) -> Self {
    let (x, y) = (x as i32, y as i32);

    match c {
      'S' if self.anywhere => self.end = (x, y),
      'E' if self.anywhere => self.start = (x, y),
      'S' => self.start = (x, y),
      'E' => self.end = (x, y),
      _ => (),
    }

    let c = match c {
      'S' => 'a',
      'E' => 'z',
      c => c,
    };

    self.map.insert((x, y), c as i32 - 'a' as i32);
    self
  }

  fn solve(input: &str, anywhere: bool) -> usize {
    let day = input
      .lines()
      .enumerate()
      .map(|(y, line)| (y, line.chars().enumerate()))
      .flat_map(|(y, line)| line.map(move |(x, c)| ((x, y), c)))
      .fold(Self::new(anywhere), Self::fold_iteration);

    day.find_shortest()
  }

  fn check_end(&self, pos: (i32, i32)) -> bool {
    if self.anywhere {
      self.map[&pos] == 0
    } else {
      pos == self.end
    }
  }

  fn check_dst(&self, src: i32, dst: i32) -> bool {
    if self.anywhere {
      src == dst || dst == src + 1 || dst <= src
    } else {
      src == dst || src == dst + 1 || src <= dst
    }
  }

  fn find_shortest(&self) -> usize {
    pathfinding::directed::bfs::bfs(
      &self.start,
      |&(x, y)| {
        [(x, y + 1), (x, y - 1), (x - 1, y), (x + 1, y)]
          .into_iter()
          .filter_map(|p| Some((p, *self.map.get(&p)?)))
          .filter(|&(_, h)| self.check_dst(h, self.map[&(x, y)]))
          .map(|(p, _)| p)
          .collect_vec() // needs to be collected because stupid lifetimes
          .into_iter()
      },
      |&pos| self.check_end(pos),
    )
    .as_ref()
    .map(Vec::len)
    .map(|len| len - 1)
    .unwrap_or(0)
  }
}

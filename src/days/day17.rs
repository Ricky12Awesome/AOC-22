day!(17, None, None, |part, input| -> isize {
  answer!(part, part1(input), 0)
});

#[derive(Debug, Copy, Clone)]
enum Direction {
  Left,
  Right,
}

impl Direction {
  fn from(c: char) -> Self {
    match c {
      '<' => Self::Left,
      '>' => Self::Right,
      _ => unreachable!(),
    }
  }
}

type Pos = (isize, isize);

#[derive(Debug)]
struct Rock<'a> {
  id: char,
  positions: &'a [Pos],
}

impl<'a> Rock<'a> {
  const fn new(id: char, positions: &'a [Pos]) -> Self {
    Self { id, positions }
  }

  fn positions(&self, (offset_x, offset_y): Pos) -> impl Iterator<Item = Pos> + '_ {
    self
      .positions
      .iter()
      .map(move |&(x, y)| (x + offset_x, y + offset_y))
  }
}

#[derive(Default, Debug)]
struct Tower {
  placed: HashMap<Pos, char>,
  patterns: Vec<Direction>,
  pattern_index: usize,
  height: isize,
  floor_for_x: [isize; 7],
}

impl Tower {
  fn from(directions: impl Iterator<Item = Direction>) -> Self {
    Self {
      patterns: directions.collect_vec(),
      ..Self::default()
    }
  }

  fn collides_with_floor(&self, pos: &Pos) -> bool {
    self.placed.contains_key(pos) || pos.1 < self.floor_for_x[pos.0 as usize]
  }

  fn collides_with_sides(&self, pos: &Pos) -> bool {
    self.placed.contains_key(pos) || pos.0 >= 7 || pos.0 < 0
  }

  fn get_direction(&mut self) -> Direction {
    let value = self.patterns[self.pattern_index];
    self.pattern_index += 1;
    self.pattern_index %= self.patterns.len();
    value
  }

  fn can_move(&self, rock: &Rock, pos: Pos) -> bool {
    rock
      .positions(pos)
      .all(|pos| !self.collides_with_sides(&pos))
  }

  fn hit_floor(&self, rock: &Rock, pos: Pos) -> bool {
    rock
      .positions(pos)
      .any(|pos| self.collides_with_floor(&pos))
  }

  fn place(&mut self, rock: &Rock) {
    let spawn_x = 2;
    let spawn_y = self.height + 3;

    let mut x = spawn_x;
    let mut y = spawn_y;

    loop {
      let direction = self.get_direction();

      let offset_x = match direction {
        Direction::Left => -1,
        Direction::Right => 1,
      };

      if self.can_move(rock, (x + offset_x, y)) {
        x += offset_x;
      }

      if self.hit_floor(rock, (x, y - 1)) {
        break;
      }

      y -= 1;
    }

    for pos in rock.positions((x, y)) {
      self.placed.insert(pos, rock.id);
      self.floor_for_x[pos.0 as usize] = pos.1;

      if self.height <= pos.1 {
        self.height = pos.1 + 1;
      }
    }
  }
}

const ROCKS: [Rock; 5] = [
  Rock::new('-', &[(0, 0), (1, 0), (2, 0), (3, 0)]),
  Rock::new('+', &[(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)]),
  Rock::new('L', &[(2, 1), (2, 2), (0, 0), (1, 0), (2, 0)]),
  Rock::new('|', &[(0, 0), (0, 1), (0, 2), (0, 3)]),
  Rock::new('#', &[(0, 0), (0, 1), (1, 1), (1, 0)]),
];

fn part1(input: &str) -> isize {
  let patterns = input.chars().map(Direction::from);
  let mut tower = Tower::from(patterns);

  for i in 0..2022 {
    let rock = &ROCKS[i % ROCKS.len()];

    tower.place(rock);
  }

  // 3165 too high

  tower.height
}

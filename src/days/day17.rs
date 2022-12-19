#![allow(unused)]
use indexmap::IndexMap;
day!(17, Some(3114), None, |part, input| -> isize {
  answer!(part, part1(input), part2(input))
});

const LEFT: u8 = b'<';
const RIGHT: u8 = b'>';

type Pos = (isize, isize);

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Rock<'a> {
  id: u8,
  positions: &'a [Pos],
}

impl<'a> Rock<'a> {
  const fn new(id: u8, positions: &'a [Pos]) -> Self {
    Self { id, positions }
  }

  fn positions(&self, (offset_x, offset_y): Pos) -> impl Iterator<Item = Pos> + '_ {
    self
      .positions
      .iter()
      .map(move |&(x, y)| (x + offset_x, y + offset_y))
  }
}

#[derive(Debug)]
struct Tower<'a> {
  placed: [[u8; 7]; 512],
  patterns: &'a [u8],
  pattern_index: usize,
  height: isize,
  offset_y: isize,
}

impl<'a> Tower<'a> {
  fn from(patterns: &'a [u8]) -> Self {
    Self {
      placed: [[0; 7]; 512],
      patterns,
      pattern_index: 0,
      height: 0,
      offset_y: 0,
    }
  }

  fn get_y(&mut self, y: isize) -> usize {
    let y = y - self.offset_y;

    if y >= 500 {
      self.placed[..250].swap_with_slice(&mut [[0; 7]; 250]);

      let (left, right) = self.placed[..500].split_at_mut(250);

      left.swap_with_slice(right);

      self.offset_y += 250;
    }

    y as usize
  }

  fn set(&mut self, (x, y): Pos, id: u8) {
    self.placed[self.get_y(y)][x as usize] = id;
  }

  fn collides(&mut self, (x, y): Pos) -> bool {
    self.placed[self.get_y(y)][x as usize] != 0
  }

  fn collides_with_floor(&mut self, pos: Pos) -> bool {
    pos.1 < 0 || self.collides(pos)
  }

  fn collides_with_sides(&mut self, pos: Pos) -> bool {
    pos.0 >= 7 || pos.0 < 0 || self.collides(pos)
  }

  fn can_move(&mut self, rock: &Rock, pos: Pos) -> bool {
    rock
      .positions(pos)
      .all(|pos| !self.collides_with_sides(pos))
  }

  fn hit_floor(&mut self, rock: &Rock, pos: Pos) -> bool {
    rock.positions(pos).any(|pos| self.collides_with_floor(pos))
  }

  fn place(&mut self, rock: &Rock) {
    let spawn_x = 2;
    let spawn_y = self.height + 3;

    let mut x = spawn_x;
    let mut y = spawn_y;

    loop {
      let direction = self.patterns[self.pattern_index % self.patterns.len()];

      let offset_x = match direction {
        LEFT => -1,
        RIGHT => 1,
        _ => unreachable!(),
      };

      self.pattern_index += 1;

      if self.can_move(rock, (x + offset_x, y)) {
        x += offset_x;
      }

      if self.hit_floor(rock, (x, y - 1)) {
        break;
      }

      y -= 1;
    }

    for pos in rock.positions((x, y)) {
      self.set(pos, rock.id);

      if self.height <= pos.1 {
        self.height = pos.1 + 1;
      }
    }
  }
}

const ROCKS: [Rock; 5] = [
  Rock::new(b'=', &[(0, 0), (1, 0), (2, 0), (3, 0)]),
  Rock::new(b'+', &[(1, 0), (0, 1), (1, 1), (2, 1), (1, 2)]),
  Rock::new(b'L', &[(2, 1), (2, 2), (0, 0), (1, 0), (2, 0)]),
  Rock::new(b'|', &[(0, 0), (0, 1), (0, 2), (0, 3)]),
  Rock::new(b'#', &[(0, 0), (0, 1), (1, 1), (1, 0)]),
];

fn part1(input: &str) -> isize {
  let patterns = input.as_bytes();
  let mut tower = Tower::from(patterns);

  for i in 0..2022 {
    let rock = &ROCKS[i % ROCKS.len()];

    tower.place(rock);
  }

  tower.height
}

fn part2(input: &str) -> isize {
  // let patterns = input.as_bytes();
  // let mut tower = Tower::from(patterns);
  //
  // let mut snapshots = IndexMap::new();
  //
  // for i in 0..1000000000000 {
  //   let id = (tower.pattern_index % patterns.len(), i % ROCKS.len());
  //
  //   if let Some(&height) = snapshots.get(&id) {
  //     snapshots.remove(&id);
  //     let h = snapshots.values().sum::<isize>();
  //
  //     let i = i as isize;
  //     println!("{i}");
  //     println!("{}", tower.height);
  //     // 1514285714288
  //
  //     println!("1000000000000");
  //     println!("1514285714288");
  //     println!("{}",(1000000000000 / i) * height);
  //     println!("{}", 1000000000000 * height);
  //     break;
  //   } else {
  //     snapshots.insert(id, tower.height);
  //   }
  //
  //   tower.place(&ROCKS[i % ROCKS.len()]);
  //
  // }
  //
  // tower.height
  0
}

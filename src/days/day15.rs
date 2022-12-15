// x=3135800, y=2766584
#[rustfmt::skip]
day!(15, Some(5_607_466), Some(12_543_202_766_584), |part, input| -> usize {
  let input = parse(input);
  answer!(part, part1(&input, 2_000_000) - 1, part2(&input, 4_000_000))
});

#[derive(Debug, Display, FromStr)]
#[display("Sensor at x={sensor_x}, y={sensor_y}: closest beacon is at x={beacon_x}, y={beacon_y}")]
struct Line {
  sensor_x: isize,
  sensor_y: isize,
  beacon_x: isize,
  beacon_y: isize,
}

struct Sensor {
  pos: (isize, isize),
  closest: (isize, isize),
  distance: isize,
}

impl Sensor {
  fn new(line: Line) -> Self {
    Self {
      pos: (line.sensor_x, line.sensor_y),
      closest: (line.beacon_x, line.sensor_y),
      distance: (line.sensor_x - line.beacon_x).abs() + (line.sensor_y - line.beacon_y).abs(),
    }
  }

  fn within(&self, pos: (isize, isize)) -> bool {
    if self.closest == pos {
      return false;
    }

    (self.pos.0 - pos.0).abs() + (self.pos.1 - pos.1).abs() <= self.distance
  }
}

fn parse(input: &str) -> Vec<Sensor> {
  input
    .lines()
    .map(str::parse::<Line>)
    .map(Result::unwrap)
    .map(Sensor::new)
    .collect_vec()
}

fn part1(input: &[Sensor], n: isize) -> usize {
  let l_bound = input.iter().map(|s| s.pos.0 - s.distance).min().unwrap();
  let r_bound = input.iter().map(|s| s.pos.0 + s.distance).max().unwrap();

  (l_bound..=r_bound)
    .into_par_iter()
    .filter(|&x| input.iter().any(|s| s.within((x, n))))
    .count()
}

fn part2(input: &[Sensor], n: usize) -> usize {
  input
    .iter()
    .find_map(|s| {
      ((s.pos.0 - s.distance - 1).max(0) as _..(s.pos.0 as usize).min(n + 1))
        .into_par_iter()
        .zip(s.pos.1 as _..n + 1)
        .find_map_first(|p| {
          input
            .iter()
            .all(|s| !s.within((p.0 as _, p.1 as _)))
            .then_some(p.0 * 4_000_000 + p.1)
        })
    })
    .unwrap()
}

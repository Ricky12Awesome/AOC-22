// x=3135800, y=2766584
day!(15, Some(5607466), Some(12543202766584), |part, input| -> usize, i64 {
  answer!(part, part1(input), part2(input))
});

#[derive(Debug, Display, FromStr)]
#[display("Sensor at x={sensor_x}, y={sensor_y}: closest beacon is at x={beacon_x}, y={beacon_y}")]
struct Line {
  sensor_x: i64,
  sensor_y: i64,
  beacon_x: i64,
  beacon_y: i64,
}

struct Sensor {
  pos: (i64, i64),
  closest: (i64, i64),
  distance: i64,
}

impl Sensor {
  fn new(line: Line) -> Self {
    Self {
      pos: (line.sensor_x, line.sensor_y),
      closest: (line.beacon_x, line.sensor_y),
      distance: (line.sensor_x - line.beacon_x).abs() + (line.sensor_y - line.beacon_y).abs(),
    }
  }

  fn within(&self, pos: (i64, i64)) -> bool {
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

fn part1(input: &str) -> usize {
  let input = parse(input);

  let l_bound = input.iter().map(|s| s.pos.0 - s.distance).min().unwrap();
  let r_bound = input.iter().map(|s| s.pos.0 + s.distance).max().unwrap();

  (l_bound..=r_bound)
    .filter(|&x| input.iter().any(|s| s.within((x, 2000000))))
    .count()
}

fn part2(input: &str) -> i64 {
  let input = parse(input);

  input
    .iter()
    .find_map(|s| {
      ((s.pos.0 - s.distance - 1).max(0)..=s.pos.0.min(4000000))
        .zip(s.pos.1..=4000000)
        .find_map(|p| {
          input
            .iter()
            .all(|s| !s.within(p))
            .then_some(p.0 * 4000000 + p.1)
        })
    })
    .unwrap()
}

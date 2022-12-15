use arrayvec::ArrayVec;
use std::sync::atomic::{AtomicBool, AtomicI64, Ordering};
day!(15, Some(5607466), Some(12543202766584), |part, input| -> usize, i64 {
  answer!(part, part1(input), part2(input))
});

#[derive(Debug, Display, FromStr, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[display("x={x}, y={y}")]
struct Pos {
  x: i64,
  y: i64,
}

impl Pos {
  fn new(x: i64, y: i64) -> Self {
    Self { x, y }
  }

  fn radius(&self, other: &Self) -> i64 {
    (self.x - other.x).abs() + (self.y - other.y).abs()
  }

  fn within(&self, radius: i64, point: &Self) -> bool {
    self.radius(point) <= radius
  }
}

#[derive(Debug, Display, FromStr)]
#[display("Sensor at {sensor}: closest beacon is at {beacon}")]
struct Line {
  sensor: Pos,
  beacon: Pos,
}

fn parse(input: &str) -> Vec<(Pos, Pos, i64)> {
  input
    .lines()
    .map(str::parse::<Line>)
    .map(Result::unwrap)
    .map(|Line { sensor, beacon }| (sensor, beacon, sensor.radius(&beacon)))
    .collect_vec()
}

fn part1(input: &str) -> usize {
  let positions = parse(input);

  (-5_000_000..5_000_000)
    .map(|x| Pos::new(x, 2000000))
    .filter(|current| {
      positions
        .iter()
        .filter(|(_, beacon, _)| beacon != current)
        .any(|(pos, _, radius)| current.within(*radius, pos))
    })
    .count()
}

fn part2(input: &str) -> i64 {
  let positions = parse(input);
  let finished = AtomicBool::new(false);
  let value = AtomicI64::new(0);
  let positions = ArrayVec::<_, 50>::from_iter(positions);

  std::thread::scope(|s| {
    let size = 4_000_000;
    // let size = 20;

    let thread_count = 16;
    let size_per_thread = size / thread_count;
    let mut threads = Vec::new();

    println!("{size_per_thread}");

    for i in 1..=thread_count {
      let previous = size_per_thread * (i - 1);
      let current = size_per_thread * i;

      let (positions, finished, value) = (&positions, &finished, &value);

      threads.push(s.spawn(move || {
        let area =
          (previous..=current).flat_map(|y| (0..=size).map(move |x| Pos::new(x, y)));

        for current in area {
          if finished.load(Ordering::SeqCst) {
            break
          }

          let check = positions
            .iter()
            .any(|(pos, _, radius)| current.within(*radius, pos));

          if !check {
            println!("{current}");
            finished.store(true, Ordering::SeqCst);
            value.store((current.x * 4_000_000) + current.y, Ordering::SeqCst);
            break
          }
        }
      }));
    }

    for thread in threads {
      thread.join().unwrap();
    }

    value.load(Ordering::Acquire)
  })
}

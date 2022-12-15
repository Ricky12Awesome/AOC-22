day!(14, Some(793), Some(24166), |part, input| -> u64 {
  answer!(part, _solve(input, false), 1 + _solve(input, true))
});

fn iteration_positions(
  (x1, y1): (usize, usize),
  (x2, y2): (usize, usize),
  f: impl FnMut((usize, usize)),
) {
  let min_x = x1.min(x2);
  let min_y = y1.min(y2);
  let delta_x = x1.max(x2) - min_x;
  let delta_y = y1.max(y2) - min_y;

  match () {
    _ if x1 == x2 => (0..=delta_y).map(|y| (x1, min_y + y)).for_each(f),
    _ if y1 == y2 => (0..=delta_x).map(|x| (min_x + x, y1)).for_each(f),
    _ => unreachable!(),
  }
}

#[derive(Debug, FromStr, Display, Copy, Clone)]
enum Point {
  #[display("#")]
  Rock,
  #[display("o")]
  Sand,
  #[display(".")]
  Air,
}

type Grid = Vec<Vec<Point>>;

fn new_location((x, y): (usize, usize), grid: &Grid) -> Option<(usize, usize)> {
  let bottom = *grid.get(y + 1)?.get(x)?;
  let bottom_left = *grid.get(y + 1)?.get(x.checked_sub(1)?)?;
  let bottom_right = *grid.get(y + 1)?.get(x + 1)?;

  Some(match (bottom, bottom_left, bottom_right) {
    (Point::Air, _, _) => (x, y + 1),
    (Point::Rock | Point::Sand, Point::Air, _) => (x - 1, y + 1),
    (Point::Rock | Point::Sand, _, Point::Air) => (x + 1, y + 1),
    _ => (x, y),
  })
}

fn drop_sand(grid: &mut Grid, floor: bool) -> Option<()> {
  let mut location = (500, 0);

  loop {
    let new_location = new_location(location, grid)?;

    if location == new_location {
      break;
    }

    location = new_location;
  }

  if floor && location == (500, 0) {
    return None;
  }

  grid[location.1][location.0] = Point::Sand;

  Some(())
}

fn _solve(input: &str, floor: bool) -> u64 {
  let rocks = input.lines().map(|line| {
    line
      .split(" -> ")
      .map(|pair| pair.split_once(',').unwrap())
      .map(|(x, y)| (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap()))
  });

  let mut grid: Grid = vec![vec![Point::Air; 2000]; 200];
  let mut highest = 0;

  for points in rocks {
    points.tuple_windows::<(_, _)>().for_each(|(a, b)| {
      iteration_positions(a, b, |(x, y)| {
        if y > highest {
          highest = y;
        }

        grid[y][x] = Point::Rock
      });
    });
  }

  if floor {
    grid[highest + 2] = vec![Point::Rock; 2000];
  }

  let mut counter = 0;

  while drop_sand(&mut grid, floor).is_some() {
    counter += 1;
  }

  counter
}

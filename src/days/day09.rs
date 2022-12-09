use std::collections::HashSet;
// day!(Day08, Some(0), Some(0));
day!(Day09);

impl Day09 {
  pub fn day(part: Part) -> Answer<u32> {
    let input = Self::INPUT
      .lines()
      .map(|line| (&line[..1], line[2..].parse::<isize>().unwrap()));

    let mut visited = HashSet::new();
    let mut head = (0isize, 0isize);
    let mut tail = (0isize, 0isize);
    let mut previous = "R";

    visited.insert(tail);

    for (direction, distance) in input {
      for _ in 0..distance {
        head = match direction {
          "R" => (head.0 + 1, head.1),
          "L" => (head.0 - 1, head.1),
          "U" => (head.0, head.1 + 1),
          "D" => (head.0, head.1 - 1),
          _ => unreachable!(),
        };

        let x_diff = (head.0 - tail.0).abs();
        let y_diff = (head.1 - tail.1).abs();

        if x_diff <= 1 && y_diff <= 1 {
          // If the head and tail are touching, the tail doesn't move
          continue;
        }

        if x_diff <= 2 && y_diff == 0 {
          // If the head and tail are in the same row, the tail moves horizontally
          tail.0 += (head.0 - tail.0) / x_diff;
        } else if y_diff <= 2 && x_diff == 0 {
          // If the head and tail are in the same column, the tail moves vertically
          tail.1 += (head.1 - tail.1) / y_diff;
        } else {
          // Otherwise, the tail moves diagonally to keep up with the head
          tail.0 += (head.0 - tail.0) / x_diff;
          tail.1 += (head.1 - tail.1) / y_diff;
        }

        visited.insert(tail);
      }

      previous = direction;
    }

    println!("{}", visited.len());

    let part1 = || 0;
    let part2 = || 0;

    answer!(part, part1, part2)
  }
}

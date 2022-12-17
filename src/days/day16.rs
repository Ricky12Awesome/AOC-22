#![allow(unused)]

use indexmap::IndexMap;

day!(16, None, None, |part, input| -> usize {
  answer!(part, part1(input), 0)
});

type Valves<'a> = IndexMap<&'a str, Valve<'a>>;

#[derive(Debug)]
struct Valve<'a> {
  id: &'a str,
  rate: usize,
  opened: bool,
  leads: Vec<&'a str>,
}

impl<'a> Valve<'a> {
  fn from(line: &'a str) -> Self {
    let semi_colon_index = line[23..].find(';').unwrap();
    let s_count = line[32..].chars().filter(|&c| c == 's').count();

    Self {
      id: &line[6..][..2],
      rate: line[23..][..semi_colon_index].parse().unwrap(),
      opened: false,
      leads: line[47 + s_count..].trim().split(", ").collect_vec(),
    }
  }
}

fn part1(input: &str) -> usize {
  use pathfinding::directed::strongly_connected_components::*;

  let mut valves = input
    .lines()
    .map(Valve::from)
    .map(|v| (v.id, v))
    .collect::<Valves>();

  let start = &valves["AA"].leads;
  0
}

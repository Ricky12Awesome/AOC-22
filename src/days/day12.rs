day!(12, None, None, |part, input| -> u64 {
  _solve(input);

  answer!(part, || 0, || 0)
});

fn _solve(input: &str) {
  let height_map = input.lines().map(|line| line.chars());
  let mut map = HashMap::new();
  let mut starting = (0, 0);
  let mut end = (0, 0);

  for (y, row) in height_map.enumerate() {
    for (x, height) in row.enumerate() {
      let (x, y) = (x as i32, y as i32);

      let value = match height {
        'S' => {
          starting = (x, y);
          b'a'
        }
        'E' => {
          end = (x, y);
          b'z'
        }
        _ => height as u8,
      };

      map.insert((x, y), (value - b'a') as i64);
    }
  }

  let result = pathfinding::directed::bfs::bfs(
    &starting,
    |&(x, y)| {
      [(x, y + 1), (x, y - 1), (x - 1, y), (x + 1, y)]
        .into_iter()
        .filter_map(|p| Some((p, *map.get(&p)?)))
        .filter(|&(_, h)| h == map[&(x, y)] || h == map[&(x, y)] + 1 || h <= map[&(x, y)])
        .map(|(p, _)| p)
        .collect_vec() // needs to be collected because stupid lifetimes
        .into_iter()
    },
    |&(x, y)| (x, y) == end,
  );

  dbg!(&starting);
  dbg!(&end);
  dbg!(&result);

  println!("{}", result.as_ref().map(Vec::<_>::len).unwrap_or(0));
}

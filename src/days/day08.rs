day!(08, Some(1647), Some(392080), |part, input| -> usize {
  let grid = input
    .lines()
    .map(|it| it.chars().map(char_to_u8).collect_vec())
    .collect_vec();

  let (p1, p2) = visible_trees(&grid);

  answer!(part, p1, p2)
});

fn visible_trees(grid: &Vec<Vec<u8>>) -> (usize, usize) {
  let mut visible_tree_count = 0;
  let mut highest_score = 0;

  for y in 0..grid.len() {
    for x in 0..grid[y].len() {
      let value = grid[y][x];

      let trees = [
        (0..y).all(|y| grid[y][x] < value),
        (0..x).all(|x| grid[y][x] < value),
        (y + 1..grid.len()).all(|y| grid[y][x] < value),
        (x + 1..grid[y].len()).all(|x| grid[y][x] < value),
      ];

      let scores = [
        (0..y).rev().count_inclusive(|y| grid[y][x] < value),
        (0..x).rev().count_inclusive(|x| grid[y][x] < value),
        (y + 1..grid.len()).count_inclusive(|y| grid[y][x] < value),
        (x + 1..grid[y].len()).count_inclusive(|x| grid[y][x] < value),
      ];

      let score = scores.into_iter().product();

      if score > highest_score {
        highest_score = score
      }

      if trees.into_iter().any(|is_visible| is_visible) {
        visible_tree_count += 1;
      }
    }
  }

  (visible_tree_count, highest_score)
}

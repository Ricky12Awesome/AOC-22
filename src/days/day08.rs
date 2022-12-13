day!(08, Some(1647), Some(392080), |part, input| -> usize {
  let grid = input
    .lines()
    .map(|it| it.chars().map(char_to_u8).collect_vec())
    .collect_vec();

  let (p1, p2) = visible_trees(&grid);

  answer!(part, p1, p2)
});

#[allow(clippy::needless_range_loop)]
fn visible_trees(grid: &Vec<Vec<u8>>) -> (usize, usize) {
  let mut visible_tree_count = 0;
  let mut highest_score = 0;

  for y in 0..grid.len() {
    for x in 0..grid[y].len() {
      let value = grid[y][x];

      let is_visible_from_top = (0..y).all(|y| grid[y][x] < value);
      let is_visible_from_left = (0..x).all(|x| grid[y][x] < value);
      let is_visible_from_bottom = (y + 1..grid.len()).all(|y| grid[y][x] < value);
      let is_visible_from_right = (x + 1..grid[y].len()).all(|x| grid[y][x] < value);

      let [mut top, mut left, mut bottom, mut right] = [0usize; 4];

      for y in (0..y).rev() {
        top += 1;

        if grid[y][x] >= value {
          break;
        }
      }

      for x in (0..x).rev() {
        left += 1;

        if grid[y][x] >= value {
          break;
        }
      }

      for y in y + 1..grid.len() {
        bottom += 1;

        if grid[y][x] >= value {
          break;
        }
      }

      for x in x + 1..grid[y].len() {
        right += 1;

        if grid[y][x] >= value {
          break;
        }
      }

      let score = top * left * bottom * right;

      if score > highest_score {
        highest_score = score
      }

      if is_visible_from_top
        || is_visible_from_left
        || is_visible_from_bottom
        || is_visible_from_right
      {
        visible_tree_count += 1;
      }
    }
  }

  (visible_tree_count, highest_score)
}

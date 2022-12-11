// day!(Day10, Some(0), Some(0));
day!(Day10);

#[derive(Debug, FromStr, Copy, Clone)]
#[rustfmt::skip]
enum Instruction {
  #[display("noop")] Noop,
  #[display("addx {0}")] Add(i32),
}

impl Day10 {
  pub fn day(part: Part) -> Answer<i32> {
    let instructions = Self::INPUT
      .lines()
      .map(|line| line.parse::<Instruction>().unwrap());

    let mut cycle = 0i32;
    let mut x = 1i32;
    let mut strength = Vec::with_capacity(10);
    let mut crt = String::with_capacity(42 * 7);

    for instruction in instructions {
      for c in 0..2 {
        let pos = cycle % 40;

        if x.abs_diff(pos) <= 1 {
          crt.push('█');
        } else {
          crt.push(' ');
        }

        if pos == 39 {
          crt.push('\n');
        }

        cycle += 1;

        if matches!(cycle, 20 | 60 | 100 | 140 | 180 | 220) {
          strength.push(cycle * x);
        }

        match instruction {
          Instruction::Noop => break,
          Instruction::Add(v) if c == 1 => x += v,
          _ => (),
        }
      }
    }

    println!("{crt}");

    let part1 = || strength.iter().sum::<i32>();
    let part2 = || 0;

    answer!(part, part1, part2)
  }
}

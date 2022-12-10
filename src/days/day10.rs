use parse_display::FromStr;
// day!(Day10, Some(0), Some(0));
day!(Day10);

#[derive(Debug, FromStr, Copy, Clone)]
enum Instruction {
  #[display("noop")]
  Noop,
  #[display("addx {0}")]
  Add(i32),
}

impl Day10 {
  pub fn day(part: Part) -> Answer<i32> {
    let instructions = Self::INPUT
      .lines()
      .map(|line| line.parse::<Instruction>().unwrap());

    let mut cycle = 0i32;
    let mut x = 1i32;
    let mut strength = Vec::new();
    let mut crt = [[' '; 40]; 6];

    for instruction in instructions {
      let amount = match instruction {
        Instruction::Noop => 1,
        Instruction::Add(_) => 2,
      };

      for c in 0..amount {
        let y = cycle / 40;
        let xx = cycle % 40;

        if x.abs_diff(xx) <= 1 {
          crt[y as usize][xx as usize] = '█';
        }

        cycle += 1;

        if matches!(cycle, 20 | 60 | 100 | 140 | 180 | 220) {
          strength.push(cycle * x);
        }

        if let Instruction::Add(v) = instruction {
          if c == 1 {
            x += v;
          }
        }
      }
    }

    for y in crt {
      for x in y {
        print!("{x}");
      }

      println!()
    }

    let part1 = || strength.iter().sum::<i32>();
    let part2 = || 0;

    answer!(part, part1, part2)
  }
}

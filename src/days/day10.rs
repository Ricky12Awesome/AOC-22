const _TEST_P2: &str = "\
\n████ ████ ███  ███  ███  ████ ████ ████ \
\n█       █ █  █ █  █ █  █ █       █ █    \
\n███    █  ███  █  █ ███  ███    █  ███  \
\n█     █   █  █ ███  █  █ █     █   █    \
\n█    █    █  █ █    █  █ █    █    █    \
\n█    ████ ███  █    ███  █    ████ █    ";

day!(10, Some(14720), Some(String::from(_TEST_P2)), |part, input| -> i32, String {
      let instructions = input
      .lines()
      .map(|line| line.parse::<Instruction>().unwrap());

    let mut cycle = 0i32;
    let mut x = 1i32;
    let mut strength = Vec::with_capacity(10);
    let mut crt = String::with_capacity(42 * 7);

    for instruction in instructions {
      for c in 0..2 {
        let pos = cycle % 40;

        if pos == 0 {
          crt.push('\n');
        }

        if x.abs_diff(pos) <= 1 {
          crt.push('█');
        } else {
          crt.push(' ');
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

    answer!(part, || strength.iter().sum::<i32>(), || crt)
});

#[derive(Debug, FromStr, Copy, Clone)]
#[rustfmt::skip]
enum Instruction {
  #[display("noop")] Noop,
  #[display("addx {0}")] Add(i32),
}

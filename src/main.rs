use clap::Parser;
use advent_of_code_2022::day::DayArgs;

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct AnyDayArgs {
  #[clap(long, short)]
  day: usize,

  #[clap(flatten)]
  args: DayArgs,
}

macro_rules! for_day {
  ($day:ident, $args:expr) => {{
    let $day = advent_of_code_2022::days::$day::solve(
      $args.args.part.unwrap_or_default(),
      $args
        .args
        .input
        .as_ref()
        .map(String::as_str)
        .unwrap_or(advent_of_code_2022::days::$day::INPUT),
    );

    (
      $day.0.map(|answer| format!("{answer}")),
      $day.1.map(|answer| format!("{answer}")),
    )
  }};
}

fn main() -> anyhow::Result<()> {
  let args = AnyDayArgs::parse();

  let (part1, part2) = match args.day {
    1 => for_day!(day01, args),
    2 => for_day!(day02, args),
    3 => for_day!(day03, args),
    4 => for_day!(day04, args),
    5 => for_day!(day05, args),
    6 => for_day!(day06, args),
    7 => for_day!(day07, args),
    8 => for_day!(day08, args),
    9 => for_day!(day09, args),
    10 => for_day!(day10, args),
    11 => for_day!(day11, args),
    12 => for_day!(day12, args),
    13 => for_day!(day13, args),
    14 => for_day!(day14, args),
    15 => for_day!(day15, args),
    16 => for_day!(day16, args),
    17 => for_day!(day17, args),
    18 => for_day!(day18, args),
    19 => for_day!(day19, args),
    20 => for_day!(day20, args),
    21 => for_day!(day21, args),
    22 => for_day!(day22, args),
    23 => for_day!(day23, args),
    24 => for_day!(day24, args),
    25 => for_day!(day25, args),
    _ => anyhow::bail!("not a valid day!"),
  };

  if let Some(answer) = part1 {
    println!("Day {} Part 1: {answer}", args.day)
  }

  if let Some(answer) = part2 {
    println!("Day {} Part 2: {answer}", args.day)
  }

  Ok(())
}

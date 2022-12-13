use advent_of_code_2022::day::DayArgs;
use clap::Parser;
use std::time::Duration;

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
    let part = $args.args.part.unwrap_or_default();
    let input = $args
      .args
      .input
      .unwrap_or(advent_of_code_2022::days::$day::INPUT.to_string());

    if $args.args.benchmark {
      let time = $args.args.time.unwrap_or(5000);
      let time = Duration::from_millis(time);

      if !$args.args.warmup {
        benchmarking::warm_up();
      }

      let result = benchmarking::bench_function_with_duration(time, move |measure| {
        measure.measure(|| advent_of_code_2022::days::$day::solve(part, &input));
      })
      .unwrap();

      println!(
        "took {:.2?}/iter [{:.0} iter/second]",
        result.elapsed(),
        result.speed()
      );

      return Ok(());
    }

    let (part1, part2) = advent_of_code_2022::days::$day::solve(part, &input);

    if let Some(answer) = part1 {
      println!("Day {} Part 1: {answer}", $args.day);
    }

    if let Some(answer) = part2 {
      println!("Day {} Part 2: {answer}", $args.day);
    }
  }};
}

fn main() -> anyhow::Result<()> {
  let args = AnyDayArgs::parse();

  match args.day {
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

  Ok(())
}

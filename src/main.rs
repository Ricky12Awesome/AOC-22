use advent_of_code_2022::day::DayArgs;
use clap::Parser;
use std::time::Duration;

#[derive(Debug, Parser)]
#[command(author, version, about)]
struct AnyDayArgs {
  #[clap(long, short, default_value = "1")]
  day: usize,

  #[clap(long)]
  all: bool,

  #[clap(flatten)]
  args: DayArgs,
}

macro_rules! for_day {
  ($day:ident, $n:expr, $args:expr) => {{
    let part = $args.args.part.unwrap_or_default();
    // let input = $args
    //   .args
    //   .input
    //   .unwrap_or(advent_of_code_2022::days::$day::INPUT.to_string());

    let input = advent_of_code_2022::days::$day::INPUT;

    if $args.args.benchmark {
      let time = $args.args.time.unwrap_or(5000);
      let time = Duration::from_millis(time);

      let result = benchmarking::bench_function_with_duration(time, move |measure| {
        measure.measure(|| advent_of_code_2022::days::$day::solve(part, &input));
      })
      .unwrap();

      println!(
        "[Day {}] took {:.2?}/iter [{:.0} iter/second]",
        $n,
        result.elapsed(),
        result.speed()
      );

      return Ok(());
    }

    let (part1, part2) = advent_of_code_2022::days::$day::solve(part, &input);

    if let Some(answer) = part1 {
      println!("Day {} Part 1: {answer}", $n);
    }

    if let Some(answer) = part2 {
      println!("Day {} Part 2: {answer}", $n);
    }
  }};
}

fn run_day(day: usize, args: &AnyDayArgs) -> anyhow::Result<()> {
  match day {
    1 => for_day!(day01, day, args),
    2 => for_day!(day02, day, args),
    3 => for_day!(day03, day, args),
    4 => for_day!(day04, day, args),
    5 => for_day!(day05, day, args),
    6 => for_day!(day06, day, args),
    7 => for_day!(day07, day, args),
    8 => for_day!(day08, day, args),
    9 => for_day!(day09, day, args),
    10 => for_day!(day10, day, args),
    11 => for_day!(day11, day, args),
    12 => for_day!(day12, day, args),
    13 => for_day!(day13, day, args),
    14 => for_day!(day14, day, args),
    15 => for_day!(day15, day, args),
    16 => for_day!(day16, day, args),
    17 => for_day!(day17, day, args),
    18 => for_day!(day18, day, args),
    19 => for_day!(day19, day, args),
    20 => for_day!(day20, day, args),
    21 => for_day!(day21, day, args),
    22 => for_day!(day22, day, args),
    23 => for_day!(day23, day, args),
    24 => for_day!(day24, day, args),
    25 => for_day!(day25, day, args),
    _ => anyhow::bail!("not a valid day!"),
  };

  Ok(())
}

fn main() -> anyhow::Result<()> {
  let args = AnyDayArgs::parse();

  if !args.args.warmup && args.args.benchmark {
    benchmarking::warm_up();
  }

  if args.all {
    for day in 1..=25 {
      run_day(day, &args)?;
    }
  } else {
    run_day(args.day, &args)?;
  }

  Ok(())
}

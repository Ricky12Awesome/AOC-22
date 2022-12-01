#[macro_export]
macro_rules! answer {
  ($answer:expr) => {
    Some($answer)
  };
  ($part:expr, $p1:expr, $p2:expr) => {
    match $part {
      Part::One => [answer!($p1()), None],
      Part::Two => [None, answer!($p2())],
      Part::Both => [answer!($p1()), answer!($p2())],
    }
  };
}

#[macro_export]
macro_rules! input_path {
  ($name:ident) => {
    concat!("../../input/", stringify!($name), ".txt")
  };
}

#[macro_export]
macro_rules! day {
  ($name:ident) => {
    day!($name, None, None);
  };
  ($name:ident, $p1:expr, $p2:expr) => {
    use $crate::prelude::*;

    pub struct $name;

    impl Input<&'static str> for $name {
      const INPUT: &'static str = include_str!(input_path!($name));
    }

    generate_tests_for_day!($name, $p1, $p2);
  };
}

#[macro_export]
macro_rules! generate_tests_for_day {
  ($day:ident, $p1:expr, $p2:expr) => {
    #[test]
    pub fn both() {
      let [part1, part2] = $day::day(Part::Both);
      let p1 = $p1;
      let p2 = $p2;
      let day = stringify!($day).trim_start_matches("Day").parse::<u32>().unwrap();

      println!("Day {day} Part 1: {:?}", part1);

      if let Some(_) = p1 {
        assert_eq!(part1, p1);
      }

      println!("Day {day} Part 2: {:?}", part2);

      if let Some(_) = p2 {
        assert_eq!(part2, p2);
      }
    }

    #[test]
    pub fn part1() {
      let [part1, _] = $day::day(Part::One);
      let day = stringify!($day).trim_start_matches("Day").parse::<u32>().unwrap();
      let p1 = $p1;

      if let Some(_) = p1 {
        assert_eq!(part1, p1);
      }

      println!("Day {day} Part 1: {:?}", part1);
    }

    #[test]
    pub fn part2() {
      let [_, part2] = $day::day(Part::Two);
      let day = stringify!($day).trim_start_matches("Day").parse::<u32>().unwrap();
      let p2 = $p2;

      println!("Day {day} Part 2: {:?}", part2);

      if let Some(_) = p2 {
        assert_eq!(part2, p2);
      }
    }
  };
}

pub enum Part {
  One,
  Two,
  Both,
}

pub type Answer<T> = [Option<T>; 2];

pub trait Input<T: Sized + 'static> {
  const INPUT: T;
}

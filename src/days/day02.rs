day!(02, Some(13924), Some(209603), |part, input| -> u32 {
  let input = input
    .lines()
    .map(|line| line.split_once(' ').unwrap())
    .collect_vec();

  answer!(
    part,
    || input
      .iter()
      .copied()
      .map(Play::from)
      .map(Play::sum)
      .sum::<u32>(),
    || input
      .iter()
      .copied()
      .map(Play::from_p2)
      .map(Play::sum)
      .sum::<u32>()
  )
});

#[repr(u32)]
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
enum Play {
  Rock = 1,
  Paper = 2,
  Scissors = 3,
}

impl Play {
  fn from_str(str: &str) -> Self {
    match str {
      "A" | "X" => Self::Rock,
      "B" | "Y" => Self::Paper,
      "C" | "Z" => Self::Scissors,
      _ => unreachable!(),
    }
  }

  fn from((play, response): (&str, &str)) -> (Self, Self) {
    (Self::from_str(play), Self::from_str(response))
  }

  fn from_p2((play, response): (&str, &str)) -> (Self, Self) {
    let play = Self::from_str(play);

    match response {
      "X" => (
        play,
        match play {
          Play::Rock => Self::Scissors,
          Play::Paper => Self::Rock,
          Play::Scissors => Self::Paper,
        },
      ),
      "Y" => (play, play),
      "Z" => (
        play,
        match play {
          Play::Rock => Self::Paper,
          Play::Paper => Self::Scissors,
          Play::Scissors => Self::Rock,
        },
      ),
      _ => unreachable!(),
    }
  }

  fn sum((play, response): (Self, Self)) -> u32 {
    let n = match (response, play) {
      (Play::Rock, Play::Scissors) => 6,
      (Play::Paper, Play::Rock) => 6,
      (Play::Scissors, Play::Paper) => 6,
      (a, b) if a == b => 3,
      _ => 0,
    };

    response as u32 + n
  }
}

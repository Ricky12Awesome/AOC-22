use std::cmp::Ordering;

day!(13, Some(5938), Some(29025), |part, input| -> usize {
  answer!(part, part1(input), part2(input))
});

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
#[serde(untagged)]
enum Value {
  Value(i32),
  List(Vec<Value>),
}

impl Value {
  fn list(self) -> Vec<Value> {
    match self {
      Value::List(list) => list,
      _ => panic!("Value must be a list"),
    }
  }

  fn compare_n(left: i32, right: i32) -> Ordering {
    left.cmp(&right)
  }

  fn compare_list(left: &[Self], right: &[Self]) -> Ordering {
    for i in 0..left.len().max(right.len()) {
      let Some(left) = left.get(i) else {
        return Ordering::Less
      };

      let Some(right) = right.get(i) else {
        return Ordering::Greater
      };

      if !left.compare(right).is_eq() {
        return left.compare(right);
      }
    }

    Ordering::Equal
  }

  fn compare(&self, other: &Self) -> Ordering {
    match (self, other) {
      (&Self::Value(left), &Self::Value(right)) => Self::compare_n(left, right),
      (Self::List(left), Self::List(right)) => Self::compare_list(left, right),
      (&Self::Value(left), Self::List(right)) => Self::compare_list(&[Self::Value(left)], right),
      (Self::List(left), &Self::Value(right)) => Self::compare_list(left, &[Self::Value(right)]),
    }
  }

  fn compare_arr([left, right]: [Self; 2]) -> Ordering {
    Self::compare(&left, &right)
  }
}

fn part1(input: &str) -> usize {
  input
    .lines()
    .filter(|line| !line.is_empty())
    .map(serde_json::from_str::<Value>)
    .map(Result::<_, _>::unwrap)
    .array_chunks::<2>()
    .map(Value::compare_arr)
    .enumerate()
    .filter(|(_, b)| b.is_lt())
    .map(|(i, _)| i + 1)
    .sum::<usize>()
}

fn part2(input: &str) -> usize {
  let first = serde_json::from_str::<Value>("[[2]]").unwrap();
  let second = serde_json::from_str::<Value>("[[6]]").unwrap();

  let mut list = input
    .lines()
    .filter(|line| !line.is_empty())
    .map(serde_json::from_str::<Value>)
    .map(Result::<_, _>::unwrap)
    .interleave([first.clone(), second.clone()])
    .sorted_by(Value::compare)
    .inspect(|val| println!("{val:?}"));

  let (first, _) = list.find_position(|it| *it == first).unwrap();
  let (second, _) = list.find_position(|it| *it == second).unwrap();

  (first + 1) * (first + second + 2)
}

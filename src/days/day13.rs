day!(13, None, None, |part, input| -> u64 {
  answer!(part, _solve(input), 0)
});

#[derive(Serialize, Deserialize)]
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

  fn compare_n(left: i32, right: i32) -> Option<bool> {
    if left == right {
      None
    } else {
      Some(left < right)
    }
  }

  fn compare_list(left: &[Self], right: &[Self]) -> Option<bool> {
    for i in 0..left.len().max(right.len()) {
      let Some(left) = left.get(i) else {
        return Some(true)
      };

      let Some(right) = right.get(i) else {
        return Some(false)
      };

      if let Some(val) = left.compare(right) {
        return Some(val);
      }
    }

    None
  }

  fn compare(&self, other: &Self) -> Option<bool> {
    match (self, other) {
      (&Self::Value(left), &Self::Value(right)) => Self::compare_n(left, right),
      (Self::List(left), Self::List(right)) => Self::compare_list(left, right),
      (&Self::Value(left), Self::List(right)) => Self::compare_list(&[Self::Value(left)], right),
      (Self::List(left), &Self::Value(right)) => Self::compare_list(left, &[Self::Value(right)]),
    }
  }

  fn compare_arr([left, right]: [Self; 2]) -> bool {
    Self::compare(&left, &right).unwrap()
  }
}

fn _solve(input: &str) -> u64 {
  input
    .lines()
    .filter(|line| !line.is_empty())
    .map(serde_json::from_str::<Value>)
    .map(Result::<_, _>::unwrap)
    .array_chunks::<2>()
    .map(Value::compare_arr)
    .enumerate()
    .filter(|(_, b)| *b)
    .map(|(i, _)| i as u64 + 1)
    // .map(|it| dbg!(it))
    .sum::<u64>()
}

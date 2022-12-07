#![allow(unused)]

use std::{
  cell::RefCell,
  collections::{hash_map::Entry, HashMap},
};

day!(Day07, Some(1749646), Some(1498966));
// day!(Day07);

#[derive(Debug, Clone)]
enum Path<'a> {
  Directory(&'a str, HashMap<&'a str, Path<'a>>),
  File(&'a str, usize),
}

impl<'a> Path<'a> {
  fn dir(name: &'a str) -> Self {
    Self::Directory(name, HashMap::new())
  }

  fn insert(&mut self, path: &[&'a str], file: &'a str, size: usize) {
    let mut iter = path.iter();
    let mut entry = self;

    for name in iter {
      if let Self::Directory(_, contents) = entry {
        entry = contents.entry(name).or_insert_with(|| Path::dir(name))
      }
    }

    if let Self::Directory(_, contents) = entry {
      contents.insert(file, Self::File(file, size));
    }
  }

  fn contents(self) -> HashMap<&'a str, Path<'a>> {
    match self {
      Path::Directory(_, contents) => contents,
      Path::File(_, _) => unreachable!(),
    }
  }

  fn size(&self) -> usize {
    match self {
      Self::Directory(_, contents) => contents.values().map(Self::size).sum::<usize>(),
      Self::File(_, size) => *size,
    }
  }
}

impl Day07 {
  pub fn day(part: Part) -> Answer<usize> {
    let mut root = RefCell::new(Path::dir("/"));
    let mut tree = vec![];

    for line in Self::INPUT.lines().skip(1) {
      match () {
        _ if line.starts_with("$ cd ") => match &line[5..] {
          ".." => {
            tree.pop().unwrap();
          }
          name => {
            tree.push(name);
          }
        },
        _ if line.starts_with("$ ls") || line.starts_with("dir ") => {}
        _ => {
          let (size, name) = line.split_once(' ').unwrap();
          let size = size.parse::<usize>().unwrap();

          let mut root = root.borrow_mut();

          root.insert(&tree, name, size);
        }
      }
    }

    let total_size = root.borrow().size();
    let limit = 30_000_000 - (70_000_000 - total_size);

    let mut dirs = Vec::<usize>::new();
    let mut root = root.into_inner().contents();

    fn iterate(dirs: &mut Vec<usize>, contents: &HashMap<&str, Path>) {
      for path in contents.values() {
        if let Path::Directory(name, contents) = path {
          let size = path.size();
          dirs.push(size);

          iterate(dirs, contents);
        }
      }
    };

    iterate(&mut dirs, &root);

    let part1 = || dirs.iter().filter(|size| **size < 100_000).sum::<usize>();
    let part2 = || *dirs.iter().filter(|size| **size >= limit).min().unwrap();

    answer!(part, part1, part2)
  }
}

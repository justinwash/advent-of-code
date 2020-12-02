#[macro_use]
extern crate measure_time;

use std::{
  fs::File,
  io::{prelude::*, BufReader},
  path::Path,
};

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
  let file = File::open(filename).expect("no such file");
  let buf = BufReader::new(file);
  buf
    .lines()
    .map(|l| l.expect("Could not parse line"))
    .collect()
}

fn part_one() {
  print_time!("part 1");

  let input = lines_from_file("input");
  let mut result = 0;

  for line in input {
    let mut sections = line.split_whitespace();

    let mut min_max = sections.next().unwrap().split("-");
    let min: i32 = min_max.next().unwrap().parse().unwrap();
    let max: i32 = min_max.next().unwrap().parse().unwrap();

    let required_letter = sections.next().unwrap().chars().next().unwrap();
    let password = sections.next().unwrap();

    if (min..max + 1).contains(&(password.matches(required_letter).count() as i32)) {
      result += 1;
    }
  }

  println!("part 1 result: {}", result);
}

fn part_two() {
  print_time!("part 2");

  let input = lines_from_file("input");
  let mut result = 0;

  for line in input {
    let mut sections = line.split_whitespace();

    let mut min_max = sections.next().unwrap().split("-");
    let a_pos: i32 = min_max.next().unwrap().parse().unwrap();
    let b_pos: i32 = min_max.next().unwrap().parse().unwrap();

    let required_letter = sections.next().unwrap().chars().next().unwrap();
    let password = sections.next().unwrap();

    let instances = password.match_indices(required_letter).collect::<Vec<_>>();

    if instances_are_valid(instances, a_pos - 1, b_pos - 1, required_letter) {
      result += 1
    }
  }
  println!("part 2 result: {}", result)
}

fn instances_are_valid(
  instances: Vec<(usize, &str)>,
  a_pos: i32,
  b_pos: i32,
  required_letter: char,
) -> bool {
  let a_valid = instances.contains(&(a_pos as usize, &required_letter.to_string()));
  let b_valid = instances.contains(&(b_pos as usize, &required_letter.to_string()));
  return (a_valid || b_valid) && !(a_valid && b_valid);
}

fn main() {
  part_one();
  part_two();
}

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

fn ints_from_lines(lines: Vec<String>) -> Vec<i32> {
  let ints = lines.into_iter().map(|line| line.parse().unwrap());
  ints.collect()
}

fn part_one() {
  print_time!("part 1");

  let input = lines_from_file("input");
  let input_as_ints = ints_from_lines(input);

  let mut result: i32 = -1;

  for i in 0..input_as_ints.len() {
    if result != -1 {
      break;
    }
    for j in 0..input_as_ints.len() {
      if input_as_ints[i] == input_as_ints[j] {
      } else if input_as_ints[i] + input_as_ints[j] == 2020 {
        result = input_as_ints[i] * input_as_ints[j];
        break;
      }
    }
  }

  //println!("part 1 result: {}", result);
}

fn part_two() {
  print_time!("part 2");

  let input = lines_from_file("input");
  let input_as_ints = ints_from_lines(input);

  let mut result: i32 = -1;

  for i in 0..input_as_ints.len() {
    if result != -1 {
      break;
    }
    for j in 0..input_as_ints.len() {
      if result != -1 {
        break;
      }
      for k in 0..input_as_ints.len() {
        if input_as_ints[i] == input_as_ints[j]
          || input_as_ints[i] == input_as_ints[k]
          || input_as_ints[j] == input_as_ints[k]
        {
        } else if input_as_ints[i] + input_as_ints[j] + input_as_ints[k] == 2020 {
          result = input_as_ints[i] * input_as_ints[j] * input_as_ints[k];
          break;
        }
      }
    }
  }

  //println!("part 2 result: {}", result)
}

fn main() {
  part_one();

  part_two();
}

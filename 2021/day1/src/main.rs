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

fn part_one(input: &Vec<i32>) {
  print_time!("part 1");

  let mut increases: i32 = 0;

  for i in 1..input.len() {
    let a = input[i - 1];
    let b = input[i];

    if b > a {
      increases += 1;
    }
  }

  println!("part 1 result: {}", increases);
}

fn part_two(input: &Vec<i32>) {
  print_time!("part 2");

  let mut increases: i32 = 0;

  for i in 3..input.len() {
    let window1 = input[i - 3] + input[i - 2] + input[i - 1];
    let window2 = input[i - 2] + input[i - 1] + input[i];

    if window2 > window1 {
      increases += 1;
    }
  }

  println!("part 2 result: {}", increases);
}

fn main() {
  let input = lines_from_file("input");
  let input_as_ints = ints_from_lines(input);

  part_one(&input_as_ints);

  part_two(&input_as_ints);
}

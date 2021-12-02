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

fn instructions_from_lines(lines: &Vec<String>) -> Vec<(String, i32)> {
  let instructions = lines.into_iter().map(|line| {
    let direction_amount: Vec<&str> = line.split(" ").collect();
    (
      String::from(direction_amount[0]),
      direction_amount[1].parse().unwrap(),
    )
  });

  instructions.collect()
}

fn part_one(input: &Vec<String>) {
  print_time!("part 1");

  let instructions = instructions_from_lines(input);

  let mut x_pos: i32 = 0;
  let mut y_pos: i32 = 0;

  for instruction in 0..instructions.len() {
    let (direction, amount) = &instructions[instruction];

    match direction.as_str() {
      "up" => y_pos -= amount,
      "down" => y_pos += amount,
      "forward" => x_pos += amount,
      _ => panic!("Unknown direction"),
    }
  }

  println!("part 1 result: {}", x_pos * y_pos);
}

fn part_two(input: &Vec<String>) {
  print_time!("part 2");

  let instructions = instructions_from_lines(input);

  let mut aim: i32 = 0;
  let mut x_pos: i64 = 0;
  let mut y_pos: i64 = 0;

  for instruction in 0..instructions.len() {
    let (direction, amount) = &instructions[instruction];

    match direction.as_str() {
      "up" => aim -= amount,
      "down" => aim += amount,
      "forward" => {
        x_pos += *amount as i64;
        y_pos += *amount as i64 * aim as i64;
      }
      _ => panic!("Unknown direction"),
    }
  }

  println!("part 2 result: {}", x_pos * y_pos);
}

fn main() {
  let input = lines_from_file("input");
  part_one(&input);
  part_two(&input);
}

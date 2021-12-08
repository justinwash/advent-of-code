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

fn columns_from_lines(lines: &Vec<String>) -> Vec<Vec<i32>> {
  let mut columns: Vec<Vec<i32>> = (0..12).map(|_i| Vec::new()).collect();

  for line in lines {
    let line_values: Vec<&str> = line.split("").filter(|s| !s.is_empty()).collect();

    for (j, value) in line_values.into_iter().enumerate() {
      columns[j].push(value.parse::<i32>().unwrap());
    }
  }

  columns
}

fn part_one(input: &Vec<String>) {
  print_time!("part 1");

  let columns = columns_from_lines(input);

  let mut gamma: Vec<&str> = (0..12).map(|_i| "0").collect();
  let mut epsilon: Vec<&str> = (0..12).map(|_i| "0").collect();

  for (i, column) in columns.clone().into_iter().enumerate() {
    if column.clone().into_iter().filter(|&x| &x == &1).count()
      > column.into_iter().filter(|x| x == &0).count()
    {
      gamma[i] = "1";
      epsilon[i] = "0";
    } else {
      gamma[i] = "0";
      epsilon[i] = "1";
    }
  }

  let gamma_dec = isize::from_str_radix(&gamma.join(""), 2).unwrap();
  let epsilon_dec = isize::from_str_radix(&epsilon.join(""), 2).unwrap();

  println!("part 1 result: {}", gamma_dec * epsilon_dec);
}

fn part_two(input: &Vec<String>) {
  print_time!("part 2");

  println!("part 2 result: {}", "");
}

fn main() {
  let input = lines_from_file("input");
  part_one(&input);
  part_two(&input);
}

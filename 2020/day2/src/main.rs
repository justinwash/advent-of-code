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
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn ints_from_lines(lines: Vec<String>) -> Vec<i32> {
    let ints = lines.into_iter().map(|line| line.parse().unwrap());
    ints.collect()
}

fn part_one() {
    print_time!("part 1");

    println!("part 1 result: {}", "result");
}

fn part_two() {
    print_time!("part 2");

    println!("part 2 result: {}", "result")
}

fn main() {
    part_one();
    part_two();
}

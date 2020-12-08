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

fn locate_trees(input: &Vec<String>, r: usize, d: usize) -> u128 {
    let mut x = 0;
    let mut y = 0;

    let mut trees = 0;

    while y < input.len() {
        if input[y].chars().nth(x % input[y].len()).unwrap() == '#' {
            trees += 1;
        }
        x += r;
        y += d;
    }

    trees
}

fn part_one() {
    print_time!("part 1");

    let input = lines_from_file("input");
    let trees = locate_trees(&input, 3, 1);

    println!("part 1 result: {}", trees);
}

fn part_two() {
    print_time!("part 2");

    let input = lines_from_file("input");
    let result: u128 = locate_trees(&input, 1, 1)
        * locate_trees(&input, 3, 1)
        * locate_trees(&input, 5, 1)
        * locate_trees(&input, 7, 1)
        * locate_trees(&input, 1, 2);

    println!("{}", result)
}

fn main() {
    part_one();
    part_two();
}

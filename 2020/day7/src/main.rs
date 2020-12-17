#[macro_use]
extern crate measure_time;
extern crate regex;

use std::{
    collections::HashMap,
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

fn part_one() {
    print_time!("part 1");

    let input = lines_from_file("example_input");

    let mut rules: HashMap<String, Vec<String>> = HashMap::new();

    for line in input {
        let line = line.replace("bags", "").replace("bag", "").replace('.', "");

        let parts: Vec<String> = line.split("contain").map(|item| item.into()).collect();

        let (bag_name, bags_contains): (String, &str) = (parts[0].trim().into(), parts[1].trim());

        let can_hold: Vec<String> = bags_contains
            .split(',')
            .map(|item| item.trim().into())
            .collect();

        rules.insert(bag_name, can_hold);
    }

    println!("{:?}", rules);

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

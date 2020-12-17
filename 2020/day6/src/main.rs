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

fn part_one() {
    print_time!("part 1");

    let input = lines_from_file("input");

    let mut groups: Vec<Vec<String>> = Vec::new();

    let mut temp_group: Vec<String> = Vec::new();
    for line in input {
        if line != "" {
            temp_group.push(line);
        } else {
            groups.push(temp_group);
            temp_group = Vec::new();
        }
    }

    let mut result = 0;
    for group in groups {
        let mut seen: Vec<char> = Vec::new();
        for answer_list in group {
            for char in answer_list.chars() {
                if !seen.contains(&char) {
                    seen.push(char)
                }
            }
        }
        result += seen.len();
    }

    println!("part 1 result: {}", result);
}

fn part_two() {
    print_time!("part 2");

    println!("part 2 result: {}", "result")
}

fn main() {
    part_one();
    part_two();
}

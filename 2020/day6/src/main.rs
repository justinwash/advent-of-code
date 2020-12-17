#[macro_use]
extern crate measure_time;

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
        let mut seen = HashMap::new();
        let mut unanimous: Vec<char> = Vec::new();
        for answer_list in &group {
            for char in answer_list.chars() {
                if let Some(x) = seen.get_mut(&char) {
                    *x = *x + 1;

                    if *x as i32 >= group.len() as i32 {
                        unanimous.push(char)
                    }
                } else {
                    seen.entry(char).or_insert(1);
                    if *seen.get(&char).unwrap() as i32 >= group.len() as i32 {
                        unanimous.push(char)
                    }
                }
            }
        }
        result += &unanimous.len();
        println!(
            "Group: {:?} \n Seen: {:?} \n Unanimous: {:?} Count: {}",
            group,
            seen,
            unanimous,
            unanimous.len()
        )
    }

    println!("part 2 result: {}", result)
}

fn main() {
    part_one();
    part_two();
}

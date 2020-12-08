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

#[derive(Clone, Debug)]
struct Entry {
    byr: String,
    iyr: String,
    eyr: String,
    hgt: String,
    hcl: String,
    ecl: String,
    pid: String,
    cid: String,
}

fn get_field_value(field_obj: &Vec<&str>, field_name: String) -> String {
    if field_obj.iter().position(|&s| s == field_name) != None {
        return field_obj[field_obj.iter().position(|&s| s == field_name).unwrap() + 1].to_string();
    } else {
        return "".to_string();
    }
}

fn part_one() {
    print_time!("part 1");

    let input = lines_from_file("input");

    let mut entries: Vec<Entry> = Vec::new();
    let mut entry_index = 0;

    for i in 0..input.len() {
        if input[i].len() > 0 {
            let fields = input[i].split_whitespace();
            for field in fields {
                let field_obj: Vec<&str> = field.split(":").collect();

                if entry_index >= entries.len() {
                    entries.resize(
                        entry_index + 1,
                        Entry {
                            byr: "".to_string(),
                            iyr: "".to_string(),
                            eyr: "".to_string(),
                            hgt: "".to_string(),
                            hcl: "".to_string(),
                            ecl: "".to_string(),
                            pid: "".to_string(),
                            cid: "".to_string(),
                        },
                    );
                }

                if get_field_value(&field_obj, "byr".to_string()) != "" {
                    entries[entry_index].byr = get_field_value(&field_obj, "byr".to_string())
                }
                if get_field_value(&field_obj, "iyr".to_string()) != "" {
                    entries[entry_index].iyr = get_field_value(&field_obj, "iyr".to_string())
                }
                if get_field_value(&field_obj, "eyr".to_string()) != "" {
                    entries[entry_index].eyr = get_field_value(&field_obj, "eyr".to_string())
                }
                if get_field_value(&field_obj, "hgt".to_string()) != "" {
                    entries[entry_index].hgt = get_field_value(&field_obj, "hgt".to_string())
                }
                if get_field_value(&field_obj, "hcl".to_string()) != "" {
                    entries[entry_index].hcl = get_field_value(&field_obj, "hcl".to_string())
                }
                if get_field_value(&field_obj, "ecl".to_string()) != "" {
                    entries[entry_index].ecl = get_field_value(&field_obj, "ecl".to_string())
                }
                if get_field_value(&field_obj, "pid".to_string()) != "" {
                    entries[entry_index].pid = get_field_value(&field_obj, "pid".to_string())
                }
                if get_field_value(&field_obj, "cid".to_string()) != "" {
                    entries[entry_index].cid = get_field_value(&field_obj, "cid".to_string())
                }
            }
        } else {
            entry_index += 1;
        }
    }

    let mut result = 0;
    for i in 0..entries.len() {
        if entries[i].byr == ""
            || entries[i].iyr == ""
            || entries[i].eyr == ""
            || entries[i].hgt == ""
            || entries[i].hcl == ""
            || entries[i].ecl == ""
            || entries[i].pid == ""
        {
        } else {
            result += 1
        }
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

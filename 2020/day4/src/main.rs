#[macro_use]
extern crate measure_time;
extern crate lazy_static;

use regex::Regex;
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
        if entries[i].byr != ""
            || entries[i].iyr != ""
            || entries[i].eyr != ""
            || entries[i].hgt != ""
            || entries[i].hcl != ""
            || entries[i].ecl != ""
            || entries[i].pid != ""
        {
            if validate_byr(&entries[i].byr)
                && validate_iyr(&entries[i].iyr)
                && validate_eyr(&entries[i].eyr)
                && validate_hgt(&entries[i].hgt)
                && validate_hcl(&entries[i].hcl)
                && validate_ecl(&entries[i].ecl)
                && validate_pid(&entries[i].pid)
            {
                result += 1
            }
        }
    }

    println!("part 2 result: {}", result)
}

fn validate_yr(year: &String, min_year: u32, max_year: u32) -> bool {
    lazy_static::lazy_static! {
        static ref YR_REGEX: Regex = Regex::new(r"^(?x)\d{4}$").unwrap();
    }

    if YR_REGEX.is_match(&year) {
        let year = year.parse::<u32>().unwrap();
        return (min_year..=max_year).contains(&year);
    } else {
        return false;
    }
}

fn validate_byr(byr: &String) -> bool {
    validate_yr(byr, 1920, 2002)
}

fn validate_iyr(iyr: &String) -> bool {
    validate_yr(iyr, 2010, 2020)
}

fn validate_eyr(eyr: &String) -> bool {
    validate_yr(eyr, 2020, 2030)
}

fn validate_hgt(hgt: &String) -> bool {
    lazy_static::lazy_static! {
        static ref HGT_REGEX: Regex = Regex::new(r"^\d*(cm|in)$").unwrap();
    }

    let v_height = if HGT_REGEX.is_match(hgt) {
        let height = hgt[0..hgt.len() - 2].parse::<i32>().unwrap();

        match &hgt[hgt.len() - 2..hgt.len()] {
            "cm" => (150..=193).contains(&height),
            "in" => (59..=76).contains(&height),
            _ => unreachable!(),
        }
    } else {
        false
    };

    v_height
}

fn validate_hcl(hcl: &String) -> bool {
    lazy_static::lazy_static! {
        static ref HCL_REGEX: Regex = Regex::new(r"^#(\d|[a-f]){6}$").unwrap();
    }

    HCL_REGEX.is_match(hcl)
}

fn validate_ecl(ecl: &String) -> bool {
    lazy_static::lazy_static! {
        static ref ECL_REGEX: Regex = Regex::new(r"^amb|blu|brn|gry|grn|hzl|oth$").unwrap();
    }

    ECL_REGEX.is_match(ecl)
}

fn validate_pid(pid: &String) -> bool {
    lazy_static::lazy_static! {
        static ref PID_REGEX: Regex = Regex::new(r"^\d{9}$").unwrap();
    }

    PID_REGEX.is_match(pid)
}

fn main() {
    part_one();
    part_two();
}

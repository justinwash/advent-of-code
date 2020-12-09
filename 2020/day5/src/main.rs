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
    let mut result = 0;

    for seat in input {
        let mut rows = 0..127;
        let mut columns = 0..7;

        let (row_id, column_id) = seat.split_at(7);

        for i in row_id.chars() {
            match i {
                'F' => {
                    rows.end = rows.end - ((rows.end - rows.start) / 2) - 1;
                }
                'B' => {
                    rows.start = rows.start + ((rows.end - rows.start) / 2) + 1;
                }
                _ => unreachable!(),
            }
        }

        for i in column_id.chars() {
            match i {
                'L' => {
                    columns.end = columns.end - ((columns.end - columns.start) / 2) - 1;
                }
                'R' => {
                    columns.start = columns.start + ((columns.end - columns.start) / 2) + 1;
                }
                _ => unreachable!(),
            }
        }

        let id = rows.start * 8 + columns.start;
        if id > result {
            result = id
        }
    }

    println!("part 1 result: {}", result);
}

fn part_two() {
    print_time!("part 2");

    let input = lines_from_file("input");
    let mut ids: Vec<i32> = Vec::new();

    for seat in input {
        let mut rows = 0..127;
        let mut columns = 0..7;

        let (row_id, column_id) = seat.split_at(7);

        for i in row_id.chars() {
            match i {
                'F' => {
                    rows.end = rows.end - ((rows.end - rows.start) / 2) - 1;
                }
                'B' => {
                    rows.start = rows.start + ((rows.end - rows.start) / 2) + 1;
                }
                _ => unreachable!(),
            }
        }

        for i in column_id.chars() {
            match i {
                'L' => {
                    columns.end = columns.end - ((columns.end - columns.start) / 2) - 1;
                }
                'R' => {
                    columns.start = columns.start + ((columns.end - columns.start) / 2) + 1;
                }
                _ => unreachable!(),
            }
        }
        let row = rows.start;
        let column = columns.start;

        ids.push(row * 8 + column);
    }

    ids.sort();
    let mut result = 0;
    for i in 0..ids.len() {
        if ids[i + 1] - ids[i] > 1 {
            result = ids[i] + 1;
        } else {
            result = 0;
        }

        if result != 0 {
            break;
        }
    }

    println!("part 2 result: {:?}", result)
}

fn main() {
    part_one();
    part_two();
}

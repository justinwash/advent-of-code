use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};
fn weights_from_file(filename: impl AsRef<Path>) -> Vec<f64> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.unwrap().parse::<f64>().expect("oops"))
        .collect()
}
fn main() {
    let weights = weights_from_file("input.txt");
    let fuel_requirements: Vec<f64> = weights.iter().map(|x| {
        (x / 3 as f64).floor() - 2 as f64
    }).collect();
    let mut fuel_sum = 0.0;
    for req in &fuel_requirements {
        fuel_sum += req
    }
    print!("{:?}", fuel_sum)
}
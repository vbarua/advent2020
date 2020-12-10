use std::fs::File;
use std::io::BufReader;

use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let file = File::open("src/10/bin/input.txt")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    let mut input: Vec<i32> = contents.lines().map(|l| l.parse().unwrap()).collect();
    let device_joltage = input.iter().max().unwrap() + 3;
    input.push(0);
    input.push(device_joltage);
    input.sort();

    let diffs: Vec<i32> = input
        .iter()
        .zip(input.iter().skip(1))
        .map(|(j, jn)| *jn - *j)
        .collect();

    let ones = diffs.iter().filter(|&d| *d == 1).count();
    let threes = diffs.iter().filter(|&d| *d == 3).count();
    println!("Part 1: {} * {} = {}", ones, threes, ones * threes);

    Ok(())
}

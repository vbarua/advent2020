use std::fs::File;
use std::io::BufReader;
use std::str::FromStr;

use std::io::prelude::*;

const TARGET_VALUE: i32 = 2020;

fn dual_sum(target: i32, input: &Vec<i32>) -> Option<(i32, i32)> {
    let mut forward_iter = input.iter();
    let mut reverse_iter = input.iter().rev();

    let mut first = forward_iter.next().unwrap();
    let mut last = reverse_iter.next().unwrap();
    loop {
        let sum = first + last;
        if sum == target {
            return Some((*first, *last));
        } else if first == last {
            return None;
        } else if sum < target {
            first = forward_iter.next().unwrap();
        } else {
            last = reverse_iter.next().unwrap();
        }
    }
}

fn triple_sum(target: i32, input: &Vec<i32>) -> Option<(i32, i32, i32)> {
    for x in input {
        match dual_sum(target - x, input) {
            Some(res) => {
                let (y, z) = res;
                return Some((*x, y, z));
            }
            None => continue,
        }
    }
    None
}

fn main() -> std::io::Result<()> {
    let file = File::open("src/01/bin/input.txt")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    let mut lines = contents
        .lines()
        .map(|s| i32::from_str(s).unwrap())
        .collect::<Vec<_>>();
    lines.sort();

    dual_sum(TARGET_VALUE, &lines).map(|res| {
        let (i, j) = res;
        println!("Dual Sum Solution");
        println!("{} + {} = {}", i, j, i + j);
        println!("{} * {} = {}", i, j, i * j);
        println!();
    });

    triple_sum(TARGET_VALUE, &lines).map(|res| {
        let (x, y, z) = res;
        println!("Triple Sum Solution");
        println!("{} + {} + {} = {}", x, y, z, x + y + z);
        println!("{} * {} * {} = {}", x, y, z, x * y * z);
    });

    Ok(())
}

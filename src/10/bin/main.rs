use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

use std::io::prelude::*;

fn paths(input: &[i32]) -> i64 {
    let minimum_joltage = *input.iter().min().unwrap();
    let maximum_joltage = *input.iter().max().unwrap();
    let mut joltages: HashMap<i32, i64> = input.iter().cloned().map(|j| (j, 0)).collect();
    joltages.insert(maximum_joltage, 1);

    for &j in input.iter().rev().skip(1) {
        let j1 = joltages.get(&(j + 1)).cloned().unwrap_or(0);
        let j2 = joltages.get(&(j + 2)).cloned().unwrap_or(0);
        let j3 = joltages.get(&(j + 3)).cloned().unwrap_or(0);

        let paths_from_j = j1 + j2 + j3;
        joltages.insert(j, paths_from_j);
    }

    return *joltages.get(&minimum_joltage).unwrap();
}

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

    // Part 1
    let diffs: Vec<i32> = input
        .iter()
        .zip(input.iter().skip(1))
        .map(|(j, jn)| *jn - *j)
        .collect();

    let ones = diffs.iter().filter(|&d| *d == 1).count();
    let threes = diffs.iter().filter(|&d| *d == 3).count();
    println!("Part 1: {} * {} = {}", ones, threes, ones * threes);

    // Part 2
    let combinations = paths(&input);
    println!("Part 2: {} Combinations", combinations);

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::assert_eq;

    use super::*;

    #[test]
    fn paths_tests() {
        assert_eq!(1, paths(&vec![0]));
        assert_eq!(1, paths(&vec![0, 3]));
        assert_eq!(1, paths(&vec![0, 3, 6]));
        assert_eq!(4, paths(&vec![0, 1, 2, 3]));
    }
}

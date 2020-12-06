use std::collections::HashSet;
use std::fs::File;
use std::io::BufReader;
use std::iter::FromIterator;

use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let file = File::open("src/06/bin/input.txt")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    let p1: usize = contents
        .split_terminator("\n\n")
        .map(|chunk| {
            let mut responses: HashSet<char> = HashSet::new();
            for l in chunk.lines() {
                for c in l.chars() {
                    responses.insert(c);
                }
            }
            responses.len()
        })
        .sum();

    println!("Part 1: {}", p1);

    let p2: usize = contents
        .split_terminator("\n\n")
        .map(|chunk| {
            let mut responses: HashSet<char> =
                HashSet::from_iter("abcdefghijklmnopqrstuvwxyz".chars());
            for l in chunk.lines() {
                let temp: HashSet<char> = HashSet::from_iter(l.chars());
                responses = responses.intersection(&temp).cloned().collect();
            }
            responses.len()
        })
        .sum();

    println!("Part 2: {}", p2);

    Ok(())
}

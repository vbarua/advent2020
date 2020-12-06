use std::fs::File;
use std::collections::HashSet;
use std::io::BufReader;

use std::io::prelude::*;

fn main() -> std::io::Result<()> {
  let file = File::open("src/06/bin/input.txt")?;
  let mut buf_reader = BufReader::new(file);
  let mut contents = String::new();
  buf_reader.read_to_string(&mut contents)?;

  let p1: usize = contents.split_terminator("\n\n").map(|chunk| {
    let mut responses: HashSet<char> = HashSet::new();
    for l in chunk.lines() {
      for c in l.chars() {
        responses.insert(c);
      }
    }
    responses.len()
  }).sum();

  println!("Part 1: {}", p1);

  Ok(())
}
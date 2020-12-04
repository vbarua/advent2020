use std::fs::File;
use std::io::BufReader;

use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let file = File::open("src/03/bin/input.txt")?;
  let mut buf_reader = BufReader::new(file);
  let mut contents = String::new();
  buf_reader.read_to_string(&mut contents)?;
  let terrain: Vec<Vec<char>> = contents.lines().map(|l| l.chars().collect()).collect::<Vec<_>>();

  let width = terrain.first().unwrap().len();
  let height = terrain.len();

  let mut x = 0;
  let mut collisions = 0;
  for y in 1..height {
    x += 3;
    x %= width;
    let collision = terrain[y][x] == '#';
    if collision {
      collisions += 1;
    }
    println!("({}, {}, {})", x, y, collision);
  }
  println!("Number of Collisions: {}", collisions);
  Ok(())
}
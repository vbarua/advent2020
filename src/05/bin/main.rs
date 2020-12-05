use std::{fs::File, cmp::max};
use std::io::BufReader;

use std::io::prelude::*;

fn bsp_fb(input: &str) -> i32 {
  let mut lower = 0;
  let mut upper = 127;
  for c in input.chars().take(7) {
    match c {
      'F' => upper -= (upper - lower) / 2 + 1,
      'B' => lower += (upper - lower) / 2 + 1,
      _ => panic!()
    }
  };

  match input.chars().take(7).last().unwrap() {
    'F' => lower,
    'B' => upper,
    _ => panic!()
  }
}

fn bsp_lr(input: &str) -> i32 {
  let mut lower = 0;
  let mut upper = 7;
  for c in input.chars().skip(7) {
    match c {
      'L' => upper -= (upper - lower) / 2 + 1,
      'R' => lower += (upper - lower) / 2 + 1,
      _ => panic!()
    }
  };

  match input.chars().skip(7).last().unwrap() {
    'L' => lower,
    'R' => upper,
    _ => panic!()
  }
}


fn find_seat(s: &str) -> (i32, i32, i32) {
  let row = bsp_fb(s);
  let col = bsp_lr(s);
  (row, col, row * 8 + col)
}

fn main() -> std::io::Result<()> {
  let file = File::open("src/05/bin/input.txt")?;
  let mut buf_reader = BufReader::new(file);
  let mut contents = String::new();
  buf_reader.read_to_string(&mut contents)?;

  let mut max_id = 0;
  for l in contents.lines() {
    let (row, col, id) = find_seat(l);
    println!("{} - {} | {}", row, col, id);
    max_id = max(max_id, id)
  }
  println!();
  println!("Max Id: {}", max_id);

  Ok(())
}

#[cfg(test)]
mod tests {
  use std::assert_eq;
  use super::*;

  #[test]
  fn test_row_bsp() {
      assert_eq!(bsp_fb("FBFBBFFRLR"), 44);
      assert_eq!(bsp_fb("BFFFBBFRRR"), 70);
      assert_eq!(bsp_fb("FFFBBBFRRR"), 14);
      assert_eq!(bsp_fb("BBFFBBFRLL"), 102);
  }
  #[test]
  fn test_column_bsp() {
    assert_eq!(bsp_lr("FBFBBFFRLR"), 5);
    assert_eq!(bsp_lr("BFFFBBFRRR"), 7);
    assert_eq!(bsp_lr("BBFFBBFRLL"), 4);
  }
}
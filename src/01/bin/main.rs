use std::fs::File;
use std::io::BufReader;
use std::str::FromStr;

use std::io::prelude::*;

const TARGET_VALUE: i32 = 2020;

fn main() -> std::io::Result<()> {
  let file = File::open("src/01/bin/input.txt")?;
  let mut buf_reader = BufReader::new(file);
  let mut contents = String::new();
  buf_reader.read_to_string(&mut contents)?;
  let mut lines  = contents.lines().map(|s| i32::from_str(s).unwrap()).collect::<Vec<_>>();
  lines.sort();
  let forward_iter = lines.iter();
  let mut reverse_iter = lines.iter().rev();
  // let last = reverse_iter.next();
  for i in forward_iter {
    while let Some(j) = reverse_iter.next() {
      let sum = i + j;
      if (sum) == TARGET_VALUE {
        println!("{} + {} = {}\n{} * {} = {}",
          i, j, sum, i, j, i*j);
      } else if sum < TARGET_VALUE {
        break;
      }
    }
  }
  // println!("{}", first);

  // for l in lines.iter() {
  //   println!("{}", l);
  // }

  Ok(())
}

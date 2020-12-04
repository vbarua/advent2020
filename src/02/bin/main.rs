use std::fs::File;
use std::io::BufReader;

use std::io::prelude::*;


#[derive(PartialEq, Debug)]
struct Entry {
  from: i32,
  to: i32,
  character: char,
  password: String
}

fn to_entry(s: &str) -> Entry {
  let parts: Vec<&str> = s.split(|c| c == '-' || c == ' ' || c == ':').collect();

  if let [from, to, character, _, password] = parts[..] {
    return Entry {
      from: from.parse::<i32>().unwrap(),
      to: to.parse::<i32>().unwrap(),
      character: character.chars().next().unwrap(),
      password: String::from(password)
    }
  }
  panic!("Failed to parse");
}

fn is_valid(e: &Entry) -> bool {
  let usages = e.password.chars().filter(|c| *c == e.character).count();
  (e.from as usize) <= usages && usages <= (e.to as usize)
}

fn main() -> std::io::Result<()> {
  let file = File::open("src/02/bin/input.txt")?;
  let mut buf_reader = BufReader::new(file);
  let mut contents = String::new();
  buf_reader.read_to_string(&mut contents)?;
  let entries  = contents.lines().map(|l| to_entry(l));

  println!("Valid Passwords: {}", entries.filter(|e| is_valid(e)).count());


  Ok(())
}

#[cfg(test)]
mod tests {
  use std::assert_eq;

    use super::*;

  #[test]
  fn extract_entry() {
    let e = to_entry("17-18 q: cqqjvhqhrpqlqvqdjlbb");
    assert_eq!(e, Entry {
      from: 17,
      to: 18,
      character: 'q',
      password: String::from("cqqjvhqhrpqlqvqdjlbb")
    })
  }

  #[test]
  fn valid_password() {
    assert!(is_valid(&to_entry("1-3 a: abcde")));
    assert!(is_valid(&to_entry("2-9 c: ccccccccc")));
  }

  #[test]
  fn invalid_password() {
    let e = to_entry("1-3 b: cdefg");
    assert!(!is_valid(&e))
  }
}

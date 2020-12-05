use std::fs::File;
use std::io::BufReader;
use std::collections::HashMap;

use std::io::prelude::*;
#[derive(PartialEq, Debug)]
struct Passport {
  byr: bool,
  iyr: bool,
  eyr: bool,
  hgt: bool,
  hcl: bool,
  ecl: bool,
  pid: bool,
  cid: bool
}

fn validate_byr(s: &String) -> bool {
  s.parse::<i32>().map(|v| 1920 <= v && v <= 2002).unwrap()
}

fn validate_iyr(s: &String) -> bool {
  s.parse::<i32>().map(|v| 2010 <= v && v <= 2020).unwrap()
}

fn validate_eyr(s: &String) -> bool {
  s.parse::<i32>().map(|v| 2020 <= v && v <= 2030).unwrap()
}

fn validate_hgt(s: &String) -> bool {
  let height = s.chars().take_while(|c| c.is_digit(10)).collect::<String>().parse::<i32>().unwrap();
  if s.ends_with("cm") {
    150 <= height && height <= 193
  } else if s.ends_with("in") {
    59 <= height && height <= 76
  } else {
    false
  }
}

fn validate_hcl(s: &String) -> bool {
  s.starts_with("#") && s.chars().skip(1).all(|c| c.is_digit(16)) && s.len() == 7
}

fn validate_ecl(s: &String) -> bool {
  for color in &["amb", "blu", "brn", "gry", "grn", "hzl", "oth"] {
    if s == *color { return true }
  }
  false
}

fn validate_pid(s: &String) -> bool {
  s.chars().all(|c| c.is_digit(10)) && s.len() == 9
}

fn to_passport(input_chunk: &str) -> Passport {
  let raw_passport = input_chunk.replace("\n", " ");
  let fields: HashMap<_, _> =
    raw_passport.split_terminator(" ").map(|item| {
      let key = &item[0..3];
      let value = &item[4..];
      (key.to_string(), value.to_string())
    }).collect();

  Passport {
    byr: fields.get("byr").map(validate_byr).unwrap_or(false),
    iyr: fields.get("iyr").map(validate_iyr).unwrap_or(false),
    eyr: fields.get("eyr").map(validate_eyr).unwrap_or(false),
    hgt: fields.get("hgt").map(validate_hgt).unwrap_or(false),
    hcl: fields.get("hcl").map(validate_hcl).unwrap_or(false),
    ecl: fields.get("ecl").map(validate_ecl).unwrap_or(false),
    pid: fields.get("pid").map(validate_pid).unwrap_or(false),
    cid: fields.get("cid").is_some()
  }
}

fn is_valid(p: &Passport) -> bool {
  p.byr && p.iyr && p.eyr && p.hgt && p.hcl && p.ecl && p.pid //&& p.cid
}

fn main() -> std::io::Result<()> {
  let file = File::open("src/04/bin/input.txt")?;
  let mut buf_reader = BufReader::new(file);
  let mut contents = String::new();
  buf_reader.read_to_string(&mut contents)?;

  let passports = contents.split_terminator("\n\n").map(to_passport);

  let valid_passports = passports.filter(is_valid).count();
  println!("Valid Passports: {}", valid_passports);

  Ok(())
}
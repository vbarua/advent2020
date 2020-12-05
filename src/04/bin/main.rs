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

fn to_passport(input_chunk: &str) -> Passport {
  let raw_passport = input_chunk.replace("\n", " ");
  let fields: HashMap<_, _> =
    raw_passport.split_terminator(" ").map(|item| {
      let key = &item[0..3];
      let value = &item[4..];
      (key.to_string(), value.to_string())
    }).collect();

  Passport {
    byr: fields.get("byr").is_some(),
    iyr: fields.get("iyr").is_some(),
    eyr: fields.get("eyr").is_some(),
    hgt: fields.get("hgt").is_some(),
    hcl: fields.get("hcl").is_some(),
    ecl: fields.get("ecl").is_some(),
    pid: fields.get("pid").is_some(),
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
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::BufReader;

use std::io::prelude::*;

#[derive(Debug)]
struct Bag {
    color: String,
    contents: Vec<BagContent>,
}

#[derive(Debug)]
struct BagContent {
    color: String,
    number: i32,
}

fn parse_line(l: &str) -> Bag {
    let chunks: Vec<&str> = l.split_terminator(" ").collect();
    let mut c_iter = chunks.iter();

    let mut color = String::new();
    color.push_str(c_iter.next().unwrap());
    color.push_str(c_iter.next().unwrap());

    c_iter.next();
    c_iter.next();

    let mut bag_contents: Vec<BagContent> = Vec::new();

    loop {
        let next = c_iter.next();
        let number: i32 = match next {
            None => break,
            Some(s) if *s == "no" => break,
            Some(n) => n.parse().unwrap(),
        };

        let mut c = String::new();
        c.push_str(c_iter.next().unwrap());
        c.push_str(c_iter.next().unwrap());

        bag_contents.push(BagContent {
            color: c,
            number: number,
        });

        c_iter.next();
    }

    Bag {
        color: color,
        contents: bag_contents,
    }
}

fn shinygold_parents(bags: &Vec<Bag>) -> usize {
    let mut visited: HashSet<String> = HashSet::new();
    let mut search_for: Vec<String> = Vec::new();
    search_for.push(String::from("shinygold"));
    while let Some(search_color) = search_for.pop() {
        if visited.contains(&search_color) {
            continue;
        }
        visited.insert(search_color.clone());
        for bag in bags {
            for sub_bag in &bag.contents {
                if sub_bag.color == search_color {
                    search_for.push(bag.color.clone());
                }
            }
        }
    }
    visited.len() - 1
}

fn shinygold_contents(bags: &Vec<Bag>) -> i32 {
    let bag_map: HashMap<String, &Bag> =
        bags.iter().map(|item| (item.color.clone(), item)).collect();

    let mut include: Vec<&Bag> = Vec::new();
    include.push(bag_map["shinygold"]);

    let mut number_bags = 0;
    while let Some(bag) = include.pop() {
        number_bags += 1;
        for bag_content in &bag.contents {
            let b: &Bag = bag_map[&bag_content.color];
            for _ in 0..bag_content.number {
                include.push(b);
            }
        }
    }

    number_bags - 1
}

fn main() -> std::io::Result<()> {
    let file = File::open("src/07/bin/input.txt")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    let bags: Vec<Bag> = contents.lines().map(|l| parse_line(l)).collect();

    println!("Potential shinygold parents: {}", shinygold_parents(&bags));
    println!();
    println!("shinygold contents count: {}", shinygold_contents(&bags));

    Ok(())
}

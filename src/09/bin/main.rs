use std::collections::VecDeque;
use std::fs::File;
use std::io::BufReader;

use std::io::prelude::*;

fn find_weakness(preamble_len: usize, input: &Vec<i64>) -> i64 {
    let mut ns: VecDeque<i64> = input.iter().take(preamble_len).cloned().collect();
    for m in input.iter().skip(preamble_len) {
        let mut failed = true;
        for n in ns.iter() {
            let target = m - n;
            if ns.contains(&target) {
                failed = false;
            }
        }
        if failed {
            return *m;
        }
        ns.pop_front();
        ns.push_back(*m);
    }
    panic!();
}

fn main() -> std::io::Result<()> {
    let file = File::open("src/09/bin/input.txt")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    let input: Vec<i64> = contents.lines().map(|l| l.parse().unwrap()).collect();

    let weakness = find_weakness(25, &input);
    println!("Part 1: {}", weakness);

    Ok(())
}

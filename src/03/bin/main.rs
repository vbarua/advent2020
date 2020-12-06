use std::fs::File;
use std::io::BufReader;

use std::io::prelude::*;

fn traverse(terrain: &Vec<Vec<char>>, dx: usize, dy: usize) -> u64 {
    let width = terrain.first().unwrap().len();
    let height = terrain.len();

    let mut x = 0;
    let mut y = 0;
    let mut collisions = 0;
    while y < height - 1 {
        x += dx;
        x %= width;
        y += dy;
        let collision = terrain[y][x] == '#';
        if collision {
            collisions += 1;
        }
    }
    println!("dx: {}, dy: {} - Collisions: {}", dx, dy, collisions);
    return collisions;
}
#[cfg(debug_assertions)]
fn main() -> std::io::Result<()> {
    let file = File::open("src/03/bin/input.txt")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;
    let terrain: Vec<Vec<char>> = contents
        .lines()
        .map(|l| l.chars().collect())
        .collect::<Vec<_>>();

    let results = [
        traverse(&terrain, 1, 1),
        traverse(&terrain, 3, 1),
        traverse(&terrain, 5, 1),
        traverse(&terrain, 7, 1),
        traverse(&terrain, 1, 2),
    ];

    let trees_product: u64 = results.iter().product();
    println!("Product of All Trees: {}", trees_product);

    Ok(())
}

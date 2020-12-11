use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

use std::io::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tile {
    Floor,
    Empty,
    Occupied,
}

type Key = (isize, isize);
type Board = HashMap<Key, Tile>;

fn parse_tile(c: char) -> Tile {
    match c {
        '.' => Tile::Floor,
        'L' => Tile::Empty,
        '#' => Tile::Occupied,
        _ => panic!("Invalid Input"),
    }
}

fn parse_input(input: String) -> Board {
    let mut board: Board = HashMap::new();
    for (row, l) in input.lines().enumerate() {
        for (col, c) in l.chars().enumerate() {
            board.insert((row as isize, col as isize), parse_tile(c));
        }
    }
    board
}

fn simulate(board: &Board) -> Board {
    let mut board: Board = board.clone();
    let mut next_board: Board = HashMap::new();
    let mut cycle = 1;

    loop {
        println!("{}", cycle);
        cycle += 1;
        for (k, v) in board.iter() {
            let (row, col) = *k;
            // Assume (row, col) == (1, 1) for notation
            // 00, 01, 02
            // 10, 11, 12
            // 20, 21, 22
            let cell00 = board.get(&(row - 1, col - 1)).unwrap_or(&Tile::Floor);
            let cell01 = board.get(&(row - 1, col)).unwrap_or(&Tile::Floor);
            let cell02 = board.get(&(row - 1, col + 1)).unwrap_or(&Tile::Floor);

            let cell10 = board.get(&(row, col - 1)).unwrap_or(&Tile::Floor);
            let cell12 = board.get(&(row, col + 1)).unwrap_or(&Tile::Floor);

            let cell20 = board.get(&(row + 1, col - 1)).unwrap_or(&Tile::Floor);
            let cell21 = board.get(&(row + 1, col)).unwrap_or(&Tile::Floor);
            let cell22 = board.get(&(row + 1, col + 1)).unwrap_or(&Tile::Floor);

            let adjacenct = [
                cell00, cell01, cell02, cell10, cell12, cell20, cell21, cell22,
            ];

            let key = (row as isize, col as isize);
            match *v {
                Tile::Floor => {
                    next_board.insert(key, Tile::Floor);
                    ()
                }
                Tile::Empty => {
                    if adjacenct.iter().any(|&t| *t == Tile::Occupied) {
                        next_board.insert(key, Tile::Empty);
                    } else {
                        next_board.insert(key, Tile::Occupied);
                    }
                }
                Tile::Occupied => {
                    if adjacenct.iter().filter(|&&t| *t == Tile::Occupied).count() >= 4 {
                        next_board.insert(key, Tile::Empty);
                    } else {
                        next_board.insert(key, Tile::Occupied);
                    }
                }
            };
        }
        if board == next_board {
            return next_board;
        } else {
            board = next_board.clone();
        }
    }
}

fn main() -> std::io::Result<()> {
    let file = File::open("src/11/bin/input.txt")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    let board = parse_input(contents);
    let stable_board = simulate(&board);

    let occupied_seats = stable_board
        .values()
        .filter(|&t| *t == Tile::Occupied)
        .count();
    println!("Occupied Seats: {}", occupied_seats);

    Ok(())
}

use std::fs::File;
use std::io::BufReader;

use std::io::prelude::*;

#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}
#[derive(Debug, Clone, Copy)]
struct Heading {
    direction: Direction,
    distance: i32,
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Shift(Heading),
    Left(i32),
    Right(i32),
    Forward(i32),
}

fn parse_distance(input: &str) -> i32 {
    input[1..].parse().unwrap()
}

fn parse_input(input: &str) -> Vec<Instruction> {
    return input
        .lines()
        .map(|l| match l {
            l if l.starts_with("N") => Instruction::Shift(Heading {
                direction: Direction::North,
                distance: parse_distance(l),
            }),
            l if l.starts_with("E") => Instruction::Shift(Heading {
                direction: Direction::East,
                distance: parse_distance(l),
            }),
            l if l.starts_with("S") => Instruction::Shift(Heading {
                direction: Direction::South,
                distance: parse_distance(l),
            }),
            l if l.starts_with("W") => Instruction::Shift(Heading {
                direction: Direction::West,
                distance: parse_distance(l),
            }),
            l if l.starts_with("L") => Instruction::Left(parse_distance(l)),
            l if l.starts_with("R") => Instruction::Right(parse_distance(l)),
            l if l.starts_with("F") => Instruction::Forward(parse_distance(l)),
            _ => panic!("Invalid Input"),
        })
        .collect();
}

type Coord = (i32, i32);
struct State {
    coord: Coord,
    dir: Direction,
    waypoint: Coord,
}

fn shift((x, y): Coord, direction: Direction, distance: i32) -> Coord {
    match direction {
        Direction::North => (x, y + distance),
        Direction::East => (x + distance, y),
        Direction::South => (x, y - distance),
        Direction::West => (x - distance, y),
    }
}

fn left(direction: Direction, degrees: i32) -> Direction {
    let mut d = direction;
    let turns = degrees / 90;
    for _ in 0..turns {
        d = match d {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }
    return d;
}

fn right(direction: Direction, degrees: i32) -> Direction {
    let mut d = direction;
    let turns = degrees / 90;
    for _ in 0..turns {
        d = match d {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }
    return d;
}

fn simulate1(insructions: &Vec<Instruction>) -> Coord {
    let mut state = State {
        coord: (0, 0),
        dir: Direction::East,
        waypoint: (10, 1),
    };

    for &instr in insructions {
        match instr {
            Instruction::Shift(heading) => {
                state.coord = shift(state.coord, heading.direction, heading.distance)
            }
            Instruction::Forward(distance) => state.coord = shift(state.coord, state.dir, distance),
            Instruction::Left(degrees) => state.dir = left(state.dir, degrees),
            Instruction::Right(degrees) => state.dir = right(state.dir, degrees),
        }
    }

    state.coord
}

fn towards_waypoint((x, y): Coord, (x_shift, y_shift): Coord, multiplier: i32) -> Coord {
    (x + x_shift * multiplier, y + y_shift * multiplier)
}

fn clockwise((x, y): Coord, degrees: i32) -> Coord {
    match degrees {
        90 => (y, -x),
        180 => (-x, -y),
        270 => (-y, x),
        _ => panic!("Invalid Rotation"),
    }
}

fn counterclockwise((x, y): Coord, degrees: i32) -> Coord {
    match degrees {
        90 => (-y, x),
        180 => (-x, -y),
        270 => (y, -x),
        _ => panic!("Invalid Rotation"),
    }
}

fn simulate2(insructions: &Vec<Instruction>) -> Coord {
    let mut state = State {
        coord: (0, 0),
        dir: Direction::East,
        waypoint: (10, 1),
    };

    for &instr in insructions {
        match instr {
            Instruction::Shift(heading) => {
                state.waypoint = shift(state.waypoint, heading.direction, heading.distance)
            }
            Instruction::Forward(multiplier) => {
                state.coord = towards_waypoint(state.coord, state.waypoint, multiplier)
            }
            Instruction::Left(degrees) => {
                state.waypoint = counterclockwise(state.waypoint, degrees)
            }
            Instruction::Right(degrees) => state.waypoint = clockwise(state.waypoint, degrees),
        }
    }

    state.coord
}

fn main() -> std::io::Result<()> {
    let file = File::open("src/12/bin/input.txt")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    let instructions = parse_input(&contents);

    {
        let (final_x, final_y) = simulate1(&instructions);
        println!(
            "P1\nFinal Coords: ({}, {}). Distance: {}",
            final_x,
            final_y,
            final_x.abs() + final_y.abs()
        );
    }

    let (final_x, final_y) = simulate2(&instructions);
    println!(
        "\nP2\nFinal Coords: ({}, {}). Distance: {}",
        final_x,
        final_y,
        final_x.abs() + final_y.abs()
    );

    Ok(())
}

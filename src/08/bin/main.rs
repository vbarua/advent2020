use std::fs::File;
use std::io::BufReader;

use std::io::prelude::*;

#[derive(Debug)]
enum Instruction {
    ACC(i32),
    JMP(i32),
    NOP,
}

fn parse_instruction(input: &str) -> Instruction {
    match input {
        input if input.starts_with("nop") => Instruction::NOP,
        input if input.starts_with("acc") => {
            let value = input.split_terminator(" ").collect::<Vec<_>>()[1];
            Instruction::ACC(value.parse().expect("Expected a number"))
        }
        input if input.starts_with("jmp") => {
            let value = input.split_terminator(" ").collect::<Vec<_>>()[1];
            Instruction::JMP(value.parse().expect("Expected a number"))
        }
        _ => {
            println!("{}", input);
            panic!("Invalid Input")
        }
    }
}

fn compute(instructions: &Vec<Instruction>) -> i32 {
    let mut visited = vec![false; instructions.len()];
    let mut instruction_pointer: i32 = 0;
    let mut acc: i32 = 0;
    while !visited[instruction_pointer as usize] {
        visited[instruction_pointer as usize] = true;
        match instructions[instruction_pointer as usize] {
            Instruction::ACC(n) => {
                acc += n;
                instruction_pointer += 1;
            }
            Instruction::JMP(n) => {
                instruction_pointer += n;
            }
            Instruction::NOP => {
                instruction_pointer += 1;
            }
        }
    }
    acc
}

fn main() -> std::io::Result<()> {
    let file = File::open("src/08/bin/input.txt")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    let instructions: Vec<Instruction> = contents.lines().map(parse_instruction).collect();

    let p1 = compute(&instructions);
    println!("Part 1: {}", p1);

    Ok(())
}

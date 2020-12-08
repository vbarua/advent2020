use std::fs::File;
use std::io::BufReader;

use std::io::prelude::*;

#[derive(Debug, Clone, Copy)]
enum Instruction {
    ACC(i32),
    JMP(i32),
    NOP(i32),
}

fn parse_instruction(input: &str) -> Instruction {
    match input {
        input if input.starts_with("nop") => {
            let value = input.split_terminator(" ").collect::<Vec<_>>()[1];
            Instruction::NOP(value.parse().expect("Expected a number"))
        }
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

fn compute(instructions: &Vec<Instruction>) -> (i32, bool) {
    let mut visited = vec![false; instructions.len()];
    let mut instruction_pointer: i32 = 0;
    let mut acc: i32 = 0;
    while instruction_pointer < instructions.len() as i32 && !visited[instruction_pointer as usize]
    {
        visited[instruction_pointer as usize] = true;
        match instructions[instruction_pointer as usize] {
            Instruction::ACC(n) => {
                acc += n;
                instruction_pointer += 1;
            }
            Instruction::JMP(n) => {
                instruction_pointer += n;
            }
            Instruction::NOP(_) => {
                instruction_pointer += 1;
            }
        }
    }
    (acc, instruction_pointer == instructions.len() as i32)
}

fn mutate(instructions: &Vec<Instruction>) -> (i32, i32) {
    for (index, instr) in instructions.iter().enumerate() {
        match instr {
            Instruction::ACC(_) => continue,
            Instruction::NOP(n) => {
                let mut instrs = instructions.clone();
                instrs[index] = Instruction::JMP(*n);
                let (acc, terminated) = compute(&instrs);
                if terminated {
                    return (acc, index as i32);
                }
            }
            Instruction::JMP(n) => {
                let mut instrs = instructions.clone();
                instrs[index] = Instruction::NOP(*n);
                let (acc, terminated) = compute(&instrs);
                if terminated {
                    return (acc, index as i32);
                }
            }
        }
    }
    panic!("Part 2 Failed")
}

fn main() -> std::io::Result<()> {
    let file = File::open("src/08/bin/input.txt")?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents)?;

    let instructions: Vec<Instruction> = contents.lines().map(parse_instruction).collect();

    let (p1, _) = compute(&instructions);
    println!("Part 1: {}", p1);

    let (p2, instruction_index) = mutate(&instructions);
    println!(
        "Part 2: Terminated with {} after swapping {}",
        p2, instruction_index
    );

    Ok(())
}

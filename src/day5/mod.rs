use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::ops::Rem;
use itertools::Itertools;

pub fn day5_part1() {
    let mut cargo = get_cargo();

    for instruction in cargo.instructions {
        for _i in 0..instruction.amount {
            let krate = cargo.stacks[instruction.from - 1].pop().unwrap();
            cargo.stacks[instruction.to - 1].push(krate);
        }
    }

    cargo.stacks.iter()
        .map(|stack| stack.last().unwrap_or(&' '))
        .for_each(|c| print!("{}", c));
}

pub fn day5_part2() {
    let mut cargo = get_cargo();

    for instruction in cargo.instructions {
        let mut temp_stack: Vec<char> = Vec::new();
        for _i in 0..instruction.amount {
            let krate = cargo.stacks[instruction.from - 1].pop().unwrap();
            temp_stack.push(krate);
        }
        while !temp_stack.is_empty() {
            let krate = temp_stack.pop().unwrap();
            cargo.stacks[instruction.to - 1].push(krate);
        }
    }

    cargo.stacks.iter()
        .map(|stack| stack.last().unwrap_or(&' '))
        .for_each(|c| print!("{}", c));
}

#[derive(Debug)]
struct Instruction {
    amount: u8,
    from: usize,
    to: usize,
}

pub struct Cargo {
    stacks: Vec<Vec<char>>,
    instructions: Vec<Instruction>,
}

pub fn get_cargo() -> Cargo {
    let file = File::open("src/day5/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut stacks: Vec<Vec<char>> = Vec::new();
    let mut instructions: Vec<Instruction> = Vec::new();
    stacks.push(Vec::new());

    Lines::for_each(reader.lines(), |maybe_line| {
        let line = maybe_line.unwrap();

        if line.starts_with("move") {
            let mut splitted = line.split_whitespace();
            instructions.push(Instruction {
                amount: splitted.nth(1).unwrap().parse().unwrap(),
                from: splitted.nth(1).unwrap().parse().unwrap(),
                to: splitted.nth(1).unwrap().parse().unwrap(),
            });
            return;
        }

        if !line.contains('[') {
            return;
        }

        for mut i in 0..line.len() {
            let char = line.chars().nth(i).unwrap();
            if !char.is_alphabetic() {
                continue;
            }
            let stack_index = i / 4;
            while stack_index >= stacks.len() {
                stacks.push(Vec::new());
            }
            stacks[stack_index].push(char);
        }
    });

    for stack in &mut stacks {
        stack.reverse();
    }

    Cargo { stacks, instructions }
}

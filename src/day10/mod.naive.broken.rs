use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Rem;
use std::process::exit;
use std::cmp::max as math_max;

pub fn day10_part1() {
    let instructions = get_program();
    let relevant_cycles = vec![20, 60, 100, 140, 180, 220];

    let mut register_x = 1;
    let mut ip: usize = 0; // Instruction Pointer
    let mut cycle = 0;
    let mut cycles_till_next = 1;
    let mut signal_strength_sum = 0;

    while cycles_till_next > 0 {
        cycles_till_next -= 1;
        cycle += 1;

        let next_cycle = cycle + 1;
        if relevant_cycles.contains(&next_cycle) {
            println!("Adding {} * {}\t= {} to sum", next_cycle, register_x, register_x * next_cycle);
            signal_strength_sum += register_x * next_cycle;
        }

        if cycles_till_next > 0 {
            continue;
        }

        let instruction = &instructions[ip];
        register_x += instruction.arg;

        ip += 1;
        if ip < instructions.len() {
            cycles_till_next = get_command_cycles(&instruction.command);
        }
    }
    println!("Signal strength sum = {}", signal_strength_sum);
}

// Works for example input; doesn't work for actual input
pub fn day10_part2() {
    let instructions = get_program();
    let crt_width = 40;

    let mut register_x = 1;
    let mut ip: usize = 0; // Instruction Pointer
    let mut cycle: i32 = 0;
    let mut cycles_till_next = get_command_cycles(&instructions[ip].command);

    while cycles_till_next > 0 {
        cycles_till_next -= 1;
        cycle += 1;

        let horizontal_position = (cycle - 1) % crt_width;
        let sprite_range = register_x - 1..=register_x + 1;
        if sprite_range.contains(&horizontal_position) {
            print!("#");
        } else {
            print!(".");
        }
        if cycle % crt_width == 0 {
            println!();
        }

        if cycles_till_next > 0 {
            continue;
        }

        let instruction = &instructions[ip];
        register_x += instruction.arg;

        ip += 1;
        if ip < instructions.len() {
            cycles_till_next = get_command_cycles(&instruction.command);
        }
    }
}

fn get_command_cycles(command: &Command) -> u32 {
    match command {
        Command::Noop => 1,
        Command::AddX => 2,
    }
}

#[derive(Debug)]
enum Command {
    Noop,
    AddX,
}

#[derive(Debug)]
struct Instruction {
    command: Command,
    arg: i32,
}

fn get_program() -> Vec<Instruction> {
    let file = File::open("src/day10/input.txt").unwrap();
    let reader = BufReader::new(file);

    reader.lines()
        .map(|line| {
            let line = line.unwrap();
            let (command, arg) = line.split_at(4);
            let command = match command.trim() {
                "noop" => Command::Noop,
                "addx" => Command::AddX,
                _ => panic!("Unknown command: {}", command),
            };
            Instruction {
                command,
                arg: arg.trim().parse().unwrap_or(0),
            }
        })
        .collect()
}

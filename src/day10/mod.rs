use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Rem;
use std::process::exit;
use std::cmp::max as math_max;

pub fn day10_part1() {
    let mutations = get_program();
    let relevant_cycles = vec![20, 60, 100, 140, 180, 220];

    let mut register_x = 1;
    let mut cycle: i32 = 1;
    let mut signal_strength_sum = 0;

    for mutation in mutations {
        if relevant_cycles.contains(&cycle) {
            signal_strength_sum += register_x * cycle;
        }
        register_x += mutation;
        cycle += 1;
    }

    println!("Signal strength sum = {}", signal_strength_sum);
}

pub fn day10_part2() {
    let mutations = get_program();
    let crt_width = 40;

    let mut register_x = 1;
    let mut cycle: i32 = 1;

    for mutation in mutations {
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

        register_x += mutation;
        cycle += 1;
    }
}

fn get_program() -> Vec<i32> {
    let file = File::open("src/day10/input.txt").unwrap();
    let reader = BufReader::new(file);

    reader.lines()
        .flat_map(|line| {
            let line = line.unwrap();
            let (command, arg) = line.split_at(4);
            match command.trim() {
                "noop" => vec![0],
                "addx" => vec![0, arg.trim().parse::<i32>().unwrap()],
                _ => panic!("Unknown command: {}", command),
            }
        })
        .collect()
}

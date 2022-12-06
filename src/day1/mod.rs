use std::fs::{File};
use std::io::{BufRead, BufReader};

pub fn day1_part1() {
    let file = File::open("src/day1/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut max = 0;
    let mut total = 0;
    for line in reader.lines() {
        match line {
            Ok(line) => {
                if line.chars().all(|c| c == '\n') {
                    if total > max {
                        max = total;
                    }
                    total = 0;
                    continue;
                }
                let calories: u32 = line.parse().unwrap();
                total += calories;
            }
            Err(error) => eprintln!("{}", error)
        }
    }
    println!("Max {}", max);
}

pub fn day1_part1_v2() {
    let inventories = get_inventories();
    let max: Option<u32> = inventories.iter()
        .map(|v| v.iter().sum())
        .max();
    println!("Max {}", max.unwrap());
}

pub fn day1_part2() {
    let inventories = get_inventories();
    let mut sums: Vec<u32> = inventories.iter()
        .map(|v| v.iter().sum())
        .collect::<Vec<u32>>();
    sums.sort_by(|a, b| b.cmp(a)); // Reverse sort
    let total = sums[0] + sums[1] + sums[2];
    println!("{} + {} + {} = {}", sums[0], sums[1], sums[2], total);
}

fn get_inventories() -> Vec<Vec<u32>> {
    let file = File::open("src/day1/input.txt").unwrap();
    let reader = BufReader::new(file);

    let mut inventory: Vec<u32> = Vec::new();
    let mut inventories: Vec<Vec<u32>> = Vec::new();
    for line in reader.lines() {
        match line {
            Ok(line) => {
                // if line.len() == 0 {
                if line.chars().all(|c| c == '\n') {
                    inventories.push(inventory);
                    inventory = Vec::new();
                    continue;
                }
                let calories: u32 = line.parse().unwrap();
                inventory.push(calories);
            }
            Err(error) => eprintln!("{}", error)
        }
    }
    inventories
}

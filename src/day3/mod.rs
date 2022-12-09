use std::borrow::BorrowMut;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::Chars;
use itertools::{all, Itertools};

pub fn day3_part1() {
    let rucksacks = get_rucksacks();
    let sum: u32 = rucksacks.iter()
        .map(|rucksack| rucksack.one.chars().into_iter()
            // Skip any item that doesn't exist in the second compartment
            .skip_while(|item| !rucksack.two.chars().any(|c| c == *item))
            .take(1) // As soon as you DO find a matching item, take it
            .next()
            .unwrap()
        )
        .map(get_priority)
        .sum(); // .for_each(|priority| println!("Priority: {}", priority));
    println!("Sum: {}", sum);
}

pub fn day3_part2() {
    let rucksacks = get_rucksacks();

    let sum: u32 = rucksacks.iter()
        .chunks(3)
        .into_iter()
        .map(|mut group| {
            let mut unique_items = group.next().unwrap().all.chars().collect_vec();
            for rucksack in group {
                unique_items.retain(|item| rucksack.all.chars().any(|c| c == *item));
            }
            unique_items.first().unwrap().clone() // There should be only one item left
       })
        .map(get_priority)
        .sum(); // .for_each(|priority| println!("Priority: {}", priority));
    println!("Sum: {}", sum);
}

struct Rucksack {
    one: String,
    two: String,
    all: String,
}

// Just use the character's ascii value to determine priority
fn get_priority(item: char) -> u32 {
    return if item.is_ascii_lowercase() {
        item as u32 - 96
    } else {
        item as u32 - 64 + 26
    }
}

fn get_rucksacks() -> Vec<Rucksack> {
    let file = File::open("src/day3/input.txt").unwrap();
    let reader = BufReader::new(file);

    reader.lines()
        .map(|line| {
            let items = line.unwrap();
            let (one, two) = items.split_at(items.len() / 2);
            Rucksack {
                one: one.to_string(),
                two: two.to_string(),
                all: one.to_string() + two
            }
        })
        .collect()
}

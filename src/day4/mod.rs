use std::fmt::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::ops::Range;
use itertools::Itertools;

pub fn day4_part1() {
    let count: u32 = get_pairs().iter()
        .filter(|pair| (pair.elf1.start >= pair.elf2.start && pair.elf1.end <= pair.elf2.end)
            || (pair.elf2.start >= pair.elf1.start && pair.elf2.end <= pair.elf1.end))
        .count() as u32;
    println!("Count: {}", count);
}

pub fn day4_part2() {
    let count: u32 = get_pairs().iter()
        .filter(|pair| pair.elf1.start >= pair.elf2.start && pair.elf1.start <= pair.elf2.end
            || pair.elf1.end >= pair.elf2.start && pair.elf1.end <= pair.elf2.end
            || pair.elf2.start >= pair.elf1.start && pair.elf2.start <= pair.elf1.end
            || pair.elf2.end >= pair.elf1.start && pair.elf2.end <= pair.elf1.end)
        .count() as u32;
    println!("Count: {}", count);
}

#[derive(Debug)]
struct ElfPair {
    elf1: Range<u32>,
    elf2: Range<u32>,
}

fn get_pairs() -> Vec<ElfPair> {
    let file = File::open("src/day4/input.txt").unwrap();
    let reader = BufReader::new(file);

    reader.lines()
        .flat_map(|line| {
            let mut items = line.unwrap();
            let mut elves_split = items.split(',');
            let mut elf1_split = elves_split.next().map(|elf| elf.split('-'))?;
            let mut elf2_split = elves_split.next().map(|elf| elf.split('-'))?;
            let elf1: Range<u32> = elf1_split.next()?.parse().ok()?..elf1_split.next()?.parse().ok()?;
            let elf2: Range<u32> = elf2_split.next()?.parse().ok()?..elf2_split.next()?.parse().ok()?;
            Some(ElfPair { elf1, elf2 })
        })
        .collect()
}
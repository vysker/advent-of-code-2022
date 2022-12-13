use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use itertools::Itertools;

pub fn day6_part1() {
    let datastream = get_datastream();
    find_marker(4, datastream.as_str());
}

pub fn day6_part2() {
    let datastream = get_datastream();
    find_marker(14, datastream.as_str());
}

fn find_marker(marker_size: usize, datastream: &str) {
    let mut chars = datastream.chars();
    let mut buffer: Vec<char> = Vec::with_capacity(marker_size);

    for _ in 0..marker_size {
        buffer.push(chars.next().unwrap());
    }

    let mut buffer_index = 0;
    let mut characters_read = buffer.len();

    while let Some(char) = chars.next() {
        buffer[buffer_index] = char;
        buffer_index = (buffer_index + 1) % buffer.len();
        characters_read += 1;

        if buffer.iter().all_unique() {
            break;
        }
    }

    println!("{} characters read before marker found", characters_read);
    print!("Marker: ");
    for i in 0..buffer.len() {
        print!("{}", buffer[(buffer_index + i) % buffer.len()]);
    }
}

fn get_datastream() -> String {
    let file = File::open("src/day6/input.txt").unwrap();
    let mut reader = BufReader::new(file);

    let mut result = String::new();
    reader.read_line(&mut result).expect("Failed to read line");
    result
}
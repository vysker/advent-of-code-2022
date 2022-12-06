use std::collections::HashMap;
use std::fs::{File, read};
use std::io::{BufRead, BufReader};
use std::ops::Rem;

pub fn day2_part1() {
    let rounds = get_rounds();
    let total_score: i32 = rounds.iter()
        .map(|round| get_score(round.opponent, round.response))
        .sum();
    println!("Total score: {}", total_score);
}

pub fn day2_part2() {
    let rounds = get_rounds();
    let total_score: i32 = rounds.iter()
        .map(|round| {
            let response_ordinal = (round.opponent.ordinal() + round.response.ordinal() - 1).rem_euclid(3);
            let response = RPS::from_ordinal(response_ordinal).unwrap();
            get_score(round.opponent, response)
        })
        .sum();
    println!("Total score: {}", total_score);
}

#[derive(Debug, Clone, Copy, Ordinalize)]
enum RPS { ROCK, PAPER, SCISSORS }

struct Round {
    opponent: RPS,
    response: RPS
}

impl From<&str> for RPS {
    fn from(s: &str) -> Self {
        match s {
            "A" | "X" => RPS::ROCK,
            "B" | "Y" => RPS::PAPER,
            "C" | "Z" => RPS::SCISSORS,
            _ => panic!("Unknown value for RPS")
        }
    }
}

fn get_score(opponent: RPS, response: RPS) -> i32 {
    // In Rock, Paper, Scissors, any value always wins from the "previous" value. So we use
    // modulo for the wraparound. Also note that rem_euclid() is used here because '%' in
    // rust gives the remainder, not the actual modulo.
    let is_win = opponent.ordinal() == (response.ordinal() - 1).rem_euclid(3);
    let is_draw = opponent.ordinal() == response.ordinal();
    let base_score = (is_win as i8) * 6 + (is_draw as i8) * 3;
    (base_score + response.ordinal() + 1) as i32
}

fn get_rounds() -> Vec<Round> {
    let file = File::open("src/day2/input.txt").unwrap();
    let reader = BufReader::new(file);

    reader.lines()
        .map(|line| {
            let split: Vec<RPS> = line.unwrap().split_whitespace().map(RPS::from).collect();
            Round {
                opponent: split[0],
                response: split[1]
            }
        })
        .collect()
}

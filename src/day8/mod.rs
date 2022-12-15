use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn day8_part1() {
    let trees = get_tree_grid();

    let mut visible_count = 0;

    for row in 1..trees.len() - 1 {
        for col in 1..trees[row].len() - 1 {
            let mut blocks = 0;
            blocks += is_blocked_in_direction(row, col, -1, 0, &trees);
            blocks += is_blocked_in_direction(row, col, 1, 0, &trees);
            blocks += is_blocked_in_direction(row, col, 0, 1, &trees);
            blocks += is_blocked_in_direction(row, col, 0, -1, &trees);
            if blocks < 4 {
                visible_count += 1;
            }
        }
    }

    let trees_in_corner = 4;
    let trees_in_outer_ring = trees[0].len() * 2 + trees.len() * 2 - trees_in_corner;
    println!("Visible trees: {}", visible_count + trees_in_outer_ring);
}

pub fn day8_part2() {
    let trees = get_tree_grid();

    let mut best_score = 0;

    for row in 1..trees.len() - 1 {
        for col in 1..trees[row].len() - 1 {
            let mut score;
            score = visible_trees_in_direction(row, col, -1, 0, &trees);
            score *= visible_trees_in_direction(row, col, 1, 0, &trees);
            score *= visible_trees_in_direction(row, col, 0, 1, &trees);
            score *= visible_trees_in_direction(row, col, 0, -1, &trees);
            if score > best_score {
                best_score = score;
            }
        }
    }

    println!("Best score: {}", best_score);
}

fn is_blocked_in_direction(row: usize, col: usize, row_step: isize, col_step: isize, trees: &Vec<Vec<u32>>) -> u32 {
    let ref_tree = trees[row][col];
    let mut row = (row as isize + row_step) as usize;
    let mut col = (col as isize + col_step) as usize;

    while row < trees.len() && row >= 0
        && col < trees[row].len() && col >= 0
    {
        if trees[row][col] >= ref_tree {
            return 1;
        }
        row = (row as isize + row_step) as usize;
        col = (col as isize + col_step) as usize;
    }
    0
}

fn visible_trees_in_direction(row: usize, col: usize, row_step: isize, col_step: isize, trees: &Vec<Vec<u32>>) -> u32 {
    let ref_tree = trees[row][col];
    let mut row = (row as isize + row_step) as usize;
    let mut col = (col as isize + col_step) as usize;
    let mut count = 0;

    while row < trees.len() && row >= 0
        && col < trees[row].len() && col >= 0
    {
        count += 1;
        if trees[row][col] >= ref_tree {
            return count;
        }
        row = (row as isize + row_step) as usize;
        col = (col as isize + col_step) as usize;
    }
    count
}

fn get_tree_grid() -> Vec<Vec<u32>> {
    let file = File::open("src/day8/input.txt").unwrap();
    let mut reader = BufReader::new(file);

    let trees: Vec<Vec<u32>> = reader.lines()
        .map(|line| line.unwrap().chars()
            .map(|c| u32::from(c.to_digit(10).unwrap()))
            .collect())
        .collect();
    trees
}

use std::cmp::min as math_min;
use std::cmp::max as math_max;
use std::fs::File;
use std::io::{BufRead, BufReader};
use num::clamp;
use std::process::exit;
use itertools::min;

pub fn day9_part1() {
    let moves = get_moves();
    let mut tail: (i32, i32) = (0, 0);
    let mut visited_positions: Vec<(i32, i32)> = Vec::new();

    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut previous_head = tail;
    for move_ in &moves {
        let x_step = match move_.direction {
            Direction::Left => -1,
            Direction::Right => 1,
            _ => 0,
        };
        let y_step = match move_.direction {
            Direction::Up => 1,
            Direction::Down => -1,
            _ => 0,
        };
        for _ in 0..move_.distance {
            x += x_step;
            y += y_step;
            let head = (x, y);
            if get_distance(tail, head) > 1 {
                tail = previous_head;
            }
            if !visited_positions.contains(&tail) {
                visited_positions.push(tail);
            }
            previous_head = head;
        }
    }
    println!("Visited positions: {:?}", visited_positions.len());
    // print_visited_positions(visited_positions);
}

pub fn day9_part2() {
    let moves = get_moves();
    let mut visited_positions: Vec<(i32, i32)> = Vec::new();
    let mut knots = vec![Knot { x: 0 + 15, y: 0 + 15 }; 10];

    for move_ in &moves {
        let x_step = match move_.direction {
            Direction::Left => -1,
            Direction::Right => 1,
            _ => 0,
        };
        let y_step = match move_.direction {
            Direction::Up => 1,
            Direction::Down => -1,
            _ => 0,
        };

        for _ in 0..move_.distance {
            knots[0].x += x_step;
            knots[0].y += y_step;

            for i in 1..knots.len() {
                let knot = &knots[i];
                let front = &knots[i - 1];
                if get_distance_knot(knot, front) > 1 {
                    let step_x = clamp(front.x - knot.x, -1, 1);
                    let step_y = clamp(front.y - knot.y, -1, 1);
                    let x = knot.x + step_x;
                    let y = knot.y + step_y;
                    knots[i] = Knot { x, y };
                }
            }

            // print_grid(&knots, &move_);

            let tail: &Knot = &knots[knots.len() - 1];
            if !visited_positions.contains(&(tail.x, tail.y)) {
                visited_positions.push((tail.x, tail.y));
            }
        }
    }
    println!("Visited positions: {:?}", visited_positions.len());
    // print_visited_positions(visited_positions);
}

fn get_distance(a: (i32, i32), b: (i32, i32)) -> i32 {
    return if i32::abs(a.0 - b.0) > 1 || i32::abs(a.1 - b.1) > 1 {
        2
    } else {
        1
    };
}

// We could use manhattan distance here, but diagonals count for distance 2, so we'd have to
// take the root of the sum of squares, which is more expensive than just checking if the
// difference is greater than 1.
// Manhattan distance: i32::abs(a.0 - b.0) + i32::abs(a.1 - b.1)
fn get_distance_knot(a: &Knot, b: &Knot) -> i32 {
    return if i32::abs(a.x - b.x) > 1 || i32::abs(a.y - b.y) > 1 {
        2
    } else {
        1
    };
}

fn print_grid(knots: &Vec<Knot>, move_: &Move) {
    println!("== {:?} {} ==", move_.direction, move_.distance);
    let mut grid: Vec<Vec<char>> = vec![vec!['.'; 40]; 40];
    for (i, x, y) in knots.iter().enumerate().map(|(i, k)| (i, k.x, k.y)) {
        grid[y as usize][x as usize] = i.to_string().chars().next().unwrap();
    }
    grid.reverse();
    for row in grid {
        for col in row {
            print!("{}", col);
        }
        println!();
    }
}

fn print_visited_positions(visited_positions: Vec<(i32, i32)>) {
    let mut grid: Vec<Vec<char>> = vec![vec!['.'; 40]; 40];
    for (x, y) in visited_positions {
        grid[y as usize][x as usize] = '#';
    }
    grid.reverse();
    for row in grid {
        for col in row {
            print!("{}", col);
        }
        println!();
    }
}

#[derive(Clone, Debug)]
struct Knot {
    x: i32,
    y: i32,
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Move {
    direction: Direction,
    distance: u32,
}

fn get_moves() -> Vec<Move> {
    let file = File::open("src/day9/input.txt").unwrap();
    let reader = BufReader::new(file);

    reader.lines()
        .map(|line| {
            let line = line.unwrap();
            let (direction, distance) = line.split_at(2);
            let direction = match direction.trim() {
                "U" => Direction::Up,
                "D" => Direction::Down,
                "L" => Direction::Left,
                "R" => Direction::Right,
                _ => panic!("Unknown direction"),
            };
            Move {
                direction,
                distance: distance.parse().unwrap(),
            }
        })
        .collect()
}

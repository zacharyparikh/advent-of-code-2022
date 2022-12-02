use std::fs::read_to_string;

#[derive(PartialEq)]
enum Shape {
    ROCK,
    PAPER,
    SCISSORS,
}

use Shape::{PAPER, ROCK, SCISSORS};

fn get_shape(symbol: &str) -> Option<Shape> {
    match symbol {
        "A" | "X" => Some(ROCK),
        "B" | "Y" => Some(PAPER),
        "C" | "Z" => Some(SCISSORS),
        _ => None,
    }
}

fn get_outcome_points(opposing_symbol: &str, response_symbol: &str) -> u8 {
    let opposing = get_shape(opposing_symbol).unwrap();
    let response = get_shape(response_symbol).unwrap();
    if opposing == response {
        3
    } else if (opposing == ROCK && response == PAPER)
        || (opposing == PAPER && response == SCISSORS)
        || (opposing == SCISSORS && response == ROCK)
    {
        6
    } else {
        0
    }
}

pub fn part1() -> Option<u32> {
    let input = read_to_string("inputs/day2.txt").ok()?;
    Some(
        input
            .split("\n")
            .filter(|round| round.len() > 0)
            .map(|round| {
                let symbols: Vec<&str> = round.split(" ").collect();
                (symbols[0], symbols[1])
            })
            .map(|(opposing, response)| {
                let response_points = match response {
                    "X" => Some(1),
                    "Y" => Some(2),
                    "Z" => Some(3),
                    _ => None,
                };

                response_points.unwrap() + get_outcome_points(opposing, response) as u32
            })
            .sum(),
    )
}

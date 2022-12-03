use std::fs::read_to_string;

#[derive(PartialEq)]
enum Shape {
    ROCK,
    PAPER,
    SCISSORS,
}

use Shape::{PAPER, ROCK, SCISSORS};

#[derive(PartialEq)]
enum Outcome {
    WIN,
    DRAW,
    LOSE,
}

use Outcome::{DRAW, LOSE, WIN};

fn get_shape(symbol: &str) -> Option<Shape> {
    match symbol {
        "A" | "X" => Some(ROCK),
        "B" | "Y" => Some(PAPER),
        "C" | "Z" => Some(SCISSORS),
        _ => None,
    }
}

fn get_response_shape(opposing: &Shape, outcome: &Outcome) -> Shape {
    if (*opposing == ROCK && *outcome == DRAW)
        || (*opposing == PAPER && *outcome == LOSE)
        || (*opposing == SCISSORS && *outcome == WIN)
    {
        ROCK
    } else if (*opposing == ROCK && *outcome == WIN)
        || (*opposing == PAPER && *outcome == DRAW)
        || (*opposing == SCISSORS && *outcome == LOSE)
    {
        PAPER
    } else {
        SCISSORS
    }
}

fn get_outcome(opposing: &Shape, response: &Shape) -> Outcome {
    if *opposing == *response {
        DRAW
    } else if (*opposing == ROCK && *response == PAPER)
        || (*opposing == PAPER && *response == SCISSORS)
        || (*opposing == SCISSORS && *response == ROCK)
    {
        WIN
    } else {
        LOSE
    }
}

fn get_outcome_points(outcome: Outcome) -> u8 {
    match outcome {
        WIN => 6,
        DRAW => 3,
        LOSE => 0,
    }
}

pub fn part1() -> u32 {
    let input = read_to_string("inputs/day2.txt").unwrap();
    input
        .split("\n")
        .filter(|round| round.len() > 0)
        .map(|round| {
            let symbols: Vec<&str> = round.split(" ").collect();
            (symbols[0], symbols[1])
        })
        .map(|(opposing, response)| {
            let opposing_shape = get_shape(opposing).unwrap();
            let response_shape = get_shape(response).unwrap();
            let outcome = get_outcome(&opposing_shape, &response_shape);

            let response_points = match response_shape {
                ROCK => 1,
                PAPER => 2,
                SCISSORS => 3,
            };

            response_points + get_outcome_points(outcome) as u32
        })
        .sum()
}

pub fn part2() -> u32 {
    let input = read_to_string("inputs/day2.txt").unwrap();
    input
        .split("\n")
        .filter(|round| round.len() > 0)
        .map(|round| {
            let symbols: Vec<&str> = round.split(" ").collect();
            (symbols[0], symbols[1])
        })
        .map(|(opposing_symbol, outcome_symbol)| {
            let opposing_shape = get_shape(opposing_symbol).unwrap();
            let outcome = match outcome_symbol {
                "X" => Some(LOSE),
                "Y" => Some(DRAW),
                "Z" => Some(WIN),
                _ => None,
            };
            let response_shape = get_response_shape(&opposing_shape, outcome.as_ref().unwrap());

            let response_points = match response_shape {
                ROCK => 1,
                PAPER => 2,
                SCISSORS => 3,
            };

            response_points + get_outcome_points(outcome.unwrap()) as u32
        })
        .sum()
}

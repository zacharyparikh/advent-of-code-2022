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

fn part1(symbols: &Vec<(String, String)>) -> u32 {
    symbols
        .iter()
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

fn part2(symbols: &Vec<(String, String)>) -> u32 {
    symbols
        .iter()
        .map(|(opposing_symbol, outcome_symbol)| {
            let opposing_shape = get_shape(opposing_symbol).unwrap();
            let outcome = match outcome_symbol.as_str() {
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

pub fn solve() -> (u32, u32) {
    let input = read_to_string("inputs/day2.txt").unwrap();
    let symbols: Vec<(String, String)> = input
        .split("\n")
        .filter(|round| round.len() > 0)
        .map(|round| {
            let symbols: Vec<&str> = round.split(" ").collect();
            (symbols[0].to_owned(), symbols[1].to_owned())
        })
        .collect();

    (part1(&symbols), part2(&symbols))
}

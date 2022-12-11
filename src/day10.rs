use std::fs::read_to_string;

enum Instruction {
    Addx(i32),
    Noop,
}

use Instruction::{Addx, Noop};

fn part1(cycle_values: &Vec<i32>) -> i32 {
    let signal_strengths = (20..=220).step_by(40).map(|cycle: usize| {
        let value = cycle_values.get(cycle - 1).unwrap();
        value * (cycle as i32)
    });

    signal_strengths.sum()
}

fn part2(cycle_values: &Vec<i32>) -> String {
    cycle_values
        .iter()
        .enumerate()
        .map(|(index, value)| {
            let position: i32 = (index as i32) % 40;
            let mut pixel = if value.abs_diff(position) < 2 {
                String::from("#")
            } else {
                String::from(".")
            };

            if position == 39 {
                pixel.push('\n');
            }

            pixel
        })
        .collect()
}

pub fn solve() -> (i32, String) {
    let input = read_to_string("inputs/day10.txt").unwrap();
    let instructions: Vec<Instruction> = input
        .split_terminator("\n")
        .map(|instruction| {
            if instruction == "noop" {
                Noop
            } else {
                let addx_value: i32 = instruction.strip_prefix("addx ").unwrap().parse().unwrap();
                Addx(addx_value)
            }
        })
        .collect();

    let mut register_value = 1;
    let mut cycle_values: Vec<i32> = vec![register_value];
    instructions
        .iter()
        .for_each(|instruction| match instruction {
            Noop => {
                cycle_values.push(register_value);
            }
            Addx(value) => {
                cycle_values.push(register_value);
                register_value += value;
                cycle_values.push(register_value);
            }
        });
    (part1(&cycle_values), part2(&cycle_values))
}

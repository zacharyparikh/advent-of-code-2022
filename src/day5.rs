use std::{
    collections::{HashMap, VecDeque},
    fs::read_to_string,
};

use regex::Regex;

fn read_input() -> (HashMap<u8, VecDeque<String>>, Vec<String>) {
    let input = read_to_string("inputs/day5.txt").unwrap();
    let mut input_sections = input.split("\n\n");

    let crates: Vec<String> = input_sections
        .next()
        .unwrap()
        .split_terminator("\n")
        .filter(|line| line.chars().nth(0).unwrap() != ' ')
        .map(|line| line.to_owned())
        .collect();

    let instructions: Vec<String> = input_sections
        .next()
        .unwrap()
        .split_terminator("\n")
        .map(|line| line.to_owned())
        .collect();

    let item_rows = crates.iter().map(|line| {
        line.chars()
            .collect::<Vec<char>>()
            .chunks(4)
            .map(|item| String::from_iter(item).replace(&['[', ']', ' '], ""))
            .collect::<Vec<String>>()
    });

    let mut item_stacks: HashMap<u8, VecDeque<String>> = HashMap::new();

    for item_row in item_rows {
        for (index, item) in item_row.iter().enumerate() {
            if item.len() == 0 {
                continue;
            }

            let column: u8 = (index + 1).try_into().unwrap();

            match item_stacks.get_mut(&column) {
                Some(stack) => {
                    stack.push_front(item.to_owned());
                }
                None => {
                    item_stacks.insert(column, VecDeque::from([item.to_owned()]));
                }
            }
        }
    }

    (item_stacks, instructions)
}

fn move_items(
    item_stacks: &mut HashMap<u8, VecDeque<String>>,
    count: u8,
    from: u8,
    to: u8,
    in_order: bool,
) {
    if count == 0 {
        return;
    }

    let from_items = {
        let from_stack = item_stacks.get_mut(&from).unwrap();
        let items: VecDeque<String> = from_stack.split_off(from_stack.len() - (count as usize));

        if in_order {
            items
        } else {
            items.into_iter().rev().collect()
        }
    };

    let to_stack = item_stacks.get_mut(&to).unwrap();

    for item in from_items {
        to_stack.push_back(item);
    }
}

fn part1() -> String {
    let (mut item_stacks, instructions) = read_input();

    let instruction_re =
        Regex::new(r"move (?P<count>\d+) from (?P<from>\d+) to (?P<to>\d+)").unwrap();

    for instruction in instructions {
        let caps = instruction_re.captures(instruction.as_str()).unwrap();
        let count: u8 = caps["count"].parse().unwrap();
        let from: u8 = caps["from"].parse().unwrap();
        let to: u8 = caps["to"].parse().unwrap();
        move_items(&mut item_stacks, count, from, to, false)
    }

    (1..=item_stacks.len())
        .map(|column| {
            let stack = item_stacks.get(&(column as u8)).unwrap();
            stack.back().unwrap().to_owned()
        })
        .collect::<Vec<String>>()
        .join("")
}

fn part2() -> String {
    let (mut item_stacks, instructions) = read_input();

    let instruction_re =
        Regex::new(r"move (?P<count>\d+) from (?P<from>\d+) to (?P<to>\d+)").unwrap();

    for instruction in instructions {
        let caps = instruction_re.captures(instruction.as_str()).unwrap();
        let count: u8 = caps["count"].parse().unwrap();
        let from: u8 = caps["from"].parse().unwrap();
        let to: u8 = caps["to"].parse().unwrap();
        move_items(&mut item_stacks, count, from, to, true)
    }

    (1..=item_stacks.len())
        .map(|column| {
            let stack = item_stacks.get(&(column as u8)).unwrap();
            stack.back().unwrap().to_owned()
        })
        .collect::<Vec<String>>()
        .join("")
}

pub fn solve() -> (String, String) {
    (part1(), part2())
}

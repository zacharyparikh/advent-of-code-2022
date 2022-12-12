use std::{collections::HashMap, fs::read_to_string};

use regex::Regex;

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<u64>,
    num_items_inspected: u64,
    operation: String,
    test: u8,
    true_monkey: u8,
    false_monkey: u8,
}

fn inspect_items(operation: &String, items: &Vec<u64>) -> Vec<u64> {
    let mut split = operation.split(" ").skip(1);
    let operation_symbol = split.next().unwrap();
    let operand_symbol: Option<u64> = split.next().unwrap().parse().ok();

    items
        .iter()
        .map(|old| match operand_symbol {
            Some(operand) => match operation_symbol {
                "*" => old * operand,
                "+" => old + operand,
                _ => panic!(),
            },
            None => match operation_symbol {
                "*" => old * old,
                "+" => old + old,
                _ => panic!(),
            },
        })
        .collect::<Vec<u64>>()
}

fn throw_items(from_index: u8, items: &Vec<u64>, monkeys: &mut HashMap<u8, Monkey>) {
    let (test, true_monkey, false_monkey) = {
        let monkey = monkeys.get(&from_index).unwrap();
        (monkey.test, monkey.true_monkey, monkey.false_monkey)
    };

    items.iter().for_each(|worry_level| {
        let to_index = if worry_level % (test as u64) == 0 {
            true_monkey
        } else {
            false_monkey
        };

        let to_monkey = monkeys.get_mut(&to_index).unwrap();
        to_monkey.items.push(*worry_level);
    })
}

fn perform_rounds(rounds: u64, monkeys: &mut HashMap<u8, Monkey>, worried: bool) {
    let least_common_test_multiple: u64 =
        monkeys.values().map(|monkey| monkey.test as u64).product();
    if rounds == 0 {
        return;
    }

    (0..8).for_each(|index: u8| {
        let inspected_items: Vec<u64> = {
            let monkey = monkeys.get_mut(&index).unwrap();
            let items = inspect_items(&monkey.operation, &monkey.items);
            if worried {
                items
                    .iter()
                    .map(|worry_level| worry_level % least_common_test_multiple)
                    .collect()
            } else {
                items.iter().map(|worry_level| worry_level / 3).collect()
            }
        };

        monkeys.entry(index).and_modify(|monkey| {
            monkey.num_items_inspected += inspected_items.len() as u64;
            monkey.items.clear();
        });
        throw_items(index, &inspected_items, monkeys);
    });

    perform_rounds(rounds - 1, monkeys, worried);
}

fn get_monkey_business(monkeys: &HashMap<u8, Monkey>) -> u64 {
    let mut monkeys_num_items_inspected: Vec<u64> = monkeys
        .values()
        .map(|monkey| monkey.num_items_inspected)
        .collect();
    monkeys_num_items_inspected.sort();
    monkeys_num_items_inspected.reverse();

    monkeys_num_items_inspected.iter().take(2).product()
}

fn part1(monkeys: &mut HashMap<u8, Monkey>) -> u64 {
    perform_rounds(20, monkeys, false);
    get_monkey_business(monkeys)
}

fn part2(monkeys: &mut HashMap<u8, Monkey>) -> u64 {
    perform_rounds(10_000, monkeys, true);
    get_monkey_business(monkeys)
}

pub fn solve() -> (u64, u64) {
    let input = read_to_string("inputs/day11.txt").unwrap();
    let monkey_re = Regex::new(
        r"Monkey (?P<index>\d):\n\s+Starting items: (?P<items>.*)\n\s+Operation: new = (?P<operation>.*)\n\s+Test: divisible by (?P<test>\d+)\n\s+If true: throw to monkey (?P<true>\d)\n\s+If false: throw to monkey (?P<false>\d)",
    ).unwrap();
    let mut monkeys: HashMap<u8, Monkey> = HashMap::new();
    input.split("\n\n").for_each(|monkey_definition| {
        let caps = monkey_re.captures(monkey_definition).unwrap();
        let index: u8 = caps["index"].parse().unwrap();
        let items: Vec<u64> = caps["items"]
            .split(", ")
            .map(|worry_level| worry_level.parse::<u64>().unwrap())
            .collect();
        let operation = caps["operation"].to_owned();
        let test: u8 = caps["test"].to_owned().parse().unwrap();
        let true_monkey: u8 = caps["true"].parse().unwrap();
        let false_monkey: u8 = caps["false"].parse().unwrap();
        monkeys.insert(
            index,
            Monkey {
                items,
                num_items_inspected: 0,
                operation,
                test,
                true_monkey,
                false_monkey,
            },
        );
    });

    (part1(&mut monkeys.clone()), part2(&mut monkeys.clone()))
}

use std::{collections::HashSet, fs::read_to_string};

fn get_priority(item: &char) -> u16 {
    if item.is_uppercase() {
        let lower = 'A' as u16;
        (*item as u16) - lower + 27
    } else {
        let lower = 'a' as u16;
        (*item as u16) - lower + 1
    }
}

fn part1(rucksacks: &Vec<&str>) -> u16 {
    let common_items = rucksacks.iter().map(|rucksack| {
        let mid = rucksack.len() / 2;
        let (first_container, second_container) = rucksack.split_at(mid);
        let first_container_set: HashSet<char> = first_container.chars().collect();
        let second_container_set: HashSet<char> = second_container.chars().collect();
        let common_item = first_container_set
            .intersection(&second_container_set)
            .next()
            .unwrap()
            .to_owned();

        return common_item;
    });

    let priority_sum = common_items.map(|item| get_priority(&item)).sum();

    priority_sum
}

fn part2(rucksacks: &Vec<&str>) -> u16 {
    let group_badges = rucksacks.chunks(3).map(|group| {
        let intersection = group
            .iter()
            .map(|rucksack| HashSet::from_iter(rucksack.chars()))
            .reduce(|set1: HashSet<char>, set2: HashSet<char>| &set1 & &set2);

        let badge = intersection.unwrap().iter().next().unwrap().to_owned();
        return badge;
    });

    let priority_sum = group_badges.map(|badge| get_priority(&badge)).sum();
    priority_sum
}

pub fn solve() -> (u16, u16) {
    let input = read_to_string("inputs/day3.txt").unwrap();
    let rucksacks: Vec<&str> = input.split_terminator("\n").collect();
    (part1(&rucksacks), part2(&rucksacks))
}

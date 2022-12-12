use std::fs::read_to_string;

fn part1() -> u16 {
    0
}

fn part2() -> u16 {
    0
}

fn find_character_position(heightmap: &Vec<Vec<char>>, target: char) -> (usize, usize) {
    heightmap
        .iter()
        .enumerate()
        .find_map(|(row_index, row)| {
            if let Some(column_index) = row.iter().position(|letter| *letter == target) {
                Some((row_index, column_index))
            } else {
                None
            }
        })
        .unwrap()
}

pub fn solve() -> (u16, u16) {
    let input = read_to_string("inputs/day12.txt").unwrap();
    let heightmap: Vec<Vec<char>> = input
        .split_terminator("\n")
        .map(|row| row.chars().collect::<Vec<char>>())
        .collect();

    let start = find_character_position(&heightmap, 'S');
    let end = find_character_position(&heightmap, 'E');

    println!("{start:?}, {end:?}");
    (part1(), part2())
}

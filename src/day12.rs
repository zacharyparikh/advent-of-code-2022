use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

fn get_possible_steps(heightmap: &Vec<Vec<u8>>, position: (usize, usize)) -> Vec<(usize, usize)> {
    let row_index = position.0 as i8;
    let column_index = position.1 as i8;
    let up = (row_index - 1, column_index);
    let down = (row_index + 1, column_index);
    let left = (row_index, column_index - 1);
    let right = (row_index, column_index + 1);

    let current_elevation = heightmap.get(position.0).unwrap().get(position.1).unwrap();

    vec![up, down, left, right]
        .into_iter()
        .filter_map(|position| {
            if position.0 >= 0 && position.1 >= 0 {
                Some((position.0 as usize, position.1 as usize))
            } else {
                None
            }
        })
        .filter(|neighbor_position| {
            heightmap
                .get(neighbor_position.0)
                .and_then(|row| row.get(neighbor_position.1))
                .filter(|neighbor_elevation| {
                    (**neighbor_elevation as i8) - (*current_elevation as i8) < 2
                })
                .is_some()
        })
        .collect()
}

fn find_closest_unvisited(
    distances: &HashMap<(usize, usize), u16>,
    unvisited: &HashSet<(usize, usize)>,
) -> (usize, usize) {
    distances
        .iter()
        .filter(|entry| unvisited.contains(entry.0))
        .min_by_key(|entry| entry.1)
        .unwrap()
        .0
        .clone()
}

fn find_shortest_path(
    current: (usize, usize),
    end: (usize, usize),
    heightmap: &Vec<Vec<u8>>,
    unvisited: &mut HashSet<(usize, usize)>,
    distances: &mut HashMap<(usize, usize), u16>,
) -> u16 {
    if !unvisited.contains(&end) {
        return distances.get(&end).unwrap().clone();
    }

    let possible_steps = get_possible_steps(heightmap, current);
    let current_distance = distances.get(&current).unwrap().clone();

    possible_steps.iter().for_each(|position| {
        if let Some(neighbor_distance) = distances.get(position) {
            if *neighbor_distance < current_distance + 1 {
                return;
            }
        }

        distances.insert(*position, current_distance + 1);
    });

    unvisited.remove(&current);
    let next = find_closest_unvisited(distances, unvisited);
    find_shortest_path(next, end, heightmap, unvisited, distances)
}

fn part1(heightmap: &Vec<Vec<u8>>, start: (usize, usize), end: (usize, usize)) -> u16 {
    let mut unvisited: HashSet<(usize, usize)> =
        HashSet::from_iter((0..heightmap.len()).flat_map(|row_index| {
            let row = heightmap.get(row_index).unwrap();
            (0..row.len()).map(move |column_index| (row_index, column_index))
        }));
    let mut distances: HashMap<(usize, usize), u16> = HashMap::new();
    distances.insert(start, 0);

    find_shortest_path(start, end, &heightmap, &mut unvisited, &mut distances)
}

fn part2(heightmap: &Vec<Vec<u8>>, start: (usize, usize), end: (usize, usize)) -> u16 {
    let mut unvisited: HashSet<(usize, usize)> =
        HashSet::from_iter((0..heightmap.len()).flat_map(|row_index| {
            let row = heightmap.get(row_index).unwrap();
            (0..row.len()).map(move |column_index| (row_index, column_index))
        }));
    let mut distances: HashMap<(usize, usize), u16> = HashMap::new();

    heightmap
        .iter()
        .enumerate()
        .flat_map(|(row_index, row)| {
            row.iter()
                .enumerate()
                .filter(|entry| *entry.1 == 0)
                .map(move |entry| (row_index, entry.0))
        })
        .for_each(|position| {
            distances.insert(position, 0);
        });

    find_shortest_path(start, end, &heightmap, &mut unvisited, &mut distances)
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
    let heightmap_chars: Vec<Vec<char>> = input
        .split_terminator("\n")
        .map(|row| row.chars().collect::<Vec<char>>())
        .collect();

    let start = find_character_position(&heightmap_chars, 'S');
    let end = find_character_position(&heightmap_chars, 'E');

    let heightmap: Vec<Vec<u8>> = heightmap_chars
        .iter()
        .map(|row| {
            row.iter()
                .map(|symbol| {
                    if *symbol == 'S' {
                        0
                    } else if *symbol == 'E' {
                        ('z' as u8) - ('a' as u8)
                    } else {
                        (*symbol as u8) - ('a' as u8)
                    }
                })
                .collect()
        })
        .collect();
    (part1(&heightmap, start, end), part2(&heightmap, start, end))
}

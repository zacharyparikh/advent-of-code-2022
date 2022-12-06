use std::{
    collections::{HashSet, VecDeque},
    fs::read_to_string,
    str::Chars,
};

fn find_aux(
    mut stream: Chars,
    mut last_chars: VecDeque<char>,
    current_index: usize,
    num_chars: usize,
) -> Option<usize> {
    let current = stream.next()?;
    last_chars.push_back(current);

    if last_chars.len() == num_chars {
        let char_set: HashSet<&char> = HashSet::from_iter(last_chars.iter());
        if char_set.len() == num_chars {
            return Some(current_index);
        } else {
            last_chars.pop_front();
        }
    }

    find_aux(stream, last_chars, current_index + 1, num_chars)
}

fn find_start_of_packet(stream: Chars) -> usize {
    find_aux(stream, VecDeque::new(), 1, 4).unwrap()
}

fn find_start_of_message(stream: Chars) -> usize {
    find_aux(stream, VecDeque::new(), 1, 14).unwrap()
}

pub fn part1() -> usize {
    let input = read_to_string("inputs/day6.txt").unwrap();
    find_start_of_packet(input.chars())
}

pub fn part2() -> usize {
    let input = read_to_string("inputs/day6.txt").unwrap();
    find_start_of_message(input.chars())
}

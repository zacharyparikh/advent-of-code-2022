use std::{
    collections::{HashSet, VecDeque},
    fs::read_to_string,
    str::Chars,
};

fn find_aux(mut stream: Chars, num_chars: usize) -> usize {
    let mut last_chars = VecDeque::new();
    let index = stream
        .position(|current| {
            last_chars.push_back(current);

            if last_chars.len() == num_chars {
                let char_set: HashSet<&char> = HashSet::from_iter(last_chars.iter());
                if char_set.len() == num_chars {
                    return true;
                } else {
                    last_chars.pop_front();
                }
            }

            false
        })
        .unwrap();

    index + 1
}

fn find_start_of_packet(stream: Chars) -> usize {
    find_aux(stream, 4)
}

fn find_start_of_message(stream: Chars) -> usize {
    find_aux(stream, 14)
}

pub fn part1() -> usize {
    let input = read_to_string("inputs/day6.txt").unwrap();
    find_start_of_packet(input.chars())
}

pub fn part2() -> usize {
    let input = read_to_string("inputs/day6.txt").unwrap();
    find_start_of_message(input.chars())
}

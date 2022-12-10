use std::{collections::HashSet, fs::read_to_string, vec};

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

use Direction::{Down, Left, Right, Up};

#[derive(Debug)]
struct Motion {
    direction: Direction,
    distance: u8,
}

fn next_position<'a>(
    rope: &'a mut Vec<(i32, i32)>,
    direction: &Direction,
) -> &'a mut Vec<(i32, i32)> {
    let mut head: Option<&mut (i32, i32)> = None;

    rope.iter_mut().for_each(|knot| {
        match &head {
            Some(position) => {
                let x_diff = position.0 - knot.0;
                let y_diff = position.1 - knot.1;
                let x_distance = x_diff.abs();
                let y_distance = y_diff.abs();

                *knot = if x_distance + y_distance < 2 || (x_distance == 1 && y_distance == 1) {
                    *knot
                } else if x_distance == 2 && y_distance == 1 {
                    (knot.0 + x_diff / 2, knot.1 + y_diff)
                } else if y_distance == 2 && x_distance == 1 {
                    (knot.0 + x_diff, knot.1 + y_diff / 2)
                } else {
                    (knot.0 + x_diff / 2, knot.1 + y_diff / 2)
                };
            }
            None => {
                *knot = match direction {
                    Up => (knot.0, knot.1 + 1),
                    Down => (knot.0, knot.1 - 1),
                    Left => (knot.0 - 1, knot.1),
                    Right => (knot.0 + 1, knot.1),
                };
            }
        };

        head = Some(knot);
    });

    rope
}

fn perform_motion(rope: &mut Vec<(i32, i32)>, motion: &Motion) -> Vec<(i32, i32)> {
    let mut tail_positions: Vec<(i32, i32)> = Vec::new();

    fn step(rope: &mut Vec<(i32, i32)>, tail_positions: &mut Vec<(i32, i32)>, motion: &Motion) {
        if motion.distance == 0 {
            return;
        }

        next_position(rope, &motion.direction);
        let next_tail = rope.last().unwrap();

        tail_positions.push(*next_tail);

        step(
            rope,
            tail_positions,
            &Motion {
                distance: motion.distance - 1,
                ..*motion
            },
        )
    }

    step(rope, &mut tail_positions, motion);

    tail_positions
}

fn part1(motions: &Vec<Motion>) -> usize {
    let start = (0, 0);
    let mut rope = vec![start; 2];
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert(start);

    motions.iter().for_each(|motion| {
        let tail_positions = perform_motion(&mut rope, motion);

        tail_positions.into_iter().for_each(|position| {
            visited.insert(position);
        });
    });

    visited.len()
}

fn part2(motions: &Vec<Motion>) -> usize {
    let start = (0, 0);
    let mut rope = vec![start; 10];

    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    visited.insert(start);

    motions.iter().for_each(|motion| {
        let tail_positions = perform_motion(&mut rope, motion);

        tail_positions.into_iter().for_each(|position| {
            visited.insert(position);
        });
    });

    visited.len()
}

pub fn solve() -> (usize, usize) {
    let input = read_to_string("inputs/day9.txt").unwrap();
    let motions: Vec<Motion> = input
        .split_terminator("\n")
        .map(|motion| {
            let mut split = motion.split(" ");
            let direction = split.next().unwrap();
            let distance: u8 = split.next().unwrap().parse().unwrap();

            match direction {
                "U" => Motion {
                    direction: Up,
                    distance,
                },
                "D" => Motion {
                    direction: Down,
                    distance,
                },
                "L" => Motion {
                    direction: Left,
                    distance,
                },
                "R" => Motion {
                    direction: Right,
                    distance,
                },
                _ => panic!(),
            }
        })
        .collect();

    (part1(&motions), part2(&motions))
}

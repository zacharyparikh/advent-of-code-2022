use std::fs::read_to_string;

struct Tree {
    height: u8,
    row_index: usize,
    column_index: usize,
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}
use Direction::{Down, Left, Right, Up};

fn next_index(
    row_index: usize,
    column_index: usize,
    direction: &Direction,
) -> Option<(usize, usize)> {
    match direction {
        Up => {
            if row_index == 0 {
                None
            } else {
                Some((row_index - 1, column_index))
            }
        }
        Down => Some((row_index + 1, column_index)),
        Left => {
            if column_index == 0 {
                None
            } else {
                Some((row_index, column_index - 1))
            }
        }
        Right => Some((row_index, column_index + 1)),
    }
}

fn get_heights_in_direction(
    row_index: usize,
    column_index: usize,
    tree_grid: &Vec<Vec<u8>>,
    direction: &Direction,
) -> Vec<u8> {
    let mut heights = Vec::new();

    fn step<'a>(
        acc: &'a mut Vec<u8>,
        index: Option<(usize, usize)>,
        tree_grid: &Vec<Vec<u8>>,
        direction: &Direction,
    ) -> &'a mut Vec<u8> {
        if index.is_none() {
            return acc;
        }

        let (row_index, column_index) = index.unwrap();

        if let Some(row) = tree_grid.get(row_index) {
            if let Some(height) = row.get(column_index) {
                acc.push(height.clone());

                return step(
                    acc,
                    next_index(row_index, column_index, direction),
                    tree_grid,
                    direction,
                );
            }
        }

        acc
    }

    step(
        &mut heights,
        next_index(row_index, column_index, direction),
        tree_grid,
        direction,
    );

    heights
}

fn is_visible(tree_grid: &Vec<Vec<u8>>, tree: &Tree) -> bool {
    let is_visible_in_direction = |direction: Direction| -> bool {
        let heights =
            get_heights_in_direction(tree.row_index, tree.column_index, tree_grid, &direction);
        heights.into_iter().all(|height| tree.height > height)
    };

    is_visible_in_direction(Up)
        || is_visible_in_direction(Down)
        || is_visible_in_direction(Left)
        || is_visible_in_direction(Right)
}

fn part1(tree_grid: &Vec<Vec<u8>>, trees: impl Iterator<Item = Tree>) -> u16 {
    trees.filter(|tree| is_visible(&tree_grid, tree)).count() as u16
}

fn get_scenic_score(tree_grid: &Vec<Vec<u8>>, tree: &Tree) -> u32 {
    let count_visible_in_direction = |direction: Direction| -> u32 {
        let heights =
            get_heights_in_direction(tree.row_index, tree.column_index, tree_grid, &direction);
        let blocked_position = heights.iter().position(|height| tree.height <= *height);

        (match blocked_position {
            Some(position) => position + 1,
            None => heights.len(),
        }) as u32
    };

    count_visible_in_direction(Up)
        * count_visible_in_direction(Down)
        * count_visible_in_direction(Left)
        * count_visible_in_direction(Right)
}

fn part2(tree_grid: &Vec<Vec<u8>>, trees: impl Iterator<Item = Tree>) -> u32 {
    trees
        .map(|tree| get_scenic_score(tree_grid, &tree))
        .max()
        .unwrap()
}

pub fn solve() -> (u16, u32) {
    let input = read_to_string("inputs/day8.txt").unwrap();
    let tree_grid: Vec<Vec<u8>> = input
        .split_terminator("\n")
        .map(|line| {
            line.chars()
                .map(|height| height.to_digit(10).unwrap() as u8)
                .collect::<Vec<u8>>()
        })
        .collect();

    let trees = tree_grid.iter().enumerate().flat_map(|(row_index, row)| {
        row.iter()
            .enumerate()
            .map(move |(column_index, height)| Tree {
                height: height.clone(),
                row_index,
                column_index,
            })
    });

    (
        part1(&tree_grid, trees.clone()),
        part2(&tree_grid, trees.clone()),
    )
}

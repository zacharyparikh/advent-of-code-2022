use std::fs::read_to_string;

fn part1(sums: &Vec<i32>) -> i32 {
    sums.first().unwrap().clone()
}

fn part2(sums: &Vec<i32>) -> i32 {
    sums.iter().take(3).sum()
}

pub fn solve() -> (i32, i32) {
    let input = read_to_string("inputs/day1.txt").unwrap();
    let mut sums: Vec<i32> = input
        .split("\n\n")
        .map(|calories| -> i32 {
            calories
                .split("\n")
                .filter_map(|calorie| calorie.parse::<i32>().ok())
                .sum()
        })
        .collect();

    sums.sort();
    sums.reverse();

    (part1(&sums), part2(&sums))
}

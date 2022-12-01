use std::error::Error;
use std::fs::read_to_string;

fn sort_calorie_sums() -> Result<Vec<i32>, Box<dyn Error>> {
    let input = read_to_string("inputs/day1.txt")?;

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
    Ok(sums)
}

thread_local!(static SORTED_CALORIE_SUMS: Vec<i32> = sort_calorie_sums().unwrap());

pub fn part1() -> i32 {
    SORTED_CALORIE_SUMS.with(|sums| sums.first().unwrap().clone())
}

pub fn part2() -> i32 {
    SORTED_CALORIE_SUMS.with(|sums| sums.iter().take(3).sum())
}

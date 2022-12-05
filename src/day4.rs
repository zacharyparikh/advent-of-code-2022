use std::fs::read_to_string;

pub fn part1() -> usize {
    let input = read_to_string("inputs/day4.txt").unwrap();
    let overlapping_assignments = input
        .split_terminator("\n")
        .filter(|pair| {
            let mut ranges = pair.split(",").map(|range| -> (u8, u8) {
                let mut nums = range.split("-");
                let lower = nums.next().unwrap();
                let upper = nums.next().unwrap();
                (lower.parse().unwrap(), upper.parse().unwrap())
            });

            let (lower1, upper1) = ranges.next().unwrap();
            let (lower2, upper2) = ranges.next().unwrap();

            (lower1 <= lower2 && upper1 >= upper2) || (lower2 <= lower1 && upper2 >= upper1)
        })
        .count();

    return overlapping_assignments;
}

pub fn part2() -> usize {
    let input = read_to_string("inputs/day4.txt").unwrap();
    let overlapping_assignments = input
        .split_terminator("\n")
        .filter(|pair| {
            let mut ranges = pair.split(",").map(|range| -> (u8, u8) {
                let mut nums = range.split("-");
                let lower = nums.next().unwrap();
                let upper = nums.next().unwrap();
                (lower.parse().unwrap(), upper.parse().unwrap())
            });

            let (lower1, upper1) = ranges.next().unwrap();
            let (lower2, upper2) = ranges.next().unwrap();

            (upper1 >= lower2 && lower1 <= upper2)
                || (upper2 >= lower1 && lower2 <= upper1)
                || (lower1 <= upper2 && upper1 >= lower2)
                || (lower2 <= upper1 && upper2 >= lower1)
        })
        .count();

    return overlapping_assignments;
}

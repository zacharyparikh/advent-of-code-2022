use std::fs::read_to_string;

pub fn part1(ranges: &Vec<((u8, u8), (u8, u8))>) -> usize {
    ranges
        .iter()
        .filter(|range| {
            let ((lower1, upper1), (lower2, upper2)) = range;

            (lower1 <= lower2 && upper1 >= upper2) || (lower2 <= lower1 && upper2 >= upper1)
        })
        .count()
}

pub fn part2(ranges: &Vec<((u8, u8), (u8, u8))>) -> usize {
    ranges
        .iter()
        .filter(|range| {
            let ((lower1, upper1), (lower2, upper2)) = range;

            (upper1 >= lower2 && lower1 <= upper2)
                || (upper2 >= lower1 && lower2 <= upper1)
                || (lower1 <= upper2 && upper1 >= lower2)
                || (lower2 <= upper1 && upper2 >= lower1)
        })
        .count()
}

pub fn solve() -> (usize, usize) {
    let input = read_to_string("inputs/day4.txt").unwrap();
    let ranges: Vec<((u8, u8), (u8, u8))> = input
        .split_terminator("\n")
        .map(|pair| {
            let mut ranges = pair.split(",").map(|range| -> (u8, u8) {
                let mut nums = range.split("-");
                let lower = nums.next().unwrap();
                let upper = nums.next().unwrap();
                (lower.parse().unwrap(), upper.parse().unwrap())
            });

            let range1 = ranges.next().unwrap();
            let range2 = ranges.next().unwrap();

            (range1, range2)
        })
        .collect();

    (part1(&ranges), part2(&ranges))
}

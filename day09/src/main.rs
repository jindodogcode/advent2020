//! --- Day 9: Encoding Error ---
//! https://adventofcode.com/2020/day/9

const INPUT: &str = include_str!("../../inputs/day09.txt");

fn main() {
    println!("Part one: {}", part_one(INPUT));
}

fn part_one(input: &str) -> i64 {
    let iter = input.lines().map(|s| s.parse::<i64>().unwrap());
    todo!()
}

fn not_two_sum_of_prev_n(nums: impl Iterator<Item = i64>, n: usize) -> i64 {
    todo!()
}

#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn part_one_works() {
        todo!()
    }

    #[test]
    fn part_two_works() {
        todo!()
    }
}

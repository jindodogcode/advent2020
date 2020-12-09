//! --- Day 6: Custom Customs ---
//! https://adventofcode.com/2020/day/6

use std::collections::HashSet;

const INPUT: &str = include_str!("../input/day06.txt");

fn main() {
    println!("Part one: {}", part_one(INPUT));
    println!("Part two: {}", part_two(INPUT));
}

fn part_one(input: &str) -> u64 {
    let mut total = 0;
    let mut group_ans = HashSet::new();
    for line in input.lines() {
        if line == "" {
            total += group_ans.len();
            group_ans.clear();
            continue;
        }

        line.chars().for_each(|c| {
            group_ans.insert(c);
        });
    }
    total += group_ans.len();

    total as u64
}

fn part_two(input: &str) -> u64 {
    let mut total = 0;
    let mut group_ans = HashSet::new();
    let mut fresh = true;

    for line in input.lines() {
        if line == "" {
            total += group_ans.len();
            group_ans.clear();
            fresh = true;
            continue;
        }

        if fresh {
            group_ans = line.chars().collect();
        } else {
            group_ans = group_ans
                .intersection(&line.chars().collect::<HashSet<char>>())
                .copied()
                .collect();
        }
        fresh = false;
    }
    total += group_ans.len();

    total as u64
}

#[cfg(test)]
mod test {
    use crate::*;

    const INPUT: &str = "\
abc

a
b
c

ab
ac

a
a
a
a

b";

    #[test]
    fn part_one_works() {
        let res = part_one(INPUT);
        assert_eq!(11, res);
    }

    #[test]
    fn part_two_works() {
        let res = part_two(INPUT);
        assert_eq!(6, res);
    }
}

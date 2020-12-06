//! https://adventofcode.com/2020/day/1

use std::{
    cmp::Ordering,
    collections::HashSet,
    io::{self, BufRead},
};

fn main() -> io::Result<()> {
    println!("Part One result: {}", part_one(INPUT)?);
    println!("Part Two result: {}", part_two(INPUT)?);

    Ok(())
}

const INPUT: &str = include_str!("../input/day01.txt");

fn part_one(input: &str) -> io::Result<i32> {
    let target = 2020;
    let mut nums = HashSet::new();

    for line in io::Cursor::new(input).lines() {
        let num = line?.parse::<i32>().expect("Bad input: not a number");
        nums.insert(num);
    }
    let nums = nums; // No longer needs to be mutable

    for num in nums.iter() {
        let rem = target - num;

        if let Some(v) = nums.get(&rem) {
            return Ok(num * v);
        }
    }

    panic!("Input did not contain two values whose sum is {}", target);
}

fn part_two(input: &str) -> io::Result<i32> {
    let target = 2020;
    let mut nums = Vec::new();

    for line in io::Cursor::new(input).lines() {
        let num = line?.parse::<i32>().expect("Bad input: not a number");
        nums.push(num);
    }
    nums.sort_unstable();
    let nums = nums;
    for i in 0..nums.len() {
        let res = sorted_two_sum(&nums[i + 1..], target, nums[i]);

        if let Some((x, y, z)) = res {
            return Ok(x * y * z);
        }
    }

    panic!("Input did not contain three values whose sum is {}", target);
}

fn sorted_two_sum(nums: &[i32], target: i32, value: i32) -> Option<(i32, i32, i32)> {
    let mut left = 0;
    let mut right = nums.len() - 1;

    while left < right {
        let sum = value + nums[left] + nums[right];

        match sum.cmp(&target) {
            Ordering::Less => left += 1,
            Ordering::Greater => right -= 1,
            Ordering::Equal => return Some((value, nums[left], nums[right])),
        }
    }

    None
}

#[cfg(test)]
mod test {
    use crate::*;

    const INPUT: &str = "\
1721
979
366
299
675
1456";

    #[test]
    fn part_one_works() {
        let result = part_one(INPUT).unwrap();
        assert_eq!(514579, result);
    }

    #[test]
    fn part_two_works() {
        let result = part_two(INPUT).unwrap();
        assert_eq!(241861950, result);
    }
}

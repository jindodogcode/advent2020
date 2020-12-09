//! --- Day 5: Binary Boarding ---
//! https://adventofcode.com/2020/day/5

use std::collections::HashSet;

const INPUT: &str = include_str!("../../inputs/day05.txt");

fn main() {
    println!("Part one: {}", part_one(INPUT));
    println!("Part two: {}", part_two(INPUT));
}

fn part_one(input: &str) -> u64 {
    let rows = 128;
    let cols = 8;
    let mut max_seat_id = 0;

    for pass in input.lines() {
        let seat_id = find_seat_id(pass, rows, cols);
        max_seat_id = if seat_id > max_seat_id {
            seat_id
        } else {
            max_seat_id
        };
    }

    max_seat_id
}

fn part_two(input: &str) -> u64 {
    let rows = 128;
    let cols = 8;
    let mut max_seat_id = 0;
    let mut taken_seats = HashSet::new();

    for pass in input.lines() {
        let seat_id = find_seat_id(pass, rows, cols);
        taken_seats.insert(seat_id);
        max_seat_id = if seat_id > max_seat_id {
            seat_id
        } else {
            max_seat_id
        };
    }

    for i in 8..(max_seat_id - 8) {
        if taken_seats.get(&i).is_none()
            && taken_seats.get(&(i - 1)).is_some()
            && taken_seats.get(&(i + 1)).is_some()
        {
            return i;
        }
    }

    panic!("Can't find your seat");
}

fn find_seat_id(boarding_pass: &str, rows: u64, cols: u64) -> u64 {
    let row_directions = boarding_pass.chars().take(7);
    let mut low = 0;
    let mut high = rows - 1;
    for dir in row_directions {
        let mid = (high - low) / 2 + low;
        match dir {
            'F' => {
                high = mid;
            }
            'B' => {
                low = mid + 1;
            }
            _ => panic!("Bad Input: must be 'F' or 'B'"),
        }
    }
    let row = (high - low) / 2 + low;

    let col_directions = boarding_pass.chars().skip(7);
    let mut low = 0;
    let mut high = cols - 1;
    for dir in col_directions {
        let mid = (high - low) / 2 + low;
        match dir {
            'L' => {
                high = mid;
            }
            'R' => {
                low = mid + 1;
            }
            _ => panic!("Bad Input: must be 'L' or 'R'"),
        }
    }
    let col = (high - low) / 2 + low;

    row * 8 + col
}

#[cfg(test)]
mod test {
    use crate::*;

    const INPUT: &str = "\
BFFFBBFRRR
FFFBBBFRRR
BBFFBBFRLL";

    #[test]
    fn part_one_works() {
        let res = part_one(INPUT);
        assert_eq!(820, res);
    }
}

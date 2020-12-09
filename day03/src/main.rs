//! --- Day 3: Toboggan Trajectory ---
//! https://adventofcode.com/2020/day/3

use std::error::Error;

const INPUT: &str = include_str!("../../inputs/day03.txt");

fn main() -> Result<(), Box<dyn Error>> {
    println!("Part one: {}", part_one(INPUT)?);
    println!("Part two: {}", part_two(INPUT)?);

    Ok(())
}

fn part_one(input: &str) -> Result<u64, Box<dyn Error>> {
    let width = input.lines().next().unwrap().chars().count();

    Ok(count_trees(input, 3, 1, width)?)
}

fn part_two(input: &str) -> Result<u64, Box<dyn Error>> {
    let width = input.lines().next().unwrap().chars().count();

    let slope1 = count_trees(input, 1, 1, width)?;
    let slope2 = count_trees(input, 3, 1, width)?;
    let slope3 = count_trees(input, 5, 1, width)?;
    let slope4 = count_trees(input, 7, 1, width)?;
    let slope5 = count_trees(input, 1, 2, width)?;

    Ok(slope1 * slope2 * slope3 * slope4 * slope5)
}

fn count_trees(
    input: &str,
    x_step: usize,
    y_step: usize,
    width: usize,
) -> Result<u64, Box<dyn Error>> {
    let mut count = 0;
    let mut x = 0;
    for line in input.lines().step_by(y_step) {
        let pos = x % width;
        if line.chars().nth(pos).unwrap() == '#' {
            count += 1;
        }

        x += x_step;
    }

    Ok(count)
}

#[cfg(test)]
mod test {
    use crate::*;

    const INPUT: &str = "\
..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";

    #[test]
    fn part_one_works() {
        let res = part_one(INPUT).unwrap();
        assert_eq!(7, res);
    }

    #[test]
    fn part_two_works() {
        let res = part_two(INPUT).unwrap();
        assert_eq!(336, res);
    }
}

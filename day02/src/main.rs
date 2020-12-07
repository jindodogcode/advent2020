//! --- Day 2: Password Philosophy ---
//! https://adventofcode.com/2020/day/2

use std::error::Error;

const INPUT: &str = include_str!("../input/day02.txt");

fn main() -> Result<(), Box<dyn Error>> {
    println!("Part one: {}", part_one(INPUT)?);
    println!("Part two: {}", part_two(INPUT)?);

    Ok(())
}

fn part_one(input: &str) -> Result<i32, Box<dyn Error>> {
    let mut count = 0;

    for line in input.lines() {
        let (min, max, p, pw) = parse_line(&line)?;

        let matches = pw.chars().filter(|&c| c == p).count();
        if matches >= min && matches <= max {
            count += 1;
        }
    }

    Ok(count)
}

fn part_two(input: &str) -> Result<i32, Box<dyn Error>> {
    let mut count = 0;
    for line in input.lines() {
        let (n1, n2, p, pw) = parse_line(&line)?;
        let p1_match = pw.chars().nth(n1 - 1).unwrap() == p;
        let p2_match = pw.chars().nth(n2 - 1).unwrap() == p;
        if (p1_match && !p2_match) || (!p1_match && p2_match) {
            count += 1;
        }
    }

    Ok(count)
}

fn parse_line(line: &str) -> Result<(usize, usize, char, &str), Box<dyn Error>> {
    let mut line = line.split(' ');
    let mut min_max = line.next().unwrap().split('-');
    let min = min_max.next().unwrap().parse::<usize>()?;
    let max = min_max.next().unwrap().parse::<usize>()?;
    let p = line.next().unwrap().trim_end_matches(':').parse::<char>()?;
    let pw = line.next().unwrap();

    Ok((min, max, p, pw))
}

#[cfg(test)]
mod test {
    use crate::*;

    const INPUT: &str = "\
1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc";

    #[test]
    fn part_one_works() {
        let res = part_one(INPUT).unwrap();
        assert_eq!(2, res);
    }

    #[test]
    fn part_two_works() {
        let res = part_two(INPUT).unwrap();
        assert_eq!(1, res);
    }
}

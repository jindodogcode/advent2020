//! --- Day 4: Passport Processing ---
//! https://adventofcode.com/2020/day/4

use std::fmt::Debug;

const INPUT: &str = include_str!("../../inputs/day04.txt");

fn main() {
    println!("Part one: {}", validate::<PassportTypeOne>(INPUT));
    println!("Part two: {}", validate::<PassportTypeTwo>(INPUT));
}

fn validate<T: Passport + Debug + Default>(input: &str) -> u64 {
    let mut count = 0;
    let mut passport = T::default();

    for line in input.lines() {
        if line == "" {
            if passport.is_valid() {
                count += 1;
            }
            passport = T::default();
            continue;
        }

        for field in line.split(' ') {
            let field = field.trim();
            let mut kv = field.split(':');
            let key = kv.next().unwrap();
            let value = kv.next().unwrap();

            match key {
                "byr" => *passport.birth_year_mut() = Some(value.to_owned()),
                "iyr" => *passport.issue_year_mut() = Some(value.to_owned()),
                "eyr" => *passport.expiration_year_mut() = Some(value.to_owned()),
                "hgt" => *passport.height_mut() = Some(value.to_owned()),
                "hcl" => *passport.hair_color_mut() = Some(value.to_owned()),
                "ecl" => *passport.eye_color_mut() = Some(value.to_owned()),
                "pid" => *passport.passport_id_mut() = Some(value.to_owned()),
                "cid" => *passport.country_id_mut() = Some(value.to_owned()),
                _ => (),
            }
        }
    }

    if passport.is_valid() {
        count += 1;
    }

    count
}

trait Passport: Default {
    fn is_valid(&self) -> bool;
    fn birth_year_mut(&mut self) -> &mut Option<String>;
    fn issue_year_mut(&mut self) -> &mut Option<String>;
    fn expiration_year_mut(&mut self) -> &mut Option<String>;
    fn height_mut(&mut self) -> &mut Option<String>;
    fn hair_color_mut(&mut self) -> &mut Option<String>;
    fn eye_color_mut(&mut self) -> &mut Option<String>;
    fn passport_id_mut(&mut self) -> &mut Option<String>;
    fn country_id_mut(&mut self) -> &mut Option<String>;
}

#[derive(Debug, Default)]
struct PassportTypeOne {
    birth_year: Option<String>,
    issue_year: Option<String>,
    expiration_year: Option<String>,
    height: Option<String>,
    hair_color: Option<String>,
    eye_color: Option<String>,
    passport_id: Option<String>,
    country_id: Option<String>,
}

impl Passport for PassportTypeOne {
    fn is_valid(&self) -> bool {
        self.birth_year.is_some()
            && self.issue_year.is_some()
            && self.expiration_year.is_some()
            && self.height.is_some()
            && self.hair_color.is_some()
            && self.eye_color.is_some()
            && self.passport_id.is_some()
    }

    fn birth_year_mut(&mut self) -> &mut Option<String> {
        &mut self.birth_year
    }

    fn issue_year_mut(&mut self) -> &mut Option<String> {
        &mut self.issue_year
    }

    fn expiration_year_mut(&mut self) -> &mut Option<String> {
        &mut self.expiration_year
    }

    fn height_mut(&mut self) -> &mut Option<String> {
        &mut self.height
    }

    fn hair_color_mut(&mut self) -> &mut Option<String> {
        &mut self.hair_color
    }

    fn eye_color_mut(&mut self) -> &mut Option<String> {
        &mut self.eye_color
    }

    fn passport_id_mut(&mut self) -> &mut Option<String> {
        &mut self.passport_id
    }

    fn country_id_mut(&mut self) -> &mut Option<String> {
        &mut self.country_id
    }
}

#[derive(Debug, Default)]
struct PassportTypeTwo {
    birth_year: Option<String>,
    issue_year: Option<String>,
    expiration_year: Option<String>,
    height: Option<String>,
    hair_color: Option<String>,
    eye_color: Option<String>,
    passport_id: Option<String>,
    country_id: Option<String>,
}

impl Passport for PassportTypeTwo {
    fn is_valid(&self) -> bool {
        self.valid_birth_year()
            && self.valid_issue_year()
            && self.valid_expiration_year()
            && self.valid_height()
            && self.valid_hair_color()
            && self.valid_eye_color()
            && self.valid_passport_id()
    }

    fn birth_year_mut(&mut self) -> &mut Option<String> {
        &mut self.birth_year
    }

    fn issue_year_mut(&mut self) -> &mut Option<String> {
        &mut self.issue_year
    }

    fn expiration_year_mut(&mut self) -> &mut Option<String> {
        &mut self.expiration_year
    }

    fn height_mut(&mut self) -> &mut Option<String> {
        &mut self.height
    }

    fn hair_color_mut(&mut self) -> &mut Option<String> {
        &mut self.hair_color
    }

    fn eye_color_mut(&mut self) -> &mut Option<String> {
        &mut self.eye_color
    }

    fn passport_id_mut(&mut self) -> &mut Option<String> {
        &mut self.passport_id
    }

    fn country_id_mut(&mut self) -> &mut Option<String> {
        &mut self.country_id
    }
}

impl PassportTypeTwo {
    fn valid_birth_year(&self) -> bool {
        Self::valid_year(&self.birth_year, 1920, 2002)
    }

    fn valid_issue_year(&self) -> bool {
        Self::valid_year(&self.issue_year, 2010, 2020)
    }

    fn valid_expiration_year(&self) -> bool {
        Self::valid_year(&self.expiration_year, 2020, 2030)
    }

    fn valid_height(&self) -> bool {
        if let Some(height) = &self.height {
            match height.split_at(height.len() - 2) {
                (h, "cm") => {
                    let h = h.parse::<u8>().unwrap();
                    h >= 150 && h <= 193
                }
                (h, "in") => {
                    let h = h.parse::<u8>().unwrap();
                    h >= 59 && h <= 76
                }
                _ => false,
            }
        } else {
            false
        }
    }

    fn valid_hair_color(&self) -> bool {
        if let Some(color) = &self.hair_color {
            let mut chars = color.chars();
            if chars.next().unwrap() != '#' {
                return false;
            }
            chars.all(|c| matches!(c, 'a'..='f' | '0'..='9'))
        } else {
            false
        }
    }

    fn valid_eye_color(&self) -> bool {
        if let Some(color) = &self.eye_color {
            matches!(
                color.as_str(),
                "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth"
            )
        } else {
            false
        }
    }

    fn valid_passport_id(&self) -> bool {
        if let Some(id) = &self.passport_id {
            if id.chars().count() != 9 {
                return false;
            }

            id.chars().all(|d| matches!(d, '0'..='9'))
        } else {
            false
        }
    }

    fn valid_year(year: &Option<String>, min: u16, max: u16) -> bool {
        if let Some(year) = &year {
            let year = year.parse::<u16>().unwrap();
            year >= min && year <= max
        } else {
            false
        }
    }
}

#[cfg(test)]
mod test {
    use crate::*;

    const INPUT: &str = "\
ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

    const INVALID_INPUT: &str = "\
eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007";

    const VALID_INPUT: &str = "\
pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";

    #[test]
    fn part_one_works() {
        let res = validate::<PassportTypeOne>(INPUT);
        assert_eq!(2, res);
    }

    #[test]
    fn part_two_works() {
        let invalid_res = validate::<PassportTypeTwo>(INVALID_INPUT);
        let valid_res = validate::<PassportTypeTwo>(VALID_INPUT);

        assert_eq!(0, invalid_res);
        assert_eq!(4, valid_res);
    }
}

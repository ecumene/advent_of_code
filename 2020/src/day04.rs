use aoc_runner_derive::{aoc, aoc_generator};
use std::collections::HashMap;

pub struct Passport(HashMap<String, String>);

const REQ_KEYS: [&str; 7] = ["byr", "ecl", "eyr", "hcl", "hgt", "iyr", "pid"];

impl Passport {
    fn has_valid_range(&self, key: &str, lower: usize, upper: usize) -> bool {
        let value = self.0[key].parse().unwrap_or(0);
        lower <= value && value <= upper
    }

    fn has_valid_height(&self) -> bool {
        let height = self.0["hgt"].as_str();
        let value = height[..height.len() - 2].parse().unwrap_or(0);
        if height.ends_with("cm") {
            150 <= value && value <= 193
        } else {
            59 <= value && value <= 76
        }
    }

    fn has_valid_hair_color(&self) -> bool {
        let color = self.0["hcl"].as_str();

        color.len() == 7 && {
            let mut iter = color.bytes();
            iter.next() == Some(b'#') && iter.all(|x| x.is_ascii_hexdigit())
        }
    }

    fn has_valid_eye_color(&self) -> bool {
        let color = self.0["ecl"].as_str();

        ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&color)
    }

    fn has_valid_pid(&self) -> bool {
        let pid = self.0["pid"].as_str();

        pid.len() == 9 && pid.bytes().all(|x| x.is_ascii_digit())
    }

    fn has_valid_keys(&self) -> bool {
        REQ_KEYS.iter().all(|&key| self.0.contains_key(key))
    }

    fn is_valid(&self) -> bool {
        self.has_valid_keys()
            && self.has_valid_range("byr", 1920, 2002)
            && self.has_valid_range("iyr", 2010, 2020)
            && self.has_valid_range("eyr", 2020, 2030)
            && self.has_valid_height()
            && self.has_valid_hair_color()
            && self.has_valid_eye_color()
            && self.has_valid_pid()
    }
}

#[aoc_generator(day4)]
pub fn parse_input(input: &str) -> Option<Vec<Passport>> {
    input
        .split("\n\n")
        .map(|section| {
            section
                .split(&[' ', '\n'][..])
                .map(|field| {
                    let mut iter = field.split(':');
                    iter.next()
                        .map(|s| s.to_owned())
                        .and_then(|first| iter.next().map(|s| (first, s.to_owned())))
                })
                .collect::<Option<_>>()
                .map(Passport)
        })
        .collect()
}

#[aoc(day4, part1)]
fn solve_part1(input: &[Passport]) -> usize {
    input.iter().filter(|&pass| pass.has_valid_keys()).count()
}

#[aoc(day4, part2)]
fn solve_part2(input: &[Passport]) -> usize {
    input.iter().filter(|&pass| pass.is_valid()).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mixed_passports() {
        let input = "\
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
        let input = parse_input(input).unwrap();

        assert_eq!(solve_part1(input.as_slice()), 2);
    }

    #[test]
    fn invalid_passports() {
        let input = "\
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
        let input = parse_input(input).unwrap();

        assert_eq!(solve_part2(input.as_slice()), 0);
    }

    #[test]
    fn valid_passports() {
        let input = "\
pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";
        let input = parse_input(input).unwrap();

        assert_eq!(solve_part1(input.as_slice()), 4);
        assert_eq!(solve_part2(input.as_slice()), 4);
    }
}

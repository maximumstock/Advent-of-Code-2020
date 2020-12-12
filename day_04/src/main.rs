use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt");
    let result = part1(&input).expect("Error in part 1");
    println!("Part 1: {}", result);
    let result = part2(&input).expect("Error in part 2");
    println!("Part 2: {}", result);
}

fn part1(input: &str) -> Result<usize, ()> {
    let raw_passports = parse_input(&input);
    let required_fields = &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    let valid_passports = raw_passports
        .iter()
        .filter(|s| required_fields.iter().all(|field| s.contains_key(field)))
        .count();
    Ok(valid_passports)
}

fn part2(input: &str) -> Result<usize, ()> {
    let required_fields = &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    Ok(parse_input(&input)
        .iter()
        .filter(|s| required_fields.iter().all(|field| s.contains_key(field)))
        .filter(|x| validate_passport(&x))
        .count())
}

fn parse_input(input: &str) -> Vec<HashMap<&str, &str>> {
    input
        .split_terminator("\n\n")
        .map(parse_passport)
        .collect::<Vec<_>>()
}

fn parse_passport(raw_passport: &str) -> HashMap<&str, &str> {
    let tokens = raw_passport.split_whitespace().flat_map(|x| x.split(':'));
    let keys = tokens.clone().step_by(2);
    let values = tokens.skip(1).step_by(2);
    keys.zip(values).collect::<HashMap<&str, &str>>()
}

fn validate_passport(passport: &HashMap<&str, &str>) -> bool {
    passport.iter().all(|(&key, value)| match key {
        "byr" => (1920..=2002).contains(&value.parse().unwrap_or(0)),
        "iyr" => (2010..=2020).contains(&value.parse().unwrap_or(0)),
        "eyr" => (2020..=2030).contains(&value.parse().unwrap_or(0)),
        "hgt" => {
            let height = &value[0..value.len() - 2].parse().unwrap_or(0);
            match &value[value.len() - 2..] {
                "cm" => (150..=193).contains(height),
                "in" => (59..=76).contains(height),
                _ => false,
            }
        }
        "hcl" => {
            value.len() == 7
                && value.starts_with('#')
                && value.chars().skip(1).all(|c| c.is_ascii_hexdigit())
        }
        "ecl" => ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(value),
        "pid" => value.len() == 9 && value.chars().all(|c| c.is_ascii_digit()),
        "cid" => true,
        _ => unreachable!(),
    })
}

#[cfg(test)]
mod day04_test {
    use crate::{part1, part2};

    #[test]
    fn test_part_1() {
        assert_eq!(part1(include_str!("../input.txt")).unwrap(), 200);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part2(include_str!("../input.txt")).unwrap(), 116);
    }

    #[test]
    fn test_part_2_valid_passports() {
        let test_input = "
            pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
            hcl:#623a2f

            eyr:2029 ecl:blu cid:129 byr:1989
            iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

            hcl:#888785
            hgt:164cm byr:2001 iyr:2015 cid:88
            pid:545766238 ecl:hzl
            eyr:2022

            iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719
        ";
        assert_eq!(part2(test_input).unwrap(), 4);
    }

    #[test]
    fn test_part_2_invalid_passports() {
        let test_input = "
            eyr:1972 cid:100
            hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

            iyr:2019
            hcl:#602927 eyr:1967 hgt:170cm
            ecl:grn pid:012533040 byr:1946

            hcl:dab227 iyr:2012
            ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

            hgt:59cm ecl:zzz
            eyr:2038 hcl:74454a iyr:2023
            pid:3556412378 byr:2007
        ";
        assert_eq!(part2(test_input).unwrap(), 0);
    }
}

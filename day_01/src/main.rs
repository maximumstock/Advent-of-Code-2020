use std::collections::HashSet;

fn main() {
    let input = parse_input(&(include_str!("../input.txt")));
    let (a, b) = part1(&input, 2020).expect("No matching tuple in input");
    println!("Part 1: Tuple is ({}, {}) -> Product: {}", a, b, a * b);

    let (a, b, c) = part2(&input, 2020).expect("No matching tuple in input");
    println!(
        "Part 2: Tuple is ({}, {}, {}) -> Product: {}",
        a,
        b,
        c,
        a * b * c
    );
}

fn part1(input: &HashSet<i32>, target: i32) -> Result<(i32, i32), ()> {
    for i in input.iter() {
        let complementary = target - *i;
        if complementary < 0 {
            continue;
        }
        if input.contains(&complementary) {
            return Ok((*i, complementary));
        }
    }
    Err(())
}

fn part2(input: &HashSet<i32>, target: i32) -> Result<(i32, i32, i32), ()> {
    for i in input.iter() {
        for k in input.iter() {
            if input.contains(&(target - *i - *k)) {
                return Ok((*i, *k, target - *i - *k));
            }
        }
    }
    Err(())
}

fn parse_input(input: &str) -> HashSet<i32> {
    input
        .split_whitespace()
        .map(|i| i.parse::<i32>().unwrap())
        .collect()
}

#[cfg(test)]
mod day01_test {
    use crate::{parse_input, part1, part2};

    #[test]
    fn test_part_1() {
        let input = parse_input(include_str!("../input.txt"));
        let res = part1(&input, 2020).unwrap();
        assert_eq!(res, (211, 1809));
        assert_eq!(res.0 * res.1, 381699);
    }

    #[test]
    fn test_part_2() {
        let input = parse_input(include_str!("../input.txt"));
        let res = part2(&input, 2020).unwrap();
        assert_eq!(res, (395, 198, 1427));
        assert_eq!(res.0 * res.1 * res.2, 111605670);
    }
}

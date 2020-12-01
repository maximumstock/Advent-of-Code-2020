fn main() {
    let (a, b) = part1(2020).expect("No matching tuple in input");
    println!("Part 1: Tuple is ({}, {}) -> Product: {}", a, b, a * b);

    let (a, b, c) = part2(2020).expect("No matching tuple in input");
    println!(
        "Part 2: Tuple is ({}, {}, {}) -> Product: {}",
        a,
        b,
        c,
        a * b * c
    );
}

fn part1(target: i32) -> Result<(i32, i32), ()> {
    let input = read_input();
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

fn part2(target: i32) -> Result<(i32, i32, i32), ()> {
    let input = read_input();
    for i in input.iter() {
        for k in input.iter() {
            if input.contains(&(target - *i - *k)) {
                return Ok((*i, *k, target - *i - *k));
            }
        }
    }
    Err(())
}

fn read_input() -> Vec<i32> {
    include_str!("../input.txt")
        .split_whitespace()
        .map(|i| i.parse::<i32>().unwrap())
        .collect()
}

#[cfg(test)]
mod test {
    use crate::{part1, part2};

    #[test]
    fn test_part_1() {
        let res = part1(2020).unwrap();
        assert_eq!(res, (211, 1809));
        assert_eq!(res.0 * res.1, 381699);
    }

    #[test]
    fn test_part_2() {
        let res = part2(2020).unwrap();
        assert_eq!(res, (395, 198, 1427));
        assert_eq!(res.0 * res.1 * res.2, 111605670);
    }
}

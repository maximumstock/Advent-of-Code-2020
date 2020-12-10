fn main() {
    let input = include_str!("../input.txt");

    let result = part1(&input, 25).expect("Error in part 1");
    println!("Part 1: {}", result);

    let (min, max) = part2(&input, result).expect("Error in part 2");
    println!("Part 2: {}", min + max);
}

fn part1(input: &str, window: usize) -> Result<usize, ()> {
    let numbers = parse_input(&input);
    let mut start = 0;

    while start + window < numbers.len() {
        let n_to_check = numbers[start + window];
        let preamble = &numbers[start..start + window];

        if !check_preamble_contains(preamble, n_to_check) {
            return Ok(n_to_check);
        }
        start += 1;
    }

    Err(())
}

fn check_preamble_contains(preamble: &[usize], n: usize) -> bool {
    for x in preamble {
        if *x > n {
            continue;
        }

        let n_to_find = n - *x;
        if preamble.contains(&&n_to_find) && n_to_find != *x {
            return true;
        }
    }
    false
}

fn part2(input: &str, n: usize) -> Result<(usize, usize), ()> {
    let numbers = parse_input(&input);
    let mut start = 0;
    let mut end = 1;

    while end < numbers.len() {
        let slice = &numbers[start..end];
        let sum: usize = slice.iter().sum();

        match sum.cmp(&n) {
            std::cmp::Ordering::Equal => {
                return Ok((*slice.iter().min().unwrap(), *slice.iter().max().unwrap()));
            }
            std::cmp::Ordering::Greater => {
                start += 1;
                end = start + 1;
                continue;
            }
            std::cmp::Ordering::Less => end += 1,
        }
    }

    Err(())
}

fn parse_input(input: &str) -> Vec<usize> {
    input.lines().map(|x| x.parse::<usize>().unwrap()).collect()
}

#[cfg(test)]
mod test {
    use crate::{part1, part2};

    #[test]
    fn test_part_1() {
        assert_eq!(part1(include_str!("../input.txt"), 25).unwrap(), 177777905);
    }

    #[test]
    fn test_part_2() {
        let (min, max) = part2(include_str!("../input.txt"), 177777905).unwrap();
        assert_eq!(min + max, 23463012);
    }
}

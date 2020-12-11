use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt");

    let result = part1(&input).expect("Error in part 1");
    println!("Part 1: {}", result);

    let result = part2(&input).expect("Error in part 2");
    println!("Part 2: {}", result);
}

fn part1(input: &str) -> Result<usize, ()> {
    let numbers = parse_input(&input);
    let (ones, threes) =
        numbers
            .iter()
            .zip(numbers.iter().skip(1))
            .fold((1, 1), |(ones, threes), (a, b)| match b - a {
                3 => (ones, threes + 1),
                1 => (ones + 1, threes),
                _ => unreachable!(),
            });

    Ok(ones * threes)
}

fn part2(input: &str) -> Result<usize, ()> {
    let numbers = parse_input(&input);

    let mut distances = HashMap::new();
    distances.insert(0, 1);

    for x in &numbers {
        let n_paths = (1..=3)
            .map(|d| {
                x.checked_sub(d)
                    .and_then(|x| distances.get(&x))
                    .unwrap_or(&0)
            })
            .sum::<usize>();
        distances.insert(*x, n_paths);
    }

    Ok(*distances.get(numbers.last().unwrap()).unwrap())
}

fn parse_input(input: &str) -> Vec<u8> {
    let mut numbers: Vec<u8> = input.lines().map(|l| l.parse().unwrap()).collect();
    numbers.sort_unstable();
    numbers
}

#[cfg(test)]
mod test {
    use crate::{part1, part2};

    #[test]
    fn test_part_1() {
        assert_eq!(part1(include_str!("../input.txt")).unwrap(), 2664);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            part2(include_str!("../input.txt")).unwrap(),
            148098383347712
        );
    }
}

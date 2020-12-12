use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../input.txt");

    let result = part1(&input).expect("Error in part 1");
    println!("Part 1: {}", result);

    let result = part2(&input).expect("Error in part 2");
    println!("Part 2: {}", result);
}

fn part1(input: &str) -> Result<usize, ()> {
    Ok(input
        .split_terminator("\n\n")
        .map(|group| {
            group
                .lines()
                .flat_map(|line| line.chars())
                .collect::<HashSet<_>>()
                .len()
        })
        .sum())
}

fn part2(input: &str) -> Result<usize, ()> {
    Ok(input
        .split_terminator("\n\n")
        .map(|group| {
            let group_size = group.lines().count();
            group
                .lines()
                .flat_map(|line| line.chars())
                .fold(HashMap::<char, usize>::new(), |mut counts, c| {
                    match counts.get_mut(&c) {
                        Some(count) => *count += 1,
                        None => {
                            counts.insert(c, 1);
                        }
                    }
                    counts
                })
                .iter()
                .filter(|(_, count)| **count == group_size)
                .count()
        })
        .sum())
}

#[cfg(test)]
mod day06_test {
    use crate::{part1, part2};

    #[test]
    fn test_part_1() {
        assert_eq!(part1(include_str!("../input.txt")).unwrap(), 7110);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part2(include_str!("../input.txt")).unwrap(), 3628);
    }
}

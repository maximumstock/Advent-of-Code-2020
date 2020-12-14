use std::{
    collections::{HashMap, VecDeque},
    vec,
};

fn main() {
    let input = include_str!("../input.txt");

    let result = part1(&input).expect("Error in part 1");
    println!("Part 1: {}", result);

    let result = part2(&input).expect("Error in part 2");
    println!("Part 2: {}", result);
}

fn part1(input: &str) -> Result<usize, ()> {
    let mut memory = HashMap::<usize, usize>::new();
    let operations = parse_input(&input);

    let mut and_mask = 0;
    let mut or_mask = 0;

    for operation in operations {
        match operation {
            Operation::Mask(m) => {
                and_mask = usize::from_str_radix(&m.replace("X", "1"), 2).unwrap();
                or_mask = usize::from_str_radix(&m.replace("X", "0"), 2).unwrap();
            }
            Operation::Assignment((address, value)) => {
                memory.insert(address, value & and_mask | or_mask);
            }
        }
    }

    Ok(memory.values().sum())
}

fn part2(input: &str) -> Result<usize, ()> {
    let mut memory = HashMap::<usize, usize>::new();
    let operations = parse_input(&input);

    let mut mask_variations = vec![];

    for operation in operations {
        match operation {
            Operation::Mask(m) => {
                mask_variations = find_mask_variations(&m);
            }
            Operation::Assignment((address, value)) => {
                for (or_mask, and_mask) in &mask_variations {
                    memory.insert(address & and_mask | or_mask, value);
                }
            }
        }
    }

    Ok(memory.values().sum())
}

fn find_mask_variations(mask: &str) -> Vec<(usize, usize)> {
    let mut queue = VecDeque::new();
    let mut masks = vec![];

    queue.push_back(mask.replace('0', "N").replace('1', "E"));

    while let Some(remainder) = queue.pop_front() {
        if remainder.contains('X') {
            let x = remainder.replacen("X", "1", 1);
            let y = remainder.replacen("X", "0", 1);
            queue.push_front(x);
            queue.push_front(y);
        } else {
            let or_mask =
                usize::from_str_radix(&remainder.replace('E', "1").replace('N', "0"), 2).unwrap();
            let and_mask =
                usize::from_str_radix(&remainder.replace('E', "0").replace('N', "1"), 2).unwrap();
            masks.push((or_mask, and_mask));
        }
    }

    masks
}

enum Operation<'a> {
    Mask(&'a str),
    Assignment((usize, usize)),
}

fn parse_input(input: &str) -> Vec<Operation> {
    input
        .lines()
        .map(|line| {
            if line.starts_with("mask") {
                let mask = line.split_whitespace().nth(2).unwrap();
                Operation::Mask(mask)
            } else {
                let mut parts = line.split(" = ");
                let address = parts.next().unwrap();
                Operation::Assignment((
                    address[4..address.len() - 1].parse().unwrap(),
                    parts.next().unwrap().parse().unwrap(),
                ))
            }
        })
        .collect()
}

#[cfg(test)]
mod day14_test {
    use crate::{part1, part2};

    #[test]
    fn test_part_1_example() {
        let input =
            "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X\nmem[8] = 11\nmem[7] = 101\nmem[8] = 0";

        assert_eq!(part1(&input).unwrap(), 165);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part1(include_str!("../input.txt")).unwrap(), 6559449933360);
    }

    #[test]
    fn test_part_2() {
        let example = "mask = 000000000000000000000000000000X1001X\nmem[42] = 100\nmask = 00000000000000000000000000000000X0XX\nmem[26] = 1";
        assert_eq!(part2(&example).unwrap(), 208);

        assert_eq!(part2(include_str!("../input.txt")).unwrap(), 3369767240513);
    }
}

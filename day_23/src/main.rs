use std::collections::{HashMap, VecDeque};

fn main() {
    let input = "123487596";

    let result = part1(&input, 100).expect("Error in part 1");
    println!("Part 1: {}", result);

    let result = part2(&input, 10_000_000).expect("Error in part 2");
    println!("Part 2: {}", result);
}

fn part1(input: &str, rounds: u32) -> Result<String, ()> {
    let mut ring = parse_input(&input, false);

    for _ in 0..rounds {
        ring.tick();
    }

    Ok(ring.into())
}

fn part2(input: &str, rounds: usize) -> Result<usize, ()> {
    let mut ring = parse_input(&input, true);

    for _ in 0..rounds {
        ring.tick();
    }

    let first = ring.cups[&1];
    let second = ring.cups[&first];

    Ok(first * second)
}

#[derive(Debug)]
struct Ring {
    cups: HashMap<usize, usize>,
    current_cup: usize,
    min: usize,
    max: usize,
}

impl Ring {
    fn tick(&mut self) {
        let three_cups = self.take_three_cups();
        let mut destination_cup = self.current_cup - 1;

        loop {
            if destination_cup == 0 {
                destination_cup = self.max;
            }
            if three_cups.contains(&destination_cup) {
                destination_cup =
                    (destination_cup - 1 + self.cups.len() + 3) % (self.cups.len() + 3);
            } else {
                break;
            }
        }

        let [a, b, c] = three_cups;

        let tail = self.cups[&destination_cup];
        self.cups.insert(destination_cup, a);
        self.cups.insert(a, b);
        self.cups.insert(b, c);
        self.cups.insert(c, tail);

        self.inc_cup();
    }

    fn inc_cup(&mut self) -> usize {
        self.current_cup = self.cups[&self.current_cup];
        self.current_cup
    }

    fn take_three_cups(&mut self) -> [usize; 3] {
        let first = self.cups[&self.current_cup];
        let second = self.cups[&first];
        let third = self.cups[&second];
        let next = self.cups[&third];

        self.cups.insert(self.current_cup, next);
        self.cups.remove(&first);
        self.cups.remove(&second);
        self.cups.remove(&third);

        [first, second, third]
    }
}

impl Into<String> for Ring {
    fn into(self) -> String {
        let mut ptr = 1;
        let mut ring = vec![];
        while ring.len() < self.cups.len() - 1 {
            let v = self.cups[&ptr];
            ring.push(v);
            ptr = v;
        }
        ring.iter().map(|n| n.to_string()).collect()
    }
}

fn parse_input(input: &str, extend: bool) -> Ring {
    let mut digits = input
        .chars()
        .map(|ch| ch.to_digit(10).unwrap() as usize)
        .collect::<VecDeque<usize>>();

    let current_cup = *digits.iter().next().unwrap();

    if extend {
        for i in 10..=1_000_000 {
            digits.push_back(i);
        }
    }

    let min = *digits.iter().min().unwrap();
    let max = *digits.iter().max().unwrap();

    let it1 = digits.clone().into_iter();
    let last = digits.pop_back().unwrap();
    digits.push_front(last);

    Ring {
        current_cup,
        min,
        max,
        cups: digits.into_iter().zip(it1).collect(),
    }
}

#[cfg(test)]
mod day23_test {
    use crate::{part1, part2};

    #[test]
    fn test_part_1() {
        assert_eq!(&part1("389125467", 10).unwrap(), "92658374");
        assert_eq!(&part1("389125467", 100).unwrap(), "67384529");
        assert_eq!(&part1("123487596", 100).unwrap(), "47598263");
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part2("389125467", 10_000_000).unwrap(), 149245887792);
        assert_eq!(part2("123487596", 10_000_000).unwrap(), 248009574232);
    }
}

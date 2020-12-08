use std::collections::HashSet;

fn main() {
    let input = include_str!("../input.txt");

    let result = part1(&input).expect("Error in part 1");
    println!("Part 1: {}", result);

    let (mutated_line, acc) = part2(&input).expect("Error in part 2");
    println!(
        "Part 2: Mutated line {} with accumulator {}",
        mutated_line, acc
    );
}

fn part1(input: &str) -> Result<isize, ()> {
    let memory = parse_input(input);

    let mut com = Computer::new();
    let mut seen_ops = HashSet::new();

    loop {
        let next_op = memory[com.ip];

        if !seen_ops.insert(com.ip) {
            break;
        }

        com.evaluate_line(next_op);
    }

    Ok(com.acc)
}

fn part2(input: &str) -> Result<(usize, isize), ()> {
    let memory = parse_input(input);

    let mutations = build_mutations(&memory);

    for (program, mutated_line) in mutations {
        let mut com = Computer::new();
        let mut seen_ops = HashSet::new();
        let mut broken = false;

        loop {
            if com.ip >= program.len() {
                break;
            }

            let next_op = program[com.ip];
            if !seen_ops.insert(com.ip) {
                broken = true;
                break;
            }
            com.evaluate_line(next_op);
        }

        if !broken {
            return Ok((mutated_line, com.acc));
        }
    }

    Err(())
}

fn build_mutations<'a>(memory: &'a [(&'a str, isize)]) -> Vec<(Vec<(&'a str, isize)>, usize)> {
    let mut mutated_indices = HashSet::new();
    let mut mutations = Vec::new();

    for (idx, line) in memory.iter().enumerate() {
        if line.0.contains("jmp") {
            let mut mutation = memory.to_owned();
            mutation[idx] = line.to_owned();
            mutation[idx].0 = "nop";

            mutations.push((mutation, idx));
            mutated_indices.insert(idx);
        }

        if line.0.contains("nop") {
            let mut mutation = memory.to_owned();
            mutation[idx] = line.to_owned();
            mutation[idx].0 = "jmp";

            mutations.push((mutation, idx));
            mutated_indices.insert(idx);
        }
    }

    mutations
}

struct Computer {
    pub acc: isize,
    pub ip: usize,
}

impl Computer {
    fn new() -> Self {
        Computer { acc: 0, ip: 0 }
    }

    pub fn evaluate_line(&mut self, line: (&str, isize)) {
        match line {
            ("nop", _) => {}
            ("acc", v) => self.acc += v,
            ("jmp", v) => {
                self.ip = ((self.ip as isize) + v) as usize;
            }
            _ => unreachable!(),
        }

        match line {
            ("jmp", _) => {}
            (_, _) => self.ip += 1,
        }
    }
}

type Memory<'a> = Vec<(&'a str, isize)>;

fn parse_input(input: &str) -> Memory {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            (
                parts.next().unwrap(),
                parts.next().unwrap().parse::<isize>().unwrap(),
            )
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod test {
    use crate::{part1, part2};

    #[test]
    fn test_part_1() {
        assert_eq!(part1(include_str!("../input.txt")).unwrap(), 1384);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part2(include_str!("../input.txt")).unwrap(), (193, 761));
    }
}

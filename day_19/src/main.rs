use std::collections::HashMap;

fn main() {
    let result = part1(&include_str!("../input_1.txt")).expect("Error in part 1");
    println!("Part 1: {}", result);

    let result = part2(&include_str!("../input_1.txt")).expect("Error in part 2");
    println!("Part 2: {}", result);
}

fn part1(input: &str) -> Result<usize, ()> {
    let (rules, messages) = parse_input(&input);
    let checker = RuleChecker::new(rules);
    Ok(messages
        .iter()
        .filter(|m| checker.match_all(&m, &mut vec![0]))
        .count())
}

fn part2(input: &str) -> Result<usize, ()> {
    let (rules, messages) = parse_input(&input);
    let checker = RuleChecker::new(rules);
    Ok(messages
        .iter()
        .filter(|m| checker.match_all(&m, &mut vec![0]))
        .count())
}

struct RuleChecker {
    rules: HashMap<usize, Rule>,
}

impl RuleChecker {
    fn new(rules: HashMap<usize, Rule>) -> Self {
        Self { rules }
    }

    fn match_all(&self, message: &str, mut queue: &mut Vec<usize>) -> bool {
        if queue.is_empty() && message.is_empty() {
            return true;
        }

        if queue.is_empty() || message.is_empty() {
            return false;
        }

        let next_rule = &self.rules[&queue.pop().unwrap()];

        match next_rule {
            Rule::Terminal(ch) => self.match_char(&message, ch, &mut queue),
            Rule::Single(seq) => self.match_sequence(&message, &seq, &mut queue),
            Rule::Double(seq1, seq2) => {
                self.match_sequence(&message, &seq1, &mut queue.clone())
                    || self.match_sequence(&message, &seq2, &mut queue.clone())
            }
        }
    }

    fn match_char(&self, message: &str, ch: &char, queue: &mut Vec<usize>) -> bool {
        match message.chars().next() {
            Some(c) if c == *ch => self.match_all(&message[1..], queue),
            _ => false,
        }
    }

    fn match_sequence(&self, message: &str, sequence: &[usize], queue: &mut Vec<usize>) -> bool {
        sequence
            .iter()
            .rev()
            .for_each(|subrule| queue.push(*subrule));
        self.match_all(&message, queue)
    }
}

#[derive(Debug, Clone)]
enum Rule {
    Terminal(char),
    Single(Vec<usize>),
    Double(Vec<usize>, Vec<usize>),
}

fn parse_input(input: &str) -> (HashMap<usize, Rule>, Vec<&str>) {
    let mut sections = input.split("\n\n");
    let raw_rules = sections.next().unwrap();
    let messages = sections
        .next()
        .unwrap()
        .lines()
        .map(|x| x.trim())
        .collect::<Vec<_>>();

    let rules = raw_rules
        .lines()
        .map(|x| parse_rule(x.trim()))
        .collect::<_>();

    (rules, messages)
}

fn parse_rule(input: &str) -> (usize, Rule) {
    let mut parts = input.split(':');
    let id = parts.next().unwrap().parse().unwrap();
    let rem = parts.next().unwrap().trim();

    if input.contains('"') {
        (id, Rule::Terminal(rem.chars().nth(1).unwrap()))
    } else if input.contains('|') {
        let mut parts = rem.split(" | ");
        let left = parts
            .next()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect::<Vec<usize>>();
        let right = parts
            .next()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse().unwrap())
            .collect::<Vec<usize>>();
        (id, Rule::Double(left, right))
    } else {
        (
            id,
            Rule::Single(
                rem.split_whitespace()
                    .map(|x| x.parse().unwrap())
                    .collect::<Vec<usize>>(),
            ),
        )
    }
}

#[cfg(test)]
mod test {
    use crate::{part1, part2};

    #[test]
    fn test_part_1() {
        let input = "0: 4 1 5
                1: 2 3 | 3 2
                2: 4 4 | 5 5
                3: 4 5 | 5 4
                4: \"a\"
                5: \"b\"

                ababbb
                bababa
                abbbab
                aaabbb
                aaaabbb";

        assert_eq!(part1(&input).unwrap(), 2);
        assert_eq!(part1(include_str!("../input_1.txt")).unwrap(), 113);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part2(include_str!("../input_2.txt")).unwrap(), 253);
    }
}

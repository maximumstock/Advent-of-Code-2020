fn main() {
    let result = part1().expect("Error in part 1");
    println!("Part 1: There are {} valid rules", result.len());

    let result = part2().expect("Error in part 2");
    println!("Part 2: There are {} valid rules", result.len());
}

fn part1() -> Result<Vec<PasswordRule>, ()> {
    let valid_rules = read_input()
        .into_iter()
        .filter(|rule| rule.is_valid())
        .collect::<Vec<_>>();
    Ok(valid_rules)
}

fn part2() -> Result<Vec<PasswordRule>, ()> {
    let valid_rules = read_input()
        .into_iter()
        .filter(|rule| rule.is_valid2())
        .collect::<Vec<_>>();
    Ok(valid_rules)
}

#[derive(Debug)]
struct PasswordRule {
    min: u8,
    max: u8,
    char: char,
    password: String,
}

impl PasswordRule {
    pub fn is_valid(&self) -> bool {
        let count = self.password.chars().filter(|c| c.eq(&self.char)).count() as u8;
        count >= self.min && count <= self.max
    }

    pub fn is_valid2(&self) -> bool {
        let first = self.password.chars().nth((self.min - 1).into()).unwrap();
        let second = self.password.chars().nth((self.max - 1).into()).unwrap();
        first.eq(&self.char) ^ second.eq(&self.char)
    }
}

fn read_input() -> Vec<PasswordRule> {
    include_str!("../input.txt")
        .lines()
        .map(|i| {
            let mut parts = i
                .split(|c: char| c.eq(&' ') || c.eq(&':') || c.eq(&'-'))
                .filter(|x| !x.is_empty());
            PasswordRule {
                min: parts.next().unwrap().parse::<u8>().unwrap(),
                max: parts.next().unwrap().parse::<u8>().unwrap(),
                char: parts.next().unwrap().chars().next().unwrap(),
                password: parts.next().unwrap().to_string(),
            }
        })
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod test {
    use crate::{part1, part2};

    #[test]
    fn test_part_1() {
        assert_eq!(part1().unwrap().len(), 398);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part2().unwrap().len(), 562);
    }
}

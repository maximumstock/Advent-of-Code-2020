fn main() {
    let input = include_str!("../input.txt");

    let result = part1(&input).expect("Error in part 1");
    println!("Part 1: {}", result);

    let result = part2(&input).expect("Error in part 2");
    println!("Part 2: {}", result);
}

fn part1(input: &str) -> Result<usize, ()> {
    Ok(input.lines().map(|line| shunting_yard(&line, false)).sum())
}

fn part2(input: &str) -> Result<usize, ()> {
    Ok(input.lines().map(|line| shunting_yard(&line, true)).sum())
}

#[derive(Debug)]
enum Expression {
    Operation(char),
    Value(usize),
}

fn reverse_polish_notation(input: &[Expression]) -> usize {
    let mut stack = Vec::new();
    for expression in input {
        match expression {
            Expression::Operation(')') => {}
            Expression::Operation('(') => {}
            Expression::Operation(op) => {
                let left = stack.pop().unwrap();
                let right = stack.pop().unwrap();
                let result = match *op {
                    '+' => left + right,
                    '*' => left * right,
                    _ => unreachable!(),
                };
                stack.push(result);
            }
            Expression::Value(v) => stack.push(*v),
        }
    }

    stack.pop().unwrap()
}

/// See https://brilliant.org/wiki/shunting-yard-algorithm/
fn shunting_yard(line: &str, addition_before_multiplication: bool) -> usize {
    let mut operations = Vec::new();
    let mut output = Vec::new();

    let precedence = match addition_before_multiplication {
        true => vec!['+'],
        false => vec!['+', '*'],
    };

    for ch in line.chars().filter(|c| *c != ' ') {
        match ch {
            '(' => operations.push(ch),
            ')' => loop {
                match operations.pop() {
                    Some('(') | None => break,
                    Some(op) => output.push(Expression::Operation(op)),
                }
            },
            '+' | '*' => {
                while let Some(op) = operations.pop() {
                    if precedence.contains(&op) {
                        output.push(Expression::Operation(op));
                    } else {
                        operations.push(op);
                        break;
                    }
                }
                operations.push(ch);
            }
            ch => {
                output.push(Expression::Value(ch.to_digit(10).unwrap() as usize));
            }
        }
    }

    while let Some(op) = operations.pop() {
        output.push(Expression::Operation(op));
    }

    reverse_polish_notation(&output)
}

#[cfg(test)]
mod day18_test {
    use crate::{part1, part2, shunting_yard};

    #[test]
    fn test_evaluate_line() {
        assert_eq!(shunting_yard("1 + (2 * 3) + (4 * (5 + 6))", false), 51);
        assert_eq!(shunting_yard("2 * 3 + (4 * 5)", false), 26);
        assert_eq!(shunting_yard("5 + (8 * 3 + 9 + 3 * 4 * 3)", false), 437);
        assert_eq!(
            shunting_yard("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", false),
            12240
        );
        assert_eq!(
            shunting_yard("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", false),
            13632
        );

        assert_eq!(shunting_yard("1 + (2 * 3) + (4 * (5 + 6))", true), 51);
        assert_eq!(shunting_yard("2 * 3 + (4 * 5)", true), 46);
        assert_eq!(shunting_yard("5 + (8 * 3 + 9 + 3 * 4 * 3)", true), 1445);
        assert_eq!(
            shunting_yard("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))", true),
            669060
        );
        assert_eq!(
            shunting_yard("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2", true),
            23340
        );
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part1(include_str!("../input.txt")).unwrap(), 50956598240016);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            part2(include_str!("../input.txt")).unwrap(),
            535809575344339
        );
    }
}

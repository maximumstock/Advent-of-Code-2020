use std::collections::VecDeque;

fn main() {
    let input = include_str!("../input.txt");

    let result = part1(&input).expect("Error in part 1");
    println!("Part 1: {}", result);

    let result = part2(&input).expect("Error in part 2");
    println!("Part 2: {}", result);
}

fn part1(input: &str) -> Result<usize, ()> {
    let (rules, _, nearby_tickets) = parse_input(&input);
    Ok(nearby_tickets
        .iter()
        .filter_map(|ticket| {
            ticket
                .iter()
                .find(|n| !rules.iter().any(|rule| rule.accepts(n)))
        })
        .sum())
}

fn filter_valid_tickets<'a>(tickets: &'a [Ticket], rules: &'a [Rule]) -> Vec<&'a Ticket> {
    tickets
        .iter()
        .filter(|ticket| {
            ticket
                .iter()
                .all(|n| rules.iter().any(|rule| rule.accepts(n)))
        })
        .collect::<Vec<_>>()
}

fn part2(input: &str) -> Result<usize, ()> {
    let (rules, ticket, nearby_tickets) = parse_input(&input);

    let valid_tickets = filter_valid_tickets(&nearby_tickets, &rules);

    match find_valid_rule_arrangement(&rules, &valid_tickets) {
        Ok(arrangement) => Ok(arrangement
            .iter()
            .enumerate()
            .filter(|(_, rule)| rule.field.contains("departure"))
            .map(|(idx, _)| ticket[idx])
            .product()),
        Err(_) => Err(()),
    }
}

fn find_valid_rule_arrangement<'a>(
    rules: &'a [Rule],
    ticket_pool: &[&Ticket],
) -> Result<Vec<&'a Rule<'a>>, ()> {
    let mut queue = VecDeque::new();

    for rule in rules.iter() {
        queue.push_back(vec![rule]);
    }

    while let Some(arrangement) = queue.pop_front() {
        let is_arrangement_valid = ticket_pool.iter().all(|ticket| {
            arrangement
                .iter()
                .zip(ticket.iter())
                .all(|(rule, n)| rule.accepts(n))
        });

        if is_arrangement_valid {
            if arrangement.len() == rules.len() {
                return Ok(arrangement);
            }

            for rule in rules {
                if !arrangement.contains(&rule) {
                    let mut new_arrangement = arrangement.clone();
                    new_arrangement.push(rule);
                    queue.push_front(new_arrangement);
                }
            }
        }
    }

    Err(())
}

#[derive(Debug, PartialEq)]
struct Rule<'a> {
    field: &'a str,
    rule1: (usize, usize),
    rule2: (usize, usize),
}

impl<'a> Rule<'a> {
    fn accepts(&self, n: &usize) -> bool {
        (self.rule1.0 <= *n && self.rule1.1 >= *n) || (self.rule2.0 <= *n && self.rule2.1 >= *n)
    }
}

type Ticket = Vec<usize>;

fn parse_input(input: &str) -> (Vec<Rule>, Ticket, Vec<Ticket>) {
    let input = input.lines().map(|x| x.trim());
    let raw_rules = input.clone().filter(|l| l.contains("or"));
    let raw_ticket = input.clone().find(|l| l.contains(',')).unwrap();
    let raw_nearby_tickets = input.filter(|l| l.contains(',')).skip(1);

    let ticket: Ticket = raw_ticket
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect::<Vec<_>>();

    let rules = raw_rules
        .map(|line| {
            let mut parts = line.split(": ");
            let field = parts.next().unwrap();
            let ranges = parts
                .next()
                .unwrap()
                .split(" or ")
                .map(|rule| {
                    let mut parts = rule.split('-');
                    (
                        parts.next().unwrap().parse().unwrap(),
                        parts.next().unwrap().parse().unwrap(),
                    )
                })
                .collect::<Vec<(usize, usize)>>();
            Rule {
                field,
                rule1: *ranges.get(0).unwrap(),
                rule2: *ranges.get(1).unwrap(),
            }
        })
        .collect::<Vec<Rule>>();

    let nearby_tickets = raw_nearby_tickets
        .map(|line| {
            line.split(',')
                .map(|x| x.parse().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<Ticket>>();

    (rules, ticket, nearby_tickets)
}

#[cfg(test)]
mod day16_test {
    use crate::{
        filter_valid_tickets, find_valid_rule_arrangement, parse_input, part1, part2, Rule,
    };

    #[test]
    fn test_part_1() {
        assert_eq!(part1(include_str!("../input.txt")).unwrap(), 24980);
    }

    #[test]
    fn test_filter_valid_tickets() {
        let input = "class: 1-3 or 5-7
                          row: 6-11 or 33-44
                          seat: 13-40 or 45-50

                          your ticket:
                          7,1,14

                          nearby tickets:
                          7,3,47
                          40,4,50
                          55,2,20
                          38,6,12";

        let (rules, _, nearby_tickets) = parse_input(&input);
        let valid_tickets = filter_valid_tickets(&nearby_tickets, &rules);

        assert_eq!(valid_tickets.len(), 1);
    }

    #[test]
    fn test_part_2_example() {
        let input = "class: 0-1 or 4-19
                          row: 0-5 or 8-19
                          seat: 0-13 or 16-19

                          your ticket:
                          11,12,13

                          nearby tickets:
                          3,9,18
                          15,1,5
                          5,14,9
                          ";

        let (rules, _ticket, nearby_tickets) = parse_input(&input);
        let valid_tickets = filter_valid_tickets(&nearby_tickets, &rules);
        let arrangement = find_valid_rule_arrangement(&rules, &valid_tickets).unwrap();

        assert_eq!(
            arrangement,
            vec![
                &Rule {
                    field: "row",
                    rule1: (0, 5),
                    rule2: (8, 19)
                },
                &Rule {
                    field: "class",
                    rule1: (0, 1),
                    rule2: (4, 19)
                },
                &Rule {
                    field: "seat",
                    rule1: (0, 13),
                    rule2: (16, 19)
                }
            ]
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part2(include_str!("../input.txt")).unwrap(), 809376774329);
    }
}

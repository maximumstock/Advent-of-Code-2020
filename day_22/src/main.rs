use std::{
    collections::{HashSet, VecDeque},
    println,
};

fn main() {
    let input = include_str!("../input.txt");

    let result = part1(&input).expect("Error in part 1");
    println!("Part 1: {}", result);

    let result = part2(&input).expect("Error in part 2");
    println!("Part 2: {}", result);
}

type Deck = VecDeque<usize>;

fn part1(input: &str) -> Result<usize, ()> {
    let (mut p1, mut p2) = parse_input(&input);

    while !p1.is_empty() && !p2.is_empty() {
        let p1v = p1.pop_front().unwrap();
        let p2v = p2.pop_front().unwrap();

        if p1v > p2v {
            p1.push_back(p1v);
            p1.push_back(p2v);
        }
        if p2v > p1v {
            p2.push_back(p2v);
            p2.push_back(p1v);
        }
    }

    let winner = match p1.is_empty() {
        true => p2,
        false => p1,
    };

    Ok(calculate_score(&winner))
}

fn play_game(mut p1: Deck, mut p2: Deck) -> (bool, Deck) {
    let mut game_states = HashSet::new();

    while !p1.is_empty() && !p2.is_empty() {
        let new_state = (p1.clone(), p2.clone());
        if !game_states.insert(new_state) {
            return (true, p1);
        }

        let p1v = p1.pop_front().unwrap();
        let p2v = p2.pop_front().unwrap();

        let has_player_one_won = {
            if p1v <= p1.len() && p2v <= p2.len() {
                let new_p1 = p1.iter().take(p1v).cloned().collect();
                let new_p2 = p2.iter().take(p2v).cloned().collect();
                let (winner, _) = play_game(new_p1, new_p2);
                winner
            } else {
                p1v > p2v
            }
        };

        if has_player_one_won {
            p1.push_back(p1v);
            p1.push_back(p2v);
        } else {
            p2.push_back(p2v);
            p2.push_back(p1v);
        }
    }

    if p1.is_empty() {
        (false, p2)
    } else {
        (true, p1)
    }
}

fn part2(input: &str) -> Result<usize, ()> {
    let (p1, p2) = parse_input(&input);
    let (_, deck) = play_game(p1, p2);
    Ok(calculate_score(&deck))
}

fn calculate_score(deck: &Deck) -> usize {
    deck.iter()
        .rev()
        .enumerate()
        .map(|(idx, value)| (idx + 1) * value)
        .sum()
}

fn parse_input(input: &str) -> (Deck, Deck) {
    let mut parts = input.split("\n\n");
    let player_one = parts
        .next()
        .unwrap()
        .lines()
        .skip(1)
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Deck>();

    let player_two = parts
        .next()
        .unwrap()
        .lines()
        .skip(1)
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Deck>();

    (player_one, player_two)
}

#[cfg(test)]
mod test {
    use crate::{part1, part2};

    #[test]
    fn test_part_1() {
        assert_eq!(part1(include_str!("../test_input.txt")).unwrap(), 306);
        assert_eq!(part1(include_str!("../input.txt")).unwrap(), 32677);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part2(include_str!("../test_input.txt")).unwrap(), 291);
        assert_eq!(part2(include_str!("../input.txt")).unwrap(), 33661);
    }
}

use std::collections::HashSet;

fn main() {
    let input = include_str!("../input.txt");
    let result = part1(&input).expect("Error in part 1");
    println!("Part 1: {}", result);

    let result = part2(&input).expect("Error in part 2");
    println!("Part 2: {}", result);
}

fn part1(input: &str) -> Result<usize, ()> {
    Ok(input.lines().map(parse_seat_id).map(|x| x.2).max().unwrap())
}

fn part2(input: &str) -> Result<usize, ()> {
    let seat_ids = input
        .lines()
        .map(parse_seat_id)
        .collect::<HashSet<(usize, usize, usize)>>();

    let min_id = seat_ids.iter().map(|x| x.2).min().unwrap();
    let max_id = seat_ids.iter().map(|x| x.2).max().unwrap();

    for row in 1..=126 {
        for col in 0..7 {
            let id = row * 8 + col;
            if id < min_id || id > max_id {
                continue;
            }
            if !seat_ids.contains(&(row, col, id)) {
                return Ok(id);
            }
        }
    }

    Err(())
}

fn parse_seat_id(input: &str) -> (usize, usize, usize) {
    let raw_binary = input
        .chars()
        .map(|c| match c {
            'F' | 'L' => '0',
            'B' | 'R' => '1',
            _ => unreachable!(),
        })
        .collect::<String>();

    let row = usize::from_str_radix(&raw_binary[..7], 2).unwrap();
    let col = usize::from_str_radix(&raw_binary[7..], 2).unwrap();

    (row, col, row * 8 + col)
}

#[cfg(test)]
mod test {
    use crate::{parse_seat_id, part1, part2};

    #[test]
    fn test_parse_seat_id() {
        assert_eq!(parse_seat_id("BFFFBBFRRR").2, 567);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part1(include_str!("../input.txt")).unwrap(), 965);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part2(include_str!("../input.txt")).unwrap(), 524);
    }
}

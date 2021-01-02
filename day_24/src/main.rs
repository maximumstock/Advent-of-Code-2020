use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../input.txt");

    let result = part1(&input).expect("Error in part 1");
    println!("Part 1: {}", result);

    let result = part2(&input).expect("Error in part 2");
    println!("Part 2: {}", result);
}

fn part1(input: &str) -> Result<usize, ()> {
    let floor = build_initial_floor(&input);
    Ok(floor.len())
}

fn part2(input: &str) -> Result<usize, ()> {
    let mut floor = build_initial_floor(&input);

    for _ in 0..100 {
        let mut new_floor = HashSet::new();
        let mut tiles_to_check = HashSet::new();

        for tile in &floor {
            tiles_to_check.insert(*tile);
            let neighbours = find_neighbours(&tile);
            for n in &neighbours {
                tiles_to_check.insert(*n);
            }
        }

        for tile in &tiles_to_check {
            let neighbours = find_neighbours(&tile);
            let black_neighbours = neighbours.iter().filter(|c| floor.contains(&c)).count();
            let is_black = floor.contains(&tile);

            if is_black && (black_neighbours == 1 || black_neighbours == 2) {
                new_floor.insert(*tile);
            }

            if !is_black && black_neighbours == 2 {
                new_floor.insert(*tile);
            }
        }

        floor = new_floor
    }

    Ok(floor.len())
}

fn build_initial_floor(input: &str) -> HashSet<(isize, isize, isize)> {
    let tiles = parse_input(&input);
    let mut floor = HashSet::new();

    for tile in tiles {
        if floor.contains(&tile.coords) {
            floor.remove(&tile.coords);
        } else {
            floor.insert(tile.coords);
        }
    }

    floor
}

const DIRECTIONS: &[(isize, isize, isize)] = &[
    (1, -1, 0),
    (1, 0, -1),
    (0, -1, 1),
    (-1, 1, 0),
    (0, 1, -1),
    (-1, 0, 1),
];

fn find_neighbours(coords: &(isize, isize, isize)) -> Vec<(isize, isize, isize)> {
    DIRECTIONS
        .iter()
        .map(|(dx, dy, dz)| (coords.0 + *dx, coords.1 + *dy, coords.2 + *dz))
        .collect()
}

fn parse_input(input: &str) -> Vec<Tile> {
    input.lines().map(parse_tile).collect()
}

#[derive(Debug)]
enum Move {
    East,
    SouthEast,
    NorthEast,
    West,
    SouthWest,
    NorthWest,
}

#[derive(Debug)]
struct Tile {
    coords: (isize, isize, isize),
}

impl From<&[Move]> for Tile {
    fn from(moves: &[Move]) -> Self {
        let mut coords = (0, 0, 0);

        for m in moves {
            let (dx, dy, dz) = match m {
                Move::East => (1, -1, 0),
                Move::NorthEast => (1, 0, -1),
                Move::SouthEast => (0, -1, 1),
                Move::West => (-1, 1, 0),
                Move::NorthWest => (0, 1, -1),
                Move::SouthWest => (-1, 0, 1),
            };

            coords = (coords.0 + dx, coords.1 + dy, coords.2 + dz);
        }

        Self { coords }
    }
}

fn parse_tile(input: &str) -> Tile {
    let mut slice = input;
    let mut moves = vec![];

    while !slice.is_empty() {
        let first = slice.chars().next().unwrap();
        let second = slice.chars().nth(1);

        let r#move = match (first, second) {
            ('n', Some('w')) => Move::NorthWest,
            ('n', Some('e')) => Move::NorthEast,
            ('s', Some('e')) => Move::SouthEast,
            ('s', Some('w')) => Move::SouthWest,
            ('e', _) => Move::East,
            ('w', _) => Move::West,
            _ => unreachable!(),
        };

        let move_length = match r#move {
            Move::East | Move::West => 1,
            _ => 2,
        };

        moves.push(r#move);
        slice = &slice[move_length..];
    }

    Tile::from(&moves[..])
}

#[cfg(test)]
mod day24_test {
    use crate::{part1, part2};

    #[test]
    fn test_part_1() {
        assert_eq!(part1(include_str!("../input.txt")).unwrap(), 420);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part2(include_str!("../input.txt")).unwrap(), 4206);
    }
}

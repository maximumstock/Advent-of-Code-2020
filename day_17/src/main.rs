use std::collections::{HashMap, HashSet};

fn main() {
    let input = include_str!("../input.txt");

    let result = part1(&input, false).expect("Error in part 1");
    println!("Part 1: {}", result);

    let result = part1(&input, true).expect("Error in part 2");
    println!("Part 2: {}", result);
}

fn part1(input: &str, is_4d: bool) -> Result<usize, ()> {
    let mut grid = parse_input(&input);

    for _ in 0..6 {
        let mut new_active = HashMap::new();
        let mut inactive = HashSet::new();

        for (position, status) in grid.active.iter() {
            let neighbours = grid.neighbours(position, is_4d);
            let active_neighbours = neighbours.iter().filter(|(_, status)| *status).count();

            for (pos, status) in neighbours {
                if !status {
                    inactive.insert(pos);
                }
            }

            if *status && (active_neighbours == 2 || active_neighbours == 3) {
                new_active.insert(*position, true);
            }
        }

        for position in inactive {
            let neighbours = grid.neighbours(&position, is_4d);
            let active_neighbours = neighbours.iter().filter(|(_, status)| *status).count();
            let status = grid.get_status(&position);

            if !status && active_neighbours == 3 {
                new_active.insert(position, true);
            }
        }

        grid.active = new_active;
    }

    Ok(grid.active.values().filter(|status| **status).count())
}

fn parse_input(input: &str) -> Grid {
    let mut x = 0;

    let mut active = HashMap::new();

    for (y, line) in input.lines().enumerate() {
        for ch in line.trim().chars() {
            if let '#' = ch {
                active.insert((x, y as isize, 0, 0), true);
            }
            x += 1;
        }
        x = 0;
    }

    Grid {
        active,
        inactive: HashMap::new(),
    }
}

type Position = (isize, isize, isize, isize);

#[derive(Debug, Clone)]
struct Grid {
    active: HashMap<Position, bool>,
    inactive: HashMap<Position, bool>,
}

impl Grid {
    pub fn neighbours(&self, position: &Position, is_4d: bool) -> Vec<(Position, bool)> {
        let mut neighbours = vec![];
        let mut dw_range = 0..=0;

        if is_4d {
            dw_range = -1..=1
        }

        let (x, y, z, w) = position;
        for dx in -1..=1 {
            for dy in -1..=1 {
                for dz in -1..=1 {
                    for dw in dw_range.clone() {
                        if !(dx == 0 && dy == 0 && dz == 0 && dw == 0) {
                            let neighbour_position = (x + dx, y + dy, z + dz, w + dw);
                            neighbours
                                .push((neighbour_position, self.get_status(&neighbour_position)));
                        }
                    }
                }
            }
        }
        neighbours
    }

    pub fn get_status(&self, position: &Position) -> bool {
        *self.active.get(&position).unwrap_or(&false)
    }
}

#[cfg(test)]
mod test {
    use crate::part1;

    #[test]
    fn test_part_1_example() {
        let input = ".#.
                          ..#
                          ###";

        assert_eq!(part1(&input, false).unwrap(), 112);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part1(include_str!("../input.txt"), false).unwrap(), 215);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part1(include_str!("../input.txt"), true).unwrap(), 1728);
    }
}

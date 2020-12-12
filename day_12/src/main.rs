fn main() {
    let input = include_str!("../input.txt");

    let result = part1(&input).expect("Error in part 1");
    println!("Part 1: {}", result);

    let result = part2(&input).expect("Error in part 2");
    println!("Part 2: {}", result);
}

fn part1(input: &str) -> Result<i32, ()> {
    let motions = parse_input(&input);
    let mut ship = Ship::new(0, 0);

    for motion in motions {
        match motion {
            ('N', val) => ship.y -= val,
            ('S', val) => ship.y += val,
            ('E', val) => ship.x += val,
            ('W', val) => ship.x -= val,
            ('F', val) => {
                ship.x += val * ship.waypoint.0;
                ship.y += val * ship.waypoint.1;
            }
            ('R', val) => {
                ship.waypoint = rotate_2d(ship.waypoint.0, ship.waypoint.1, val);
            }
            ('L', val) => {
                ship.waypoint = rotate_2d(ship.waypoint.0, ship.waypoint.1, -val);
            }
            _ => unreachable!(),
        }
    }

    Ok(ship.x + ship.y)
}

fn part2(input: &str) -> Result<i32, ()> {
    let motions = parse_input(&input);
    let mut ship = Ship::new(0, 0);
    ship.waypoint = (10, -1);

    for motion in motions {
        match motion {
            ('N', val) => ship.waypoint.1 -= val,
            ('S', val) => ship.waypoint.1 += val,
            ('E', val) => ship.waypoint.0 += val,
            ('W', val) => ship.waypoint.0 -= val,
            ('F', val) => {
                ship.x += val * ship.waypoint.0;
                ship.y += val * ship.waypoint.1;
            }
            ('R', val) => {
                ship.waypoint = rotate_2d(ship.waypoint.0, ship.waypoint.1, val) as (i32, i32);
            }
            ('L', val) => {
                ship.waypoint = rotate_2d(ship.waypoint.0, ship.waypoint.1, -val);
            }
            _ => unreachable!(),
        }
    }

    Ok(ship.x + ship.y)
}

struct Ship {
    pub x: i32,
    pub y: i32,
    pub waypoint: (i32, i32),
}

impl Ship {
    pub fn new(x: i32, y: i32) -> Self {
        Ship {
            x,
            y,
            waypoint: (1, 0),
        }
    }
}

fn rotate_2d(x: i32, y: i32, degrees: i32) -> (i32, i32) {
    (
        ((x as f32 * (degrees as f32).to_radians().cos())
            - (y as f32 * (degrees as f32).to_radians().sin()))
        .round() as i32,
        ((x as f32 * (degrees as f32).to_radians().sin())
            + (y as f32 * (degrees as f32).to_radians().cos()))
        .round() as i32,
    )
}

fn parse_input(input: &str) -> Vec<(char, i32)> {
    input
        .lines()
        .map(|line| line.trim())
        .map(|line| (line.chars().next().unwrap(), line[1..].parse().unwrap()))
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod day12_test {
    use crate::{part1, part2, rotate_2d};

    #[test]
    fn test_rotation() {
        assert_eq!((0, 1), rotate_2d(1, 0, 90));
    }

    #[test]
    fn test_part_1() {
        let input = "F10
                          N3
                          F7
                          R90
                          F11";
        assert_eq!(part1(&input).unwrap(), 25);
        assert_eq!(part1(include_str!("../input.txt")).unwrap(), 2280);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part2(include_str!("../input.txt")).unwrap(), 38693);
    }
}

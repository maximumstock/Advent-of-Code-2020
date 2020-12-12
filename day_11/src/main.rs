use std::collections::{HashMap, VecDeque};

static DIRECTIONS: &[(isize, isize)] = &[
    (0, 1),
    (1, 0),
    (-1, 0),
    (0, -1),
    (1, 1),
    (-1, -1),
    (-1, 1),
    (1, -1),
];

fn main() {
    let input = parse_input(include_str!("../input.txt"));

    let result = part1(input.clone()).expect("Error in part 1");
    println!("Part 1: {}", result);

    let result = part2(input).expect("Error in part 2");
    println!("Part 2: {}", result);
}

fn parse_input(input: &str) -> HashMap<(isize, isize), char> {
    let width = input.split_whitespace().into_iter().next().unwrap().len();

    input
        .chars()
        .filter(|c| !c.is_whitespace())
        .enumerate()
        .map(|(idx, c)| (((idx % width) as isize, (idx / width) as isize), c))
        .collect()
}

fn adjacent_nodes(nodes: &HashMap<(isize, isize), char>, x: isize, y: isize) -> Vec<&char> {
    DIRECTIONS
        .iter()
        .filter_map(|(dx, dy)| nodes.get(&(x + *dx, y + *dy)))
        .collect::<_>()
}

fn adjacent_nodes_fov(nodes: &HashMap<(isize, isize), char>, x: isize, y: isize) -> Vec<&char> {
    let mut neighbours = Vec::new();
    let mut queue = VecDeque::new();

    for (dx, dy) in DIRECTIONS.iter() {
        queue.push_back((*dx, *dy, 1));
    }

    while let Some((dx, dy, scalar)) = queue.pop_front() {
        if let Some(value) = nodes.get(&(x + dx * scalar, y + dy * scalar)) {
            if ['#', 'L'].contains(value) {
                neighbours.push(value);
            } else {
                queue.push_back((dx, dy, scalar + 1));
            }
        }
    }

    neighbours
}

fn part1(map: HashMap<(isize, isize), char>) -> Result<usize, ()> {
    partx(map, &|map, x, y, c| {
        let adjacent_occupied = adjacent_nodes(&map, x, y)
            .iter()
            .filter(|c| '#'.eq(c))
            .count();

        if c.eq(&'L') && adjacent_occupied == 0 {
            Some((x, y, '#'))
        } else if c.eq(&'#') && adjacent_occupied >= 4 {
            Some((x, y, 'L'))
        } else {
            None
        }
    })
}

fn part2(map: HashMap<(isize, isize), char>) -> Result<usize, ()> {
    partx(map, &|map, x, y, c| {
        let adjacent_occupied = adjacent_nodes_fov(&map, x, y)
            .iter()
            .filter(|c| '#'.eq(c))
            .count();

        if c.eq(&'L') && adjacent_occupied == 0 {
            Some((x, y, '#'))
        } else if c.eq(&'#') && adjacent_occupied >= 5 {
            Some((x, y, 'L'))
        } else {
            None
        }
    })
}

type Logic =
    dyn Fn(&HashMap<(isize, isize), char>, isize, isize, char) -> Option<(isize, isize, char)>;

fn partx(mut map: HashMap<(isize, isize), char>, f: &Logic) -> Result<usize, ()> {
    let mut changes = Vec::new();
    loop {
        let non_floor_elements = map
            .iter()
            .filter(|(_, c)| ['#', 'L'].contains(c))
            .collect::<Vec<_>>();

        for ((x, y), c) in non_floor_elements {
            if let Some(change) = f(&map, *x, *y, *c) {
                changes.push(change);
            }
        }

        if changes.is_empty() {
            break;
        }

        while let Some((x, y, c)) = changes.pop() {
            map.insert((x, y), c);
        }
    }

    Ok(map.iter().filter(|(_, c)| '#'.eq(c)).count())
}

#[cfg(test)]
mod day11_test {
    use crate::{parse_input, part1, part2};

    #[test]
    fn test_part_1() {
        let input = "L.LL.LL.LL
                          LLLLLLL.LL
                          L.L.L..L..
                          LLLL.LL.LL
                          L.LL.LL.LL
                          L.LLLLL.LL
                          ..L.L.....
                          LLLLLLLLLL
                          L.LLLLLL.L
                          L.LLLLL.LL";

        assert_eq!(part1(parse_input(&input)).unwrap(), 37);

        assert_eq!(
            part1(parse_input(include_str!("../input.txt"))).unwrap(),
            2481
        );
    }

    #[test]
    fn test_part_2() {
        let input = "L.LL.LL.LL
                          LLLLLLL.LL
                          L.L.L..L..
                          LLLL.LL.LL
                          L.LL.LL.LL
                          L.LLLLL.LL
                          ..L.L.....
                          LLLLLLLLLL
                          L.LLLLLL.L
                          L.LLLLL.LL";

        assert_eq!(part2(parse_input(&input)).unwrap(), 26);

        assert_eq!(
            part2(parse_input(include_str!("../input.txt"))).unwrap(),
            2227
        );
    }
}

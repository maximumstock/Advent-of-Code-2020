fn main() {
    let result = part1(3, 1).expect("Error in part 1");
    println!("Part 1: Found {} trees", result);

    let result = part2().expect("Error in part 2");
    println!("Part 2: Found {} trees", result);
}

fn part1(x: usize, y: usize) -> Result<usize, ()> {
    let lines = read_input();
    let width = lines.get(0).unwrap().len();

    let mut trees = 0;
    let mut index = 0;

    let mut iterator = lines.iter();

    while let Some(line) = iterator.next() {
        if line.chars().nth(index).unwrap().eq(&'#') {
            trees += 1;
        }

        for _ in 1..y {
            if iterator.next().is_none() {
                break;
            }
        }

        index = (index + x) % width;
    }

    Ok(trees)
}

fn part2() -> Result<usize, ()> {
    Ok(vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
        .into_iter()
        .map(|(a, b)| part1(a, b).unwrap())
        .product())
}

fn read_input() -> Vec<String> {
    include_str!("../input.txt")
        .lines()
        .map(|x| x.to_string())
        .collect()
}

#[cfg(test)]
mod day03_test {
    use crate::{part1, part2};

    #[test]
    fn test_part_1() {
        assert_eq!(part1(3, 1).unwrap(), 254);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part2().unwrap(), 1666768320);
    }
}

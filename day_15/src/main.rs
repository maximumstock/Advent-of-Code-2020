use std::collections::HashMap;

fn main() {
    let input = parse_input();

    let result = part1(&input, 2020).expect("Error in part 1");
    println!("Part 1: {}", result);

    let result = part2(&input, 30_000_000).expect("Error in part 2");
    println!("Part 2: {}", result);
}

fn parse_input<'a>() -> &'a [usize] {
    &[14, 8, 16, 0, 1, 17]
}

struct SpokenRecord {
    inner: HashMap<usize, Vec<usize>>,
}

impl SpokenRecord {
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }

    pub fn record_number(&mut self, n: usize, index: usize) {
        match self.inner.get_mut(&n) {
            None => {
                self.inner.insert(n, vec![index]);
            }
            Some(indices) => indices.push(index),
        };
    }

    pub fn occurences(&self, n: &usize) -> usize {
        self.inner.get(n).unwrap().len()
    }

    pub fn last_two(&self, n: &usize) -> (usize, usize) {
        let mut iter = self.inner.get(n).unwrap().iter().rev();
        let last = iter.next().unwrap();
        let second_last = iter.next().unwrap();
        (*second_last, *last)
    }
}

fn part1(input: &[usize], target_index: usize) -> Result<usize, ()> {
    let mut spoken = SpokenRecord::new();
    let mut index = 0;
    let mut last_spoken = 0;

    for n in input {
        spoken.record_number(*n, index);
        last_spoken = *n;
        index += 1;
    }

    while index < target_index {
        let n_times_spoken = spoken.occurences(&last_spoken);

        if n_times_spoken == 1 {
            last_spoken = 0;
            spoken.record_number(last_spoken, index);
        } else {
            let (second_last, last) = spoken.last_two(&last_spoken);
            last_spoken = last - second_last;
            spoken.record_number(last_spoken, index);
        }

        index += 1;
    }

    Ok(last_spoken)
}

fn part2(input: &[usize], target_index: usize) -> Result<usize, ()> {
    part1(&input, target_index)
}

#[cfg(test)]
mod day15_test {
    use crate::{parse_input, part1, part2};

    #[test]
    fn test_part_1_examples() {
        assert_eq!(part1(&[0, 3, 6], 2020).unwrap(), 436);
        assert_eq!(part1(&[1, 3, 2], 2020).unwrap(), 1);
        assert_eq!(part1(&[2, 1, 3], 2020).unwrap(), 10);
        assert_eq!(part1(&[1, 2, 3], 2020).unwrap(), 27);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part1(&parse_input(), 2020).unwrap(), 240);
    }

    #[test]
    fn test_part_2_examples() {
        assert_eq!(part1(&[0, 3, 6], 30_000_000).unwrap(), 175594);
        assert_eq!(part1(&[1, 3, 2], 30_000_000).unwrap(), 2578);
        assert_eq!(part1(&[2, 1, 3], 30_000_000).unwrap(), 3544142);
        assert_eq!(part1(&[1, 2, 3], 30_000_000).unwrap(), 261214);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part2(&parse_input(), 30_000_000).unwrap(), 505);
    }
}

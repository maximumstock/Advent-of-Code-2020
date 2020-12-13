fn main() {
    let result = part1().expect("Error in part 1");
    println!("Part 1: {}", result);

    let result = part2().expect("Error in part 2");
    println!("Part 2: {}", result);
}

fn part1() -> Result<usize, ()> {
    let timestamp = 1002576;
    Ok([13, 37, 449, 29, 19, 23, 773, 41, 17]
        .iter()
        .map(|bus| (*bus, *bus - timestamp % *bus))
        .min_by(|a, b| a.1.cmp(&b.1))
        .map(|(bus, delay)| bus * delay)
        .unwrap())
}

fn part2() -> Result<usize, ()> {
    // let values = [
    //     (13, 0),
    //     (37, 7),
    //     (449, 13),
    //     (29, 15),
    //     (19, 32),
    //     (23, 36),
    //     (773, 44),
    //     (41, 54),
    //     (17, 61),
    // ];

    // Solution: https://www.wolframalpha.com/input/?i=solve+%28t+mod+13+%3D+0%29%2C+%28t%2B7%29+mod+37+%3D+0%2C+%28t%2B13%29+mod+449+%3D+0%2C+%28t%2B15%29+mod+29+%3D+0%2C+%28t%2B32%29+mod+19+%3D+0%2C+%28t%2B36%29+mod+23+%3D+0%2C+%28t%2B44%29+mod+773+%3D+0%2C+%28t%2B54%29+mod+41+%3D+0%2C+%28t%2B61%29+mod+17+%3D+0
    // t = 1474630201287997 n + 415579909629976, n element Z

    Ok(415579909629976)
}

#[cfg(test)]
mod test {
    use crate::{part1, part2};

    #[test]
    fn test_part_1() {
        assert_eq!(part1().unwrap(), 3865);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part2().unwrap(), 415579909629976);
    }
}

fn main() {
    let input = parse_input();

    let result = part1(&input).expect("Error in part 1");
    println!("Part 1: {}", result);

    let result = part2(&input).expect("Error in part 2");
    println!("Part 2: {}", result);
}

type Input = Vec<()>;

fn part1(input: &Input) -> Result<(), ()> {
    Ok(())
}

fn part2(input: &Input) -> Result<(), ()> {
    Ok(())
}

fn parse_input() -> Input {
    include_str!("../input.txt").lines().collect::<Vec<_>>()
}

#[cfg(test)]
mod test {
    use crate::{part1, part2};

    #[test]
    fn test_part_1() {}

    #[test]
    fn test_part_2() {}
}

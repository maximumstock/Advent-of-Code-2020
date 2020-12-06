fn main() {
    let input = include_str!("../input.txt");

    let result = part1(&input).expect("Error in part 1");
    println!("Part 1: {}", result);

    let result = part2(&input).expect("Error in part 2");
    println!("Part 2: {}", result);
}

fn part1(input: &str) -> Result<(), ()> {
    Ok(())
}

fn part2(input: &str) -> Result<(), ()> {
    Ok(())
}

#[cfg(test)]
mod test {
    use crate::{part1, part2};

    #[test]
    fn test_part_1() {}

    #[test]
    fn test_part_2() {}
}

fn main() {
    let result = part1().expect("Error in part 1");
    println!("Part 1: {}", result);

    let result = part2().expect("Error in part 2");
    println!("Part 2: {}", result);
}

fn part1() -> Result<(), ()> {
    Ok(())
}

fn part2() -> Result<(), ()> {
    Ok(())
}

fn read_input() -> Vec<()> {
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

use std::collections::{HashMap, HashSet, VecDeque};

fn main() {
    let input = include_str!("../input.txt");

    let result = part1(&input).expect("Error in part 1");
    println!("Part 1: {}", result);

    let result = part2(&input).expect("Error in part 2");
    println!("Part 2: {}", result);
}

fn build_graph(input: &str) -> HashMap<String, Vec<(u8, String)>> {
    input
        .lines()
        .map(|line| {
            let mut parts = line[..line.len() - 1].split(" contain ");
            let container = parts
                .next()
                .unwrap()
                .trim()
                .split(' ')
                .take(2)
                .collect::<Vec<&str>>()
                .join(" ");
            let containees = parts.next().unwrap().split(',').collect::<String>();

            let containees = containees
                .split(' ')
                .collect::<Vec<_>>()
                .chunks(4)
                .filter(|chunk| chunk.len() == 4)
                .map(|chunk| (chunk[0].parse::<u8>().unwrap(), chunk[1..3].join(" ")))
                .collect::<Vec<_>>();

            (container, containees)
        })
        .collect::<HashMap<String, Vec<(u8, String)>>>()
}

fn part1(input: &str) -> Result<usize, ()> {
    let graph = build_graph(input);

    let mut queue = VecDeque::new();
    let mut seen = HashSet::new();

    queue.push_front("shiny gold".to_string());

    while let Some(next) = queue.pop_back() {
        let containers: Vec<String> = graph
            .iter()
            .filter(|(_, v)| v.iter().any(|(_, c)| c.eq(&next)))
            .map(|(a, _)| a.clone())
            .collect::<Vec<_>>();

        for container in containers {
            seen.insert(container.clone());
            queue.push_front(container);
        }
    }

    Ok(seen.len())
}

fn part2(input: &str) -> Result<usize, ()> {
    let graph = build_graph(input);
    Ok(count_bags(&graph, "shiny gold"))
}

fn count_bags(graph: &HashMap<String, Vec<(u8, String)>>, colour: &str) -> usize {
    graph[colour]
        .iter()
        .map(|(a, b)| *a as usize + *a as usize * count_bags(graph, &b))
        .sum()
}

#[cfg(test)]
mod test {
    use crate::{part1, part2};

    #[test]
    fn test_part_1() {
        let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.
            dark orange bags contain 3 bright white bags, 4 muted yellow bags.
            bright white bags contain 1 shiny gold bag.
            muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
            shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
            dark olive bags contain 3 faded blue bags, 4 dotted black bags.
            vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
            faded blue bags contain no other bags.
            dotted black bags contain no other bags.";
        assert_eq!(part1(&input).unwrap(), 4);

        assert_eq!(part1(include_str!("../input.txt")).unwrap(), 259);
    }

    #[test]
    fn test_part_2() {
        let input = "shiny gold bags contain 2 dark red bags.
        dark red bags contain 2 dark orange bags.
        dark orange bags contain 2 dark yellow bags.
        dark yellow bags contain 2 dark green bags.
        dark green bags contain 2 dark blue bags.
        dark blue bags contain 2 dark violet bags.
        dark violet bags contain no other bags.";

        assert_eq!(part2(&input).unwrap(), 126);

        assert_eq!(part2(include_str!("../input.txt")).unwrap(), 45018);
    }
}

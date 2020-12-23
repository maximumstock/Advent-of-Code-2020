use std::{
    collections::{HashMap, HashSet},
    vec,
};

fn main() {
    let input = include_str!("../input.txt");

    let result = part1(&input).expect("Error in part 1");
    println!("Part 1: {}", result);

    let result = part2(&input).expect("Error in part 2");
    println!("Part 2: {}", result);
}

fn part1(input: &str) -> Result<usize, ()> {
    let (foods, rules) = parse_input(&input);
    let allergen_map = build_allergen_map(&rules);

    let unique_ingredients = foods
        .iter()
        .flat_map(|f| f.ingredients.iter())
        .cloned()
        .collect::<HashSet<String>>();

    let allergen_free_ingredients = unique_ingredients
        .iter()
        .filter(|ingredient| allergen_map.values().find(|x| x.eq(ingredient)).is_none())
        .collect::<Vec<_>>();

    let occurences_of_allergen_free_ingredients = allergen_free_ingredients
        .iter()
        .map(|i| count_ingredient_occurences(&foods, &i))
        .sum::<usize>();

    Ok(occurences_of_allergen_free_ingredients)
}

fn part2(input: &str) -> Result<String, ()> {
    let (_, rules) = parse_input(&input);
    let allergen_map = build_allergen_map(&rules);

    let mut dangerous_ingredients = allergen_map.iter().collect::<Vec<_>>();
    dangerous_ingredients.sort_by(|left, right| left.0.cmp(right.0));
    let dangerous_ingredients = dangerous_ingredients
        .iter()
        .map(|(_, v)| (*v).clone())
        .collect::<Vec<_>>()
        .join(",");

    Ok(dangerous_ingredients)
}

fn build_allergen_map(rules: &[(String, HashSet<String>)]) -> HashMap<String, String> {
    let mut occurences: HashMap<String, HashSet<String>> = HashMap::new();
    let mut allergen_map: HashMap<String, String> = HashMap::new();

    for (allergen, ingredients) in rules {
        match occurences.get(allergen) {
            Some(e) => {
                let merged = e
                    .intersection(&ingredients)
                    .cloned()
                    .collect::<HashSet<_>>();
                occurences.insert(allergen.to_string(), merged);
            }
            None => {
                occurences.insert(allergen.to_string(), ingredients.clone());
            }
        }
    }

    loop {
        for (allergen, ingredients) in occurences.iter_mut() {
            let undecided_ingredients = ingredients
                .iter()
                .filter(|i| allergen_map.values().find(|x| x.eq(i)).is_none())
                .cloned()
                .collect::<HashSet<_>>();

            if undecided_ingredients.len() == 1 {
                allergen_map.insert(
                    allergen.clone(),
                    undecided_ingredients.into_iter().next().unwrap(),
                );
                ingredients.clear();
            } else {
                *ingredients = undecided_ingredients;
            }
        }

        occurences = occurences
            .into_iter()
            .filter(|(_, v)| !v.is_empty())
            .collect();

        if occurences.is_empty() {
            break;
        }
    }

    allergen_map
}

fn count_ingredient_occurences(foods: &[Food], i: &str) -> usize {
    foods
        .iter()
        .flat_map(|f| f.ingredients.iter())
        .filter(|ing| ing.eq(&i))
        .count()
}

#[derive(Debug)]
struct Food {
    ingredients: Vec<String>,
    allergens: Vec<String>,
}

fn parse_input(input: &str) -> (Vec<Food>, Vec<(String, HashSet<String>)>) {
    let foods = input
        .lines()
        .map(|x| x.trim().into())
        .collect::<Vec<Food>>();

    let mut rules = vec![];

    for f in &foods {
        for a in &f.allergens {
            let mut set = HashSet::new();
            f.ingredients.iter().for_each(|i| {
                set.insert(i.to_string());
            });
            rules.push((a.clone(), set));
        }
    }

    (foods, rules)
}

impl From<&str> for Food {
    fn from(s: &str) -> Self {
        let mut parts = s.split("(contains ");
        let ingredients = parts
            .next()
            .unwrap()
            .split_whitespace()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        let raw_allergens = parts.next().unwrap();
        let allergens = raw_allergens[0..raw_allergens.len() - 1]
            .split(", ")
            .map(|x| x.to_string())
            .collect::<Vec<String>>();

        Food {
            ingredients,
            allergens,
        }
    }
}

#[cfg(test)]
mod day21_test {
    use crate::{part1, part2};

    #[test]
    fn test_part_1() {
        let input = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
        trh fvjkl sbzzf mxmxvkd (contains dairy)
        sqjhc fvjkl (contains soy)
        sqjhc mxmxvkd sbzzf (contains fish)";

        assert_eq!(part1(&input).unwrap(), 5);

        assert_eq!(part1(include_str!("../input.txt")).unwrap(), 2230);
    }

    #[test]
    fn test_part_2() {
        let input = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
        trh fvjkl sbzzf mxmxvkd (contains dairy)
        sqjhc fvjkl (contains soy)
        sqjhc mxmxvkd sbzzf (contains fish)";

        assert_eq!(part2(&input).unwrap(), "mxmxvkd,sqjhc,fvjkl");
        assert_eq!(
            part2(include_str!("../input.txt")).unwrap(),
            "qqskn,ccvnlbp,tcm,jnqcd,qjqb,xjqd,xhzr,cjxv"
        );
    }
}

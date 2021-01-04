use mod_exp::mod_exp;
use std::collections::HashMap;

fn main() {
    let pkey1 = 8252394;
    let pkey2 = 6269621;

    let result = part1(pkey1, pkey2).expect("Error in part 1");
    println!("Part 1: {}", result);
}

fn part1(pkey1: usize, pkey2: usize) -> Result<usize, ()> {
    let loop_size_card = babystep_giantstep(7, 20201227, pkey1).unwrap();
    let loop_size_door = babystep_giantstep(7, 20201227, pkey2).unwrap();

    let enc_key1 = mod_exp(pkey1, loop_size_door, 20201227);
    let enc_key2 = mod_exp(pkey2, loop_size_card, 20201227);
    assert_eq!(enc_key1, enc_key2);

    Ok(mod_exp(pkey1, loop_size_door, 20201227))
}

/// See https://en.wikipedia.org/wiki/Baby-step_giant-step
#[allow(clippy::many_single_char_names)]
fn babystep_giantstep(g: usize, r#mod: usize, h: usize) -> Option<usize> {
    let mut table = HashMap::new();

    let m = (r#mod as f32).sqrt().ceil() as usize;

    let mut e = 1;
    for j in 0..m {
        table.insert(e, j);
        e = (e * g) % r#mod;
    }

    let factor = mod_exp(g, r#mod - m - 1, r#mod);
    e = h;

    for i in 0..m {
        match table.iter().find(|(k, _)| k.eq(&&e)) {
            Some((_, v)) => return Some(i * m + *v),
            None => {
                e = (e * factor) % r#mod;
            }
        }
    }

    None
}

#[cfg(test)]
mod day25_test {
    use crate::part1;

    #[test]
    fn test_part_1() {
        assert_eq!(part1(8252394, 6269621).unwrap(), 181800);
    }
}

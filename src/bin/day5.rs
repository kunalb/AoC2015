use std::error::Error;
use std::io::{self, Read};
use std::{
    collections::{HashMap, HashSet},
    env,
};

use lazy_static::lazy_static;

fn has_three_vowels(s: &str) -> bool {
    lazy_static! {
        static ref VOWELS: [u64; 26] = {
            let mut vowels: [u64; 26] = [0; 26];
            for ch in &[b'a', b'e', b'i', b'o', b'u'] {
                vowels[*ch as usize - b'a' as usize] = 1;
            }
            vowels
        };
    }

    s.chars()
        .map(|c| VOWELS[c as usize - b'a' as usize])
        .sum::<u64>()
        >= 3
}

fn twice_in_row(s: &str) -> bool {
    s.chars()
        .zip(s.chars().skip(1))
        .map(|(a, b)| a == b)
        .any(|x| x)
}

fn exclude_pairs(s: &str) -> bool {
    lazy_static! {
        static ref PAIRS: HashSet<(char, char)> = {
            let mut pairs: HashSet<(char, char)> = HashSet::new();
            pairs.insert(('a', 'b'));
            pairs.insert(('c', 'd'));
            pairs.insert(('p', 'q'));
            pairs.insert(('x', 'y'));
            pairs
        };
    }

    !s.chars()
        .zip(s.chars().skip(1))
        .map(|pair| PAIRS.contains(&pair))
        .any(|x| x)
}

fn alt_repeat(s: &str) -> bool {
    s.chars()
        .zip(s.chars().skip(2))
        .map(|(a, b)| a == b)
        .any(|x| x)
}

fn duplicate_pairs(s: &str) -> bool {
    let pairs = s
        .chars()
        .zip(s.chars().skip(1))
        .enumerate()
        .collect::<Vec<_>>();
    let mut record: HashMap<(char, char), usize> = HashMap::new();
    for (index, pair) in pairs {
        let mut entry = record.entry(pair);
        let previous = entry.or_insert(index);
        if index - *previous > 1 {
            return true;
        }
    }

    false
}

fn test_rules(rules: Vec<Box<dyn Fn(&str) -> bool>>, s: &str) -> bool {
    rules.iter().map(|rule| rule(s)).all(|x| x)
}

fn solve1(buffer: &str) -> Result<u64, Box<dyn Error>> {
    Ok(buffer
        .trim()
        .split('\n')
        .map(|s| {
            test_rules(
                vec![
                    Box::new(has_three_vowels),
                    Box::new(twice_in_row),
                    Box::new(exclude_pairs),
                ],
                s,
            )
        })
        .map(|a| a as u64)
        .sum())
}

fn solve2(buffer: &str) -> Result<u64, Box<dyn Error>> {
    Ok(buffer
        .trim()
        .split('\n')
        .map(|s| test_rules(vec![Box::new(duplicate_pairs), Box::new(alt_repeat)], s))
        .map(|a| a as u64)
        .sum())
}

fn main() -> Result<(), Box<dyn Error>> {
    let now = std::time::Instant::now();

    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let args: Vec<String> = env::args().collect();
    if args.len() > 1 && args[1] == "2" {
        println!("{}", solve2(&buffer)?);
    } else {
        println!("{}", solve1(&buffer)?);
    }

    eprintln!("Time: {}ms", now.elapsed().as_millis());
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {}

    #[test]
    fn test2() {}
}

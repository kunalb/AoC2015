use std::error::Error;
use std::io::{self, Read};
use std::{boxed::Box, env};

use lazy_static::lazy_static;
use regex::Regex;

fn solve1(buffer: &str) -> Result<i64, Box<dyn Error>> {
    let mut floor = 0;
    for ch in buffer.chars() {
        if ch == ')' {
            floor -= 1;
        } else if ch == '(' {
            floor += 1;
        }
    }

    Ok(floor)
}

fn solve2(buffer: &str) -> Result<i64, Box<dyn Error>> {
    let mut floor = 0;
    for (i, ch) in buffer.chars().enumerate() {
        if ch == ')' {
            floor -= 1;
        } else if ch == '(' {
            floor += 1;
        }

        if floor < 0 {
            return Ok(i as i64 + 1);
        }
    }

    unreachable!();
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

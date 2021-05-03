use std::cmp::max;
use std::error::Error;
use std::io::{self, Read};
use std::{collections::HashSet, env};

use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, PartialEq, Eq)]
enum Action {
    Toggle,
    On,
    Off,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Pt {
    x: u32,
    y: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Rect {
    tl: Pt,
    br: Pt,
}

fn parse(lines: &str) -> Result<Vec<(Action, Pt, Pt)>, Box<dyn Error>> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r#"(?P<x>\d+),(?P<y>\d+)"#).unwrap();
    }

    let mut results = vec![];
    for line in lines.trim().split("\n") {
        let mut points = vec![];
        for captured in RE.captures_iter(line) {
            points.push(Pt {
                x: captured.name("x").unwrap().as_str().parse::<u32>()?,
                y: captured.name("y").unwrap().as_str().parse::<u32>()?,
            });
        }

        let action = if line.starts_with("toggle") {
            Action::Toggle
        } else if line.starts_with("turn on") {
            Action::On
        } else if line.starts_with("turn off") {
            Action::Off
        } else {
            unreachable!(line)
        };

        results.push((action, points[0], points[1]));
    }

    Ok(results)
}

fn solve1(buffer: &str) -> Result<usize, Box<dyn Error>> {
    let mut on = HashSet::new();
    let actions = parse(buffer)?;
    for (action, tl, br) in actions {
        for x in tl.x..=br.x {
            for y in tl.y..=br.y {
                match action {
                    Action::On => {
                        on.insert((x, y));
                    }
                    Action::Off => {
                        on.remove(&(x, y));
                    }
                    Action::Toggle => {
                        if on.contains(&(x, y)) {
                            on.remove(&(x, y));
                        } else {
                            on.insert((x, y));
                        }
                    }
                }
            }
        }
    }

    Ok(on.len())
}

fn solve2(buffer: &str) -> Result<i64, Box<dyn Error>> {
    let mut brightness = std::collections::HashMap::new();
    let actions = parse(buffer)?;
    for (action, tl, br) in actions {
        for x in tl.x..=br.x {
            for y in tl.y..=br.y {
                let pt = (x, y);
                let delta = match action {
                    Action::On => 1,
                    Action::Off => -1,
                    Action::Toggle => 2,
                };

                brightness
                    .entry(pt)
                    .and_modify(|x| {
                        *x = max(*x + delta, 0);
                    })
                    .or_insert(max(delta, 0));
            }
        }
    }

    Ok(brightness.values().map(|&x| x).sum())
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

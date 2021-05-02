use std::{collections::HashSet, env};
use std::error::Error;
use std::io::{self, Read};

fn step(ch: char) -> (i64, i64) {
    match ch {
        '>' => (1, 0),
        '<' => (-1, 0),
        '^' => (0, 1),
        'v' => (0, -1),
        _ => unreachable!(),
    }
}

fn solve1(buffer: &str) -> Result<usize, Box<dyn Error>> {
    let mut visited: HashSet<(i64, i64)> = HashSet::new();
    let mut pos = (0, 0);
    visited.insert(pos);
    for ch in buffer.trim().chars() {
        let s = step(ch);
        pos.0 += s.0;
        pos.1 += s.1;
        visited.insert(pos);
    }
    Ok(visited.len())
}

fn solve2(buffer: &str) -> Result<usize, Box<dyn Error>> {
    let mut visited: HashSet<(i64, i64)> = HashSet::new();
    let mut pos = (0, 0);
    let mut robot_pos = (0, 0);
    visited.insert(pos);

    for (i, ch) in buffer.trim().chars().enumerate() {
        let s = step(ch);
        if i & 1 != 0 {
            robot_pos.0 += s.0;
            robot_pos.1 += s.1;
            visited.insert(robot_pos);
        } else {
            pos.0 += s.0;
            pos.1 += s.1;
            visited.insert(pos);
        }
    }
    Ok(visited.len())
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
use std::env;
use std::error::Error;
use std::io::{self, Read};

use md5;

fn solve(buffer: &str, prefix: &str) -> Result<i64, Box<dyn Error>> {
    let buffer = buffer.trim();
    for i in 1.. {
        let key = buffer.to_string() + &i.to_string();
        let digest = format!("{:x}", md5::compute(key));
        if digest.starts_with(prefix) {
            return Ok(i);
        }
    }
    unreachable!()
}

fn solve1(buffer: &str) -> Result<i64, Box<dyn Error>> {
    solve(buffer, "00000")
}

fn solve2(buffer: &str) -> Result<i64, Box<dyn Error>> {
    solve(buffer, "000000")
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
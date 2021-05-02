use std::error::Error;
use std::io::{self, Read};
use std::{env, num::ParseIntError};

fn parse(lines: &str) -> Result<Vec<Vec<u64>>, Box<dyn Error>> {
    let mut result = vec![];
    for line in lines.trim().split('\n') {
        let mut pieces: Vec<u64> = line
            .split('x')
            .map(|x| x.parse::<u64>())
            .collect::<Result<Vec<u64>, ParseIntError>>()?;
        pieces.sort();
        result.push(pieces);
    }
    Ok(result)
}

fn solve1(buffer: &str) -> Result<u64, Box<dyn Error>> {
    Ok(parse(buffer)?
        .iter()
        .map(|sides| {
            let mut total = 0;
            for i in 0..3 {
                total += 2 * sides[i] * sides[(i + 1) % 3];
            }
            total + sides[0] * sides[1]
        })
        .sum::<u64>())
}

fn solve2(buffer: &str) -> Result<u64, Box<dyn Error>> {
    Ok(parse(buffer)?
        .iter()
        .map(|sides: &Vec<u64>| -> u64 {
            2 * (sides[0] + sides[1]) + sides.iter().product::<u64>()
        })
        .sum::<u64>())
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
use std::env;
use std::error::Error;
use std::io::{self, Read};

use lazy_static::lazy_static;
use regex::Regex;

fn solve1(buffer: &str) -> Result<u64, Box<dyn Error>> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r#"\\x[a-f0-9]{2}"#).unwrap();
    }

    let mut delta = 0;
    for line in buffer.trim().split("\n") {
        let total_chars = line.trim().len();
        let memory_chars = total_chars 
            - 2  // exclude ""
            - line.matches("\\\\").count()
            - &line[1..line.len() - 1].matches("\\\"").count()
            - 3 * RE.find_iter(line).count();

        let replaced_line = &line[1..line.len() - 1];
        let escapes_re = Regex::new(r#"\\["\\]"#).unwrap();
        let replaced_line = escapes_re.replace_all(replaced_line, "0").to_string();
        let hex_re = Regex::new(r#"\\x[a-f0-9]{2}"#).unwrap();
        let replaced_line = hex_re.replace_all(&replaced_line, "0");

        if memory_chars != replaced_line.len() {
            println!("{} {} {} {} {}",
                line,
                replaced_line,
                total_chars,
                memory_chars,
                replaced_line.len(),
            );
        }

        delta += total_chars - replaced_line.len();
    }

    Ok(delta as u64)
}

fn solve2(buffer: &str) -> Result<u64, Box<dyn Error>> {
    let mut delta = 0;
    for line in buffer.trim().split("\n") {
        let new_line = format!("{:?}", line);
        delta += new_line.len() - line.len();
    }

    Ok(delta as u64)
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

use std::{collections::HashMap, env};
use std::error::Error;
use std::io::{self, Read};
use std::vec::Vec;

use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
enum Token {
    Signal(u16),
    Wire(String),
    Assign,
    And,
    Or,
    Not,
    LShift(u16),
    RShift(u16),
}

fn tokenize(line: &str) -> Result<Vec<Token>, Box<dyn Error>> {
    lazy_static! {
        static ref WIRE_NAME_RE: Regex = Regex::new(r"[a-z]+").unwrap();
    }

    let mut pieces = line.trim().split(" ").into_iter();
    let mut tokens: Vec<Token> = vec![];

    loop {
        let token = match pieces.next() {
            None => break,
            Some("AND") => Token::And,
            Some("OR") => Token::Or,
            Some("NOT") => Token::Not,
            Some("->") => Token::Assign,
            Some("LSHIFT") => Token::LShift(pieces.next().unwrap().parse::<u16>()?),
            Some("RSHIFT") => Token::RShift(pieces.next().unwrap().parse::<u16>()?),
            Some(val) => {
                if WIRE_NAME_RE.is_match(val) {
                    Token::Wire(String::from(val))
                } else {
                    Token::Signal(val.parse()?)
                }
            }
        };
        tokens.push(token);
    }

    Ok(tokens)
}

fn parse(lines: &str) -> Result<Vec<Vec<Token>>, Box<dyn Error>> {
    lines.trim().split("\n").map(tokenize).collect()
}

fn eval(
        instructions: &HashMap<String, Vec<Token>>, 
        cache: &mut HashMap<String, u16>,
        name: &str
    ) -> u16 {
    if cache.contains_key(name) {
        return *cache.get(name).unwrap();
    }

    let instruction = &instructions[name];
    let mut value_stack = Vec::new();
    let mut op_stack = Vec::new();
    for token in &instruction[..instruction.len() - 2] {
        match token {
            Token::Signal(x) => { value_stack.push(*x); },
            Token::Wire(next) => { value_stack.push(eval(instructions, cache, next)); },
            Token::Not | Token::And | Token::Or | Token::LShift(_) | Token::RShift(_) => { op_stack.push(token) },
            _ => { unreachable!() },
        }
    }

    let result = if op_stack.is_empty() {
        value_stack.pop().unwrap()
    } else {
        assert!(op_stack.len() == 1, "{:?} {:?}", instructions.get(name), op_stack);
        match op_stack.pop().unwrap() {
            Token::And => {
                value_stack.pop().unwrap() & value_stack.pop().unwrap()
            },
            Token::Or => {
                value_stack.pop().unwrap() | value_stack.pop().unwrap()
            },
            Token::Not => {
                !value_stack.pop().unwrap()
            },
            Token::LShift(x) => {
                value_stack.pop().unwrap() << x
            },
            Token::RShift(x) => {
                value_stack.pop().unwrap() >> x
            },
            _ => { unreachable!() }
        }
    };

    cache.insert(name.to_string(), result);
    result
}

fn eval_a(instructions: Vec<Vec<Token>>) -> u16 {
    let mut assignments: HashMap<String, Vec<Token>> = HashMap::new();
    let mut cache: HashMap<String, u16> = HashMap::new();
    for instruction in instructions.into_iter() {
        if let Token::Wire(name) = &instruction[instruction.len() - 1] {
            assignments.insert(name.to_string(), instruction);
        }
    }

    eval(&assignments, &mut cache, "a")
}

fn solve1(buffer: &str) -> Result<String, Box<dyn Error>> {
    Ok(format!("{:?}", eval_a(parse(buffer)?)))
}

fn eval_a_2(instructions: Vec<Vec<Token>>) -> u16 {
    let duplicate_instructions = instructions.clone();
    let a = eval_a(duplicate_instructions);

    let mut assignments: HashMap<String, Vec<Token>> = HashMap::new();
    let mut cache: HashMap<String, u16> = HashMap::new();
    for instruction in instructions.into_iter() {
        if let Token::Wire(name) = &instruction[instruction.len() - 1] {
            assignments.insert(name.to_string(), instruction);
        }
    }

    cache.insert("b".to_string(), a);
    eval(&assignments, &mut cache, "a")
}

fn solve2(buffer: &str) -> Result<String, Box<dyn Error>> {
    Ok(format!("{:?}", eval_a_2(parse(buffer)?)))
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

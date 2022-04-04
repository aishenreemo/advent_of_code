use regex::Regex;
use std::fs;
use std::collections::{HashSet, HashMap};

#[derive(Debug)]
enum Instruction {
    Off,
    On,
    Toggle,
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct Position {
    column: usize,
    row: usize,
}
#[derive(Debug)]
struct Command {
    instruction: Instruction,
    a: Position,
    b: Position,
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Unable to read file");
    // part1(&input);
    part2(&input);
}

fn part1(input: &str) {
    let tokens = tokenize(input.to_owned());
    let mut memory = HashSet::new();

    for (i, command) in tokens.iter().enumerate() {
        match command.instruction {
            Instruction::Toggle => for column in command.a.column..=command.b.column {
                for row in command.a.row..=command.b.row {
                    let position = Position { column, row };
                    if !memory.contains(&position) {
                        memory.insert(position);
                    } else {
                        memory.remove(&position);
                    }
                }
            },
            Instruction::On =>  for column in command.a.column..=command.b.column {
                for row in command.a.row..=command.b.row {
                    let position = Position { column, row };
                    memory.insert(position);
                }
            } ,
            Instruction::Off => for column in command.a.column..=command.b.column {
                for row in command.a.row..=command.b.row {
                    let position = Position { column, row };
                    memory.remove(&position);
                }
            }         
        }
        println!("done executing command#{i}: {:?}", command);
    }
    println!("Answer is: {}", memory.len());
}

fn part2(input: &str) {
    let tokens = tokenize(input.to_owned());
    let mut memory = HashMap::new();

    for (i, command) in tokens.iter().enumerate() {
        match command.instruction {
            Instruction::Toggle => for column in command.a.column..=command.b.column {
                for row in command.a.row..=command.b.row {
                    let position = Position { column, row };
                    match memory.get_mut(&position) {
                        Some(x) => *x += 2,
                        None => {
                            memory.insert(position, 2);
                        }
                    }    
                }
            },
            Instruction::On =>  for column in command.a.column..=command.b.column {
                for row in command.a.row..=command.b.row {
                    let position = Position { column, row };
                    match memory.get_mut(&position) {
                        Some(x) => *x += 1,
                        None => {
                            memory.insert(position, 1);
                        }
                    }
                }
            } ,
            Instruction::Off => for column in command.a.column..=command.b.column {
                for row in command.a.row..=command.b.row {
                    let position = Position { column, row };
                    match memory.get_mut(&position) {
                        Some(x) if *x >= 1 => *x -= 1,
                        _ => continue,
                    }
                }
            }         
        }
        println!("done executing command#{i}: {:?}", command);
    }

    println!("Answer is: {}", memory.into_values().sum::<usize>());
}

fn tokenize(input: String) -> Vec<Command> {
    let re = Regex::new(r"(toggle|turn (?:off|on)) (\d{0,3},\d{0,3}) through (\d{0,3},\d{0,3})").unwrap();
    let mut output = vec![];
    for mat in re.captures_iter(&input) {
        output.push(Command {
            instruction: parse_instruction(&mat[1]),
            a: parse_position(&mat[2]),
            b: parse_position(&mat[3]),
        })
    }
    output
}

fn parse_instruction(ins: &str) -> Instruction {
    match ins {
        "toggle" => Instruction::Toggle,
        "turn on" => Instruction::On,
        "turn off" => Instruction::Off,
        _ => unreachable!(),
    }
}

fn parse_position(pos: &str) -> Position {
    let mut iter = pos.split(",").map(|s| s.parse::<usize>().expect("Cannot convert to usize."));
    Position {
        column: iter.next().unwrap(),
        row: iter.next().unwrap(),
    }
}

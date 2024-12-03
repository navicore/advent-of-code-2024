use regex::Regex;
use std::error::Error;

const DAY03: &str = include_str!("../resources/day03_input.txt");

type Instruction = (String, i32, i32);
type CondInstruction = (String, Option<(i32, i32)>);

fn get_instructions(block: &str) -> Result<Vec<Instruction>, Box<dyn Error>> {
    let re = Regex::new(r"(\w+)\((\d*)?,?(\d*)?\)")?;
    let mut instructions = Vec::new();

    for cap in re.captures_iter(block) {
        let operator = cap.get(1).map_or("", |m| m.as_str());
        let num1 = cap
            .get(2)
            .map_or(0, |m| m.as_str().parse::<i32>().unwrap_or(0));
        let num2 = cap
            .get(3)
            .map_or(0, |m| m.as_str().parse::<i32>().unwrap_or(0));
        instructions.push((operator.to_string(), num1, num2));
    }

    Ok(instructions)
}

fn part1(memory: &[&str]) -> Result<(), Box<dyn Error>> {
    let mut answer = 0;
    for block in memory {
        let instructions = get_instructions(block)?;
        for (op, num1, num2) in instructions {
            if op == "mul" {
                answer += num1 * num2;
            }
        }
    }
    println!("Answer for sum of all muls: {answer}");
    Ok(())
}

fn get_instructions_conditionally(block: &str) -> Result<Vec<CondInstruction>, Box<dyn Error>> {
    let re = Regex::new(r"([a-zA-Z][\w']*)\((\d*)?,?(\d*)?\)")?;

    let mut instructions = Vec::new();

    for cap in re.captures_iter(block) {
        let operator = cap.get(1).map_or("", |m| m.as_str());
        let num1 = cap.get(2).and_then(|m| m.as_str().parse::<i32>().ok());
        let num2 = cap.get(3).and_then(|m| m.as_str().parse::<i32>().ok());
        let params = match (num1, num2) {
            (Some(n1), Some(n2)) => Some((n1, n2)),
            _ => None,
        };
        instructions.push((operator.to_string(), params));
    }

    Ok(instructions)
}

fn part2(memory: &[&str]) -> Result<(), Box<dyn Error>> {
    let mut answer = 0;
    let mut enabled = true;
    for block in memory {
        let instructions = get_instructions_conditionally(block)?;
        for (op, args) in instructions {
            match op.as_str() {
                "do" if args.is_none() => enabled = true,
                "do" if args.is_some() => panic!(),
                "don't" if args.is_none() => enabled = false,
                "don't" if args.is_some() => panic!(),
                "mul" if enabled => {
                    if let Some((num1, num2)) = args {
                        answer += num1 * num2;
                    }
                }
                _ => (),
            };
        }
    }
    println!("Answer for sum of all conditional muls: {answer}");
    Ok(())
}

pub fn run() -> Result<(), Box<dyn Error>> {
    let memory: Vec<&str> = DAY03.lines().collect();

    part1(&memory)?;
    part2(&memory)?;
    Ok(())
}

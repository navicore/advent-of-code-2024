use regex::Regex;

const DAY03: &str = include_str!("../resources/day03_input.txt");

fn get_instructions(block: &str) -> Vec<(&str, i32, i32)> {
    let re = Regex::new(r"(\w+)\((\d+),(\d+)\)").unwrap();
    let mut instructions = Vec::new();

    for cap in re.captures_iter(block) {
        let operator = cap.get(1).map_or("", |m| m.as_str());
        let num1 = cap
            .get(2)
            .map_or(0, |m| m.as_str().parse::<i32>().unwrap_or(0));
        let num2 = cap
            .get(3)
            .map_or(0, |m| m.as_str().parse::<i32>().unwrap_or(0));
        instructions.push((operator, num1, num2));
    }

    instructions
}

fn part1(memory: &[&str]) {
    let mut answer = 0;
    for block in memory {
        let instructions = get_instructions(block);
        for (op, num1, num2) in instructions {
            if op == "mul" {
                answer += num1 * num2;
            }
        }
    }
    println!("Answer for sum of all muls: {answer}");
}

pub fn run() {
    let memory: Vec<&str> = DAY03.lines().collect();

    part1(&memory);
}

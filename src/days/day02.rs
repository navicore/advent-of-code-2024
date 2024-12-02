const DAY02: &str = include_str!("../resources/day02_input.txt");

fn is_safe(report: &[i32]) -> bool {
    let mut prev_val: Option<i32> = None;
    let mut prev_dir: i32 = 0;
    for &b in report.iter() {
        if let Some(prev_val) = prev_val {
            let diff = b - prev_val;
            let dir = match diff {
                d if d > 0 => 1,
                d if d < 0 => -1,
                _ => 0,
            };
            match (prev_dir, dir, diff.abs()) {
                (_, 0, _) => return false,
                (_, _, d) if d > 3 => return false,
                (0, _, _) => prev_dir = dir,
                (pd, d, _) if pd != d => return false,
                _ => (),
            }
        }
        prev_val = Some(b);
    }
    true
}

fn part1(reports: &[Vec<i32>]) {
    let safe_count = reports.iter().filter(|report| is_safe(report)).count();
    println!("Number of safe reports: {}", safe_count);
}

pub fn run() {
    // parse file into Vec<Vec<i32>>
    let reports: Vec<Vec<i32>> = DAY02
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<i32>().expect("Invalid number"))
                .collect()
        })
        .collect();

    part1(&reports);
}

const DAY02: &str = include_str!("../resources/day02_input.txt");

fn is_safe(report: &[i32]) -> bool {
    let mut prev_val: Option<i32> = None;
    let mut prev_dir: i32 = 0;
    for b in report {
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
        prev_val = Some(*b);
    }
    true
}

fn part1(reports: &[Vec<i32>]) {
    let safe_count = reports.iter().filter(|report| is_safe(report)).count();
    println!("Number of safe reports: {safe_count}");
}

fn part2(reports: &[Vec<i32>]) {
    let safe_count = reports
        .iter()
        .filter(|report| is_safe_with_one_removal(report))
        .count();
    println!("Number of safe reports with problem dampener: {safe_count}");
}

fn is_safe_with_one_removal(report: &[i32]) -> bool {
    if is_safe(report) {
        return true;
    }
    for i in 0..report.len() {
        let mut modified_report = report.to_vec();
        modified_report.remove(i);
        if is_safe(&modified_report) {
            return true;
        }
    }
    false
}

pub fn run() {
    // parse file into Vec<Vec<i32>>
    let reports: Vec<Vec<i32>> = DAY02
        .lines()
        .filter_map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<i32>())
                .collect::<Result<Vec<i32>, _>>()
                .ok()
        })
        .collect();

    part1(&reports);
    part2(&reports);
}

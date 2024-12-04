// tries:
// 1651 is too low
const INPUT: &str = include_str!("../resources/day04_input.txt");
//const INPUT: &str = include_str!("../resources/day04_example1.txt");
//const INPUT: &str = include_str!("../resources/day04_example2.txt"); // this works

pub fn run() {
    let puzzle: Vec<String> = INPUT.lines().map(|line| line.to_string()).collect();

    part1(puzzle);
    // part2(&memory)?;
}

fn part1(puzzle: Vec<String>) {
    // create a collection of sequences of characters for each way the xmas chars can be hidden
    //   * [x] forwards (each line)
    //   * [x] backwards (each line)
    //   * [ ] down (each column)
    //   * [ ] up (each column)
    //   * [ ] diagonal down (per column per row less the angle over margins)
    //   * [ ] diagonal up (per column per row less the angle over margins)
    // count and sum the occurrences of xmas in each sequence

    let mut strings = vec![String::new()];

    // forwards
    let rows = puzzle.clone();
    strings.append(&mut rows.clone());

    // down
    let down = columns(puzzle.clone());
    strings.append(&mut down.clone());

    // diagonal down
    let diagonal_down = diagonal_down(puzzle.clone());
    strings.append(&mut diagonal_down.clone());

    // diagonal up
    let diagonal_up = diagonal_up(puzzle.clone());
    strings.append(&mut diagonal_up.clone());

    println!("the part 1 answer is {}", count_xmas(strings));
}

fn count_xmas(strings: Vec<String>) -> usize {
    let target = "XMAS";
    let mut count = 0;

    for s in strings {
        count += s.matches(target).count();
        let reverse_target = target.chars().rev().collect::<String>();
        count += s.matches(reverse_target.as_str()).count();
    }

    count
}

fn columns(puzzle: Vec<String>) -> Vec<String> {
    if puzzle.is_empty() {
        return vec![];
    }

    let num_columns = puzzle[0].len();
    let mut result = vec![String::new(); num_columns];

    for row in puzzle {
        for (i, ch) in row.chars().enumerate() {
            result[i].push(ch);
        }
    }

    result
}

fn diagonal_down(puzzle: Vec<String>) -> Vec<String> {
    if puzzle.is_empty() {
        return vec![];
    }

    let num_rows = puzzle.len();
    let num_columns = puzzle[0].len();
    let mut result = Vec::new();

    // Diagonals starting from the first row
    for col in 0..num_columns {
        let mut diagonal = String::new();
        let mut r = 0;
        let mut c = col;
        while r < num_rows && c < num_columns {
            diagonal.push(puzzle[r].chars().nth(c).unwrap());
            r += 1;
            c += 1;
        }
        result.push(diagonal);
    }

    // Diagonals starting from the first column (excluding the first element)
    for row in 1..num_rows {
        let mut diagonal = String::new();
        let mut r = row;
        let mut c = 0;
        while r < num_rows && c < num_columns {
            diagonal.push(puzzle[r].chars().nth(c).unwrap());
            r += 1;
            c += 1;
        }
        result.push(diagonal);
    }

    result
}

fn diagonal_up(puzzle: Vec<String>) -> Vec<String> {
    if puzzle.is_empty() {
        return vec![];
    }

    let num_rows = puzzle.len();
    let num_columns = puzzle[0].len();
    let mut result = Vec::new();

    // Diagonals starting from the last row
    for col in 0..num_columns {
        let mut diagonal = String::new();
        let mut r = num_rows - 1;
        let mut c = col;
        while r < num_rows && c < num_columns {
            diagonal.push(puzzle[r].chars().nth(c).unwrap());
            if r == 0 {
                break;
            }
            r -= 1;
            c += 1;
        }
        result.push(diagonal);
    }

    // Diagonals starting from the first column (excluding the last element)
    for row in (0..num_rows - 1).rev() {
        let mut diagonal = String::new();
        let mut r = row;
        let mut c = 0;
        while r < num_rows && c < num_columns {
            diagonal.push(puzzle[r].chars().nth(c).unwrap());
            if r == 0 {
                break;
            }
            r -= 1;
            c += 1;
        }
        result.push(diagonal);
    }

    result
}

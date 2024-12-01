use std::collections::HashMap;

const DAY01: &str = include_str!("../resources/day01_input.txt");

fn part1(col1: &[i32], col2: &[i32]) {
    // Combine the two sorted vectors into a vector of tuples
    let paired: Vec<(&i32, &i32)> = col1.iter().zip(col2).collect();

    // Calculate the sum of absolute differences
    #[allow(clippy::cast_possible_truncation)]
    let abs_diff_sum: i32 = paired
        .iter()
        .map(|(a, b)| (*a - *b).abs()) // Compute absolute difference
        .sum(); // Sum all the differences

    // Print the result
    println!("Sum of absolute differences: {abs_diff_sum}");
}

fn part2(col1: &[i32], col2: &[i32]) {
    // Count occurrences of each value in col2
    let col2_counts: HashMap<i32, usize> = col2.iter().fold(HashMap::new(), |mut acc, &val| {
        *acc.entry(val).or_insert(0) += 1;
        acc
    });

    // Compute the weighted sum
    #[allow(clippy::cast_possible_wrap, clippy::cast_possible_truncation)]
    let weighted_sum: i32 = col1
        .iter()
        .map(|&val| val * (*col2_counts.get(&val).unwrap_or(&0) as i32))
        .sum();

    // Print the result
    println!("Weighted sum: {weighted_sum}");
}

pub fn run() {
    // Parse the &str into two separate vectors of integers
    let (mut col1, mut col2): (Vec<i32>, Vec<i32>) = DAY01
        .lines()
        .filter_map(|line| {
            let mut nums = line
                .split_whitespace()
                .filter_map(|n| n.parse::<i32>().ok());
            Some((nums.next()?, nums.next()?)) // Take two integers per line
        })
        .unzip(); // Unzip into two separate vectors

    // Sort the two vectors
    col1.sort_unstable();
    col2.sort_unstable();

    part1(&col1, &col2);
    part2(&col1, &col2);
}

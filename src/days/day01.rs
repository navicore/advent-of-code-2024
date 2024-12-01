const DAY01: &str = include_str!("../resources/day01_input.txt");

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

    // Assert both vectors have the same length
    assert_eq!(col1.len(), col2.len(), "Columns have different lengths!");

    // Combine the two sorted vectors into a vector of tuples
    let paired: Vec<(i32, i32)> = col1.into_iter().zip(col2).collect();

    // Calculate the sum of absolute differences
    let abs_diff_sum: i32 = paired
        .iter()
        .map(|(a, b)| (a - b).abs()) // Compute absolute difference
        .sum(); // Sum all the differences

    // Print the result
    println!("Sum of absolute differences: {}", abs_diff_sum);
}

use std::collections::HashMap;

fn calculate_distance(mut left: Vec<i32>, mut right: Vec<i32>) {
    left.sort_unstable();
    right.sort_unstable();

    let distance_total: i32 = left
        .into_iter()
        .zip(right)
        .map(|(a, b)| (a - b).abs())
        .sum();

    println!("Distance total: {distance_total}");
}

fn calculate_similarity(mut left: Vec<i32>, mut right: Vec<i32>) {
    left.sort_unstable();
    right.sort_unstable();

    let mut left_index: usize = 0;
    let mut right_index: usize = 0;
    let mut cache = HashMap::new();

    let mut similarity_score: i32 = 0;
    while left_index < left.len() && right_index < right.len() {
        let mut identical_count = 0;
        let current_number = left[left_index];

        while right_index < right.len() && current_number > right[right_index] {
            right_index += 1;
        }

        while right_index < right.len() && right[right_index] == current_number {
            identical_count += 1;
            right_index += 1;
        }

        similarity_score += identical_count * current_number;
        similarity_score += cache.get(&current_number).unwrap_or(&0) * current_number;
        cache.insert(current_number, identical_count);

        left_index += 1;
    }

    println!("Similarity score: {similarity_score}");
}

fn main() {
    let input = include_str!("input.txt");
    let lines = input.lines();

    let (left, right): (Vec<i32>, Vec<i32>) = lines
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            assert!(parts.len() == 2, "Invalid input, expected 2 numbers");

            let nums: Vec<i32> = parts
                .into_iter()
                .map(|x| x.parse().expect("Expected a number"))
                .collect();
            (nums[0], nums[1])
        })
        .unzip();

    calculate_distance(left.clone(), right.clone());
    calculate_similarity(left, right);
}

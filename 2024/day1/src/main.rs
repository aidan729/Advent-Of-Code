use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::collections::HashMap;

fn main() -> io::Result<()> {
    let file_path = "src/input.txt";

    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut left_list = Vec::new();
    let mut right_list = Vec::new();

    for line in reader.lines() {
        let line = line?; // Unwrap the line
        let mut parts = line.split_whitespace(); // Split line into parts
        if let (Some(left), Some(right)) = (parts.next(), parts.next()) {
            if let (Ok(left_val), Ok(right_val)) = (left.parse::<i32>(), right.parse::<i32>()) {
                left_list.push(left_val);
                right_list.push(right_val);
            }
        }
    }

    let mut right_count = HashMap::new();
    for num in &right_list {
        *right_count.entry(num).or_insert(0) += 1;
    }

    let similarity_score: i32 = left_list.iter().map(|num| num * right_count.get(num).unwrap_or(&0)).sum();

    println!("Similarity Score: {}", similarity_score);

    left_list.sort();
    right_list.sort();

    let total_distance: i32 = left_list
        .iter()
        .zip(right_list.iter())
        .map(|(l, r)| (l - r).abs())
        .sum();

    println!("Total Distance: {}", total_distance);

    Ok(())
}
use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("src/input.txt").expect("Failed to read input");
    let stones: Vec<u64> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let part1 = solve(&stones, 25);
    let part2 = solve(&stones, 75);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn solve(stones: &[u64], blinks: usize) -> usize {
    let mut cache = HashMap::new();
    stones
        .iter()
        .map(|&stone| count_stones(stone, blinks, &mut cache))
        .sum()
}

fn count_stones(stone: u64, blinks_remaining: usize, cache: &mut HashMap<(u64, usize), usize>) -> usize {
    // Base case: no more blinks
    if blinks_remaining == 0 {
        return 1;
    }

    // Check cache
    if let Some(&result) = cache.get(&(stone, blinks_remaining)) {
        return result;
    }

    // Apply transformation rules
    let result = if stone == 0 {
        // Rule 1: 0 becomes 1
        count_stones(1, blinks_remaining - 1, cache)
    } else {
        let digits = stone.to_string();
        let len = digits.len();

        if len % 2 == 0 {
            // Rule 2: Even number of digits - split in half
            let mid = len / 2;
            let left = digits[..mid].parse::<u64>().unwrap();
            let right = digits[mid..].parse::<u64>().unwrap();

            count_stones(left, blinks_remaining - 1, cache)
                + count_stones(right, blinks_remaining - 1, cache)
        } else {
            // Rule 3: Multiply by 2024
            count_stones(stone * 2024, blinks_remaining - 1, cache)
        }
    };

    // Cache the result
    cache.insert((stone, blinks_remaining), result);
    result
}
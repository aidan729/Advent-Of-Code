use std::fs;

fn main() {
    let input = fs::read_to_string("src/input.txt").expect("Failed to read input");
    let part1 = solve_part1(&input);
    let part2 = solve_part2(&input);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn solve_part1(input: &str) -> u64 {
    input
        .trim()
        .split(',')
        .flat_map(parse_range)
        .filter(|&id| is_repeated_exactly_twice(id))
        .sum()
}

fn solve_part2(input: &str) -> u64 {
    input
        .trim()
        .split(',')
        .flat_map(parse_range)
        .filter(|&id| is_repeated_at_least_twice(id))
        .sum()
}

fn parse_range(range: &str) -> impl Iterator<Item = u64> + '_ {
    let parts: Vec<&str> = range.split('-').collect();
    let start = parts[0].parse::<u64>().unwrap();
    let end = parts[1].parse::<u64>().unwrap();
    start..=end
}

fn is_repeated_exactly_twice(n: u64) -> bool {
    let s = n.to_string();
    let len = s.len();

    // must be even length to be splittable into two identical halves
    if len % 2 != 0 {
        return false;
    }

    // split in half and check if both halves are identical
    let mid = len / 2;
    let (left, right) = s.split_at(mid);

    // check for leading zeros (which would make it invalid)
    // if right half has leading zeros its not a valid number pattern
    if right.starts_with('0') && mid > 1 {
        return false;
    }

    left == right
}

fn is_repeated_at_least_twice(n: u64) -> bool {
    let s = n.to_string();
    let len = s.len();

    // try all possible pattern lengths from 1 to len/2
    // the pattern must repeat at least twice, so max pattern length is len/2
    for pattern_len in 1..=len / 2 {
        // the total length must be evenly divisible by the pattern length
        if len % pattern_len != 0 {
            continue;
        }

        // check if the string is made of this pattern repeated
        let pattern = &s[..pattern_len];

        // dont allow patterns with leading zeros (except single "0")
        if pattern.starts_with('0') && pattern_len > 1 {
            continue;
        }

        // check if repeating this pattern gives us the full string
        let repetitions = len / pattern_len;
        if repetitions >= 2 && pattern.repeat(repetitions) == s {
            return true;
        }
    }

    false
}
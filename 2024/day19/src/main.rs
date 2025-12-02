use std::collections::HashMap;
use std::fs;

fn main() {
    let input = fs::read_to_string("src/input.txt").expect("Failed to read input");
    let part1 = solve_part1(&input);
    let part2 = solve_part2(&input);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn solve_part1(input: &str) -> usize {
    let (patterns, designs) = parse_input(input);
    let mut cache = HashMap::new();

    designs
        .iter()
        .filter(|design| can_make(design, &patterns, &mut cache))
        .count()
}

fn solve_part2(input: &str) -> usize {
    let (patterns, designs) = parse_input(input);
    let mut cache = HashMap::new();

    designs
        .iter()
        .map(|design| count_ways(design, &patterns, &mut cache))
        .sum()
}

fn can_make(design: &str, patterns: &[String], cache: &mut HashMap<String, bool>) -> bool {
    if design.is_empty() {
        return true;
    }

    if let Some(&result) = cache.get(design) {
        return result;
    }

    let result = patterns.iter().any(|pattern| {
        design.starts_with(pattern) && can_make(&design[pattern.len()..], patterns, cache)
    });

    cache.insert(design.to_string(), result);
    result
}

fn count_ways(design: &str, patterns: &[String], cache: &mut HashMap<String, usize>) -> usize {
    if design.is_empty() {
        return 1;
    }

    if let Some(&result) = cache.get(design) {
        return result;
    }

    let result = patterns
        .iter()
        .filter(|pattern| design.starts_with(pattern.as_str()))
        .map(|pattern| count_ways(&design[pattern.len()..], patterns, cache))
        .sum();

    cache.insert(design.to_string(), result);
    result
}

fn parse_input(input: &str) -> (Vec<String>, Vec<String>) {
    let mut lines = input.lines();

    let patterns = lines
        .next()
        .unwrap()
        .split(", ")
        .map(|s| s.to_string())
        .collect();

    lines.next(); // skip blank line

    let designs = lines.map(|s| s.to_string()).collect();

    (patterns, designs)
}

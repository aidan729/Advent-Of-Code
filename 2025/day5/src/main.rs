use std::fs;

fn main() {
    let input = fs::read_to_string("src/input.txt").expect("Failed to read input");
    let part1 = solve_part1(&input);
    let part2 = solve_part2(&input);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}
fn solve_part1(input: &str) -> usize {
    let (ranges, ingredients) = parse_input(input);

    ingredients
        .iter()
        .filter(|&&id| ranges.iter().any(|&(start, end)| id >= start && id <= end))
        .count()
}

fn solve_part2(input: &str) -> u64 {
    let (mut ranges, _) = parse_input(input);

    // sort ranges by the starting positiong 
    ranges.sort_unstable();

    // merge the overlaping ranges
    let mut merged: Vec<(u64, u64)> = Vec::new();
    for(start, end) in ranges {
        if let Some((_, last_end)) = merged.last_mut() {
            if start <= *last_end + 1 {
                // overlapping or adjacent, merge
                *last_end = (*last_end).max(end);
            } else {
                // gap, add new range
                merged.push((start, end));
            }
        } else {
            merged.push((start, end));
        }
    }

    // sum of all IDs in merged ranges
    merged.iter().map(|(start, end)| end - start + 1).sum()
}

fn parse_input(input: &str) -> (Vec<(u64, u64)>, Vec<u64>) {
    let input = input.trim();
    let mut lines = input.lines();

    let mut ranges = Vec::new();
    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }
        let mut parts = line.split("-");
        let start = parts.next().unwrap().parse().unwrap();
        let end = parts.next().unwrap().parse().unwrap();
        ranges.push((start, end));
    }

    let ingredients = lines.map(|line| line.parse().unwrap()).collect();

    (ranges, ingredients)
}
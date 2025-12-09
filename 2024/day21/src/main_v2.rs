use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("src/input.txt").unwrap();
    println!("Part 1: {}", solve(&input, 2));
    println!("Part 2: {}", solve(&input, 25));
}

fn get_numeric_positions() -> HashMap<char, (i32, i32)> {
    let mut map = HashMap::new();
    map.insert('7', (0, 0));
    map.insert('8', (0, 1));
    map.insert('9', (0, 2));
    map.insert('4', (1, 0));
    map.insert('5', (1, 1));
    map.insert('6', (1, 2));
    map.insert('1', (2, 0));
    map.insert('2', (2, 1));
    map.insert('3', (2, 2));
    map.insert('0', (3, 1));
    map.insert('A', (3, 2));
    map
}

fn get_directional_positions() -> HashMap<char, (i32, i32)> {
    let mut map = HashMap::new();
    map.insert('^', (0, 1));
    map.insert('A', (0, 2));
    map.insert('<', (1, 0));
    map.insert('v', (1, 1));
    map.insert('>', (1, 2));
    map
}

// Generate all shortest path sequences from start to end
fn get_all_paths(
    start: (i32, i32),
    end: (i32, i32),
    gap: (i32, i32),
) -> Vec<String> {
    if start == end {
        return vec!["A".to_string()];
    }

    let dr = end.0 - start.0;
    let dc = end.1 - start.1;

    let vertical = if dr > 0 {
        "v".repeat(dr as usize)
    } else {
        "^".repeat((-dr) as usize)
    };

    let horizontal = if dc > 0 {
        ">".repeat(dc as usize)
    } else {
        "<".repeat((-dc) as usize)
    };

    let h_first_pos = (start.0, end.1);
    let v_first_pos = (end.0, start.1);

    let mut paths = Vec::new();

    // Try horizontal first
    if h_first_pos != gap {
        paths.push(format!("{}{}A", horizontal, vertical));
    }

    // Try vertical first (only if different from horizontal first)
    if v_first_pos != gap && dr != 0 && dc != 0 {
        paths.push(format!("{}{}A", vertical, horizontal));
    }

    paths
}

// Calculate the cost (number of button presses) to type a sequence
// at a given depth of directional keypads
fn sequence_cost(
    seq: &str,
    depth: usize,
    cache: &mut HashMap<(String, usize), usize>,
) -> usize {
    if depth == 0 {
        return seq.len();
    }

    let cache_key = (seq.to_string(), depth);
    if let Some(&cost) = cache.get(&cache_key) {
        return cost;
    }

    let directional_pos = get_directional_positions();
    let gap = (0, 0);

    let mut total_cost = 0;
    let mut current = 'A';

    for target in seq.chars() {
        let start_pos = directional_pos[&current];
        let end_pos = directional_pos[&target];
        let paths = get_all_paths(start_pos, end_pos, gap);

        // Try all paths and pick the one with minimum cost
        let min_cost = paths
            .iter()
            .map(|path| sequence_cost(path, depth - 1, cache))
            .min()
            .unwrap();

        total_cost += min_cost;
        current = target;
    }

    cache.insert(cache_key, total_cost);
    total_cost
}

fn solve(input: &str, num_directional_robots: usize) -> usize {
    let numeric_pos = get_numeric_positions();
    let numeric_gap = (3, 0);
    let mut cache = HashMap::new();

    let codes: Vec<&str> = input.lines().collect();
    let mut total_complexity = 0;

    for code in codes {
        let mut current = 'A';
        let mut total_length = 0;

        for target in code.chars() {
            let start_pos = numeric_pos[&current];
            let end_pos = numeric_pos[&target];
            let paths = get_all_paths(start_pos, end_pos, numeric_gap);

            // Try all paths and pick the one with minimum cost
            let min_cost = paths
                .iter()
                .map(|path| sequence_cost(path, num_directional_robots, &mut cache))
                .min()
                .unwrap();

            total_length += min_cost;
            current = target;
        }

        let numeric_part: usize = code[..code.len() - 1].parse().unwrap();
        let complexity = total_length * numeric_part;

        println!("{}: {} * {} = {}", code, total_length, numeric_part, complexity);
        total_complexity += complexity;
    }

    total_complexity
}

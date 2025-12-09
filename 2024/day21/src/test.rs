use std::collections::{HashMap, VecDeque};

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

fn find_shortest_paths(
    start: char,
    end: char,
    positions: &HashMap<char, (i32, i32)>,
    gap: (i32, i32),
) -> Vec<String> {
    if start == end {
        return vec!["A".to_string()];
    }

    let start_pos = positions[&start];
    let end_pos = positions[&end];

    let mut queue = VecDeque::new();
    queue.push_back((start_pos, String::new()));

    let mut shortest_len = None;
    let mut paths = Vec::new();

    while let Some((pos, path)) = queue.pop_front() {
        if pos == end_pos {
            if let Some(len) = shortest_len {
                if path.len() > len {
                    break;
                }
            } else {
                shortest_len = Some(path.len());
            }
            paths.push(format!("{}A", path));
            continue;
        }

        if let Some(len) = shortest_len {
            if path.len() >= len {
                continue;
            }
        }

        for (dr, dc, dir_char) in [(0, 1, '>'), (0, -1, '<'), (1, 0, 'v'), (-1, 0, '^')] {
            let new_pos = (pos.0 + dr, pos.1 + dc);

            if new_pos == gap {
                continue;
            }

            if positions.values().any(|&p| p == new_pos) {
                let mut new_path = path.clone();
                new_path.push(dir_char);
                queue.push_back((new_pos, new_path));
            }
        }
    }

    paths
}

fn get_shortest_sequence_length(code: &str, num_directional_robots: usize) -> usize {
    let numeric_pos = get_numeric_positions();
    let directional_pos = get_directional_positions();
    let numeric_gap = (3, 0);
    let directional_gap = (0, 0);

    let mut current = 'A';
    let mut sequences = vec![String::new()];

    for target in code.chars() {
        let paths = find_shortest_paths(current, target, &numeric_pos, numeric_gap);

        let mut new_sequences = Vec::new();
        for seq in &sequences {
            for path in &paths {
                let mut new_seq = seq.clone();
                new_seq.push_str(path);
                new_sequences.push(new_seq);
            }
        }
        sequences = new_sequences;
        current = target;
    }

    // Expand through directional robot layers, exploring all possibilities
    for layer in 0..num_directional_robots {
        let mut new_sequences = Vec::new();
        let mut min_len = None;

        for seq in sequences {
            let mut current = 'A';
            let mut layer_sequences = vec![String::new()];

            for target in seq.chars() {
                let paths = find_shortest_paths(current, target, &directional_pos, directional_gap);

                let mut new_layer_sequences = Vec::new();
                for layer_seq in &layer_sequences {
                    for path in &paths {
                        let mut new_seq = layer_seq.clone();
                        new_seq.push_str(path);
                        new_layer_sequences.push(new_seq);
                    }
                }
                layer_sequences = new_layer_sequences;
                current = target;
            }

            // Add all expanded sequences from this original sequence
            for expanded in layer_sequences {
                if let Some(min) = min_len {
                    if expanded.len() > min {
                        continue;
                    }
                }
                if min_len.is_none() || expanded.len() < min_len.unwrap() {
                    min_len = Some(expanded.len());
                }
                new_sequences.push(expanded);
            }
        }

        // Keep only sequences of minimum length
        if let Some(min) = min_len {
            new_sequences.retain(|s| s.len() == min);
        }

        sequences = new_sequences;
    }

    sequences.iter().map(|s| s.len()).min().unwrap()
}

fn main() {
    let test_input = "029A\n980A\n179A\n456A\n379A";
    let codes: Vec<&str> = test_input.lines().collect();
    let mut total_complexity = 0;

    for code in codes {
        let sequence_length = get_shortest_sequence_length(code, 2);
        let numeric_part: usize = code[..code.len()-1].parse().unwrap();
        let complexity = sequence_length * numeric_part;

        println!("{}: {} * {} = {}", code, sequence_length, numeric_part, complexity);
        total_complexity += complexity;
    }

    println!("Total: {}", total_complexity);
    println!("Expected: 126384");
}

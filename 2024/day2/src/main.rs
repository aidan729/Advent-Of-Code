use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn is_safe_report(levels: &[i32]) -> bool {
    if levels.len() < 2 {
        return false; // A report must have at least 2 levels
    }

    let mut is_increasing = None;

    for window in levels.windows(2) {
        let diff = window[1] - window[0];
        if diff.abs() < 1 || diff.abs() > 3 {
            return false;
        }

        match is_increasing {
            None => {
                is_increasing = Some(diff > 0);
            }
            Some(true) => {
                if diff < 0 {
                    return false;
                }
            }
            Some(false) => {
                if diff > 0 {
                    return false;
                }
            }
        }
    }

    true
}

fn can_be_safe_with_removal(levels: &[i32]) -> bool {
    if levels.len() <= 2 {
        return false; 
    }

    for i in 0..levels.len() {
        let mut modified_levels = levels.to_vec();
        modified_levels.remove(i);

        if is_safe_report(&modified_levels) {
            return true;
        }
    }

    false
}

fn main() -> io::Result<()> {
    let file_path = "src/input.txt";

    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut safe_count = 0;

    for line in reader.lines() {
        let line = line?;

        let levels: Vec<i32> = line.split_whitespace().filter_map(|n| n.parse::<i32>().ok()).collect();

        if is_safe_report(&levels) || can_be_safe_with_removal(&levels) {
            safe_count += 1;
        }
    }

    println!("Number of safe reports: {}", safe_count);

    Ok(())
}

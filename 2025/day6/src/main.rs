use std::fs;

fn main() {
    let input = fs::read_to_string("src/input.txt").expect("Failed to read input");
    let part1 = solve_part1(&input);
    let part2 = solve_part2(&input);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn solve_part1(input: &str) -> i64 {
    let lines: Vec<&str> = input.lines().collect();

    // find max width
    let max_width = lines.iter().map(|line| line.len()).max().unwrap();

    // identify columns that are all spaces (separators)
    let mut is_separator = vec![true; max_width];
    for line in &lines {
        for (i, ch) in line.chars().enumerate() {
            if ch != ' ' {
                is_separator[i] = false;
            }
        }
    }

    // find problem boundaries
    let mut problems = Vec::new();
    let mut start = None;

    for i in 0..max_width {
        if is_separator[i] {
            if let Some(s) = start {
                problems.push((s, i));
                start = None;
            }
        } else if start.is_none() {
            start = Some(i);
        }
    }

    // add last problem if exists
    if let Some(s) = start {
        problems.push((s, max_width));
    }

    let mut total = 0i64;

    // solve each problem
    for (start_col, end_col) in problems {
        let mut nums = Vec::new();
        let mut op = '+';

        for (row_idx, line) in lines.iter().enumerate() {
            let segment: String = line.chars()
                .skip(start_col)
                .take(end_col - start_col)
                .collect();
            let trimmed = segment.trim();

            if row_idx < lines.len() - 1 {
                // all rows except last are numbers
                if let Ok(num) = trimmed.parse::<i64>() {
                    nums.push(num);
                }
            } else {
                // last row is operator
                if trimmed == "*" {
                    op = '*';
                }
            }
        }

        let result: i64 = if op == '*' {
            nums.iter().copied().product()
        } else {
            nums.iter().copied().sum()
        };

        total += result;
    }

    total
}

fn solve_part2(input: &str) -> i64 {
    let lines: Vec<&str> = input.lines().collect();

    // find max width
    let max_width = lines.iter().map(|line| line.len()).max().unwrap();

    // identify columns that are all spaces (separators)
    let mut is_separator = vec![true; max_width];
    for line in &lines {
        for (i, ch) in line.chars().enumerate() {
            if ch != ' ' {
                is_separator[i] = false;
            }
        }
    }

    // find problem boundaries
    let mut problems = Vec::new();
    let mut start = None;

    for i in 0..max_width {
        if is_separator[i] {
            if let Some(s) = start {
                problems.push((s, i));
                start = None;
            }
        } else if start.is_none() {
            start = Some(i);
        }
    }

    // add last problem if exists
    if let Some(s) = start {
        problems.push((s, max_width));
    }

    let mut total = 0i64;

    // solve each problem
    for (start_col, end_col) in problems {
        let num_rows = lines.len() - 1;
        let mut nums_by_col = Vec::new();
        let mut op = '+';

        // For each column in this problem, build a number by reading top to bottom
        for col in start_col..end_col {
            let mut num_str = String::new();
            for row_idx in 0..num_rows {
                let ch = lines[row_idx].chars().nth(col).unwrap_or(' ');
                if ch != ' ' {
                    num_str.push(ch);
                }
            }
            if !num_str.is_empty() {
                nums_by_col.push(num_str);
            }
        }

        // Check operator row
        for col in start_col..end_col {
            let ch = lines[num_rows].chars().nth(col).unwrap_or(' ');
            if ch == '*' {
                op = '*';
            }
        }

        let nums: Vec<i64> = nums_by_col
            .iter()
            .filter_map(|s| s.parse().ok())
            .collect();

        let result: i64 = if op == '*' {
            nums.iter().copied().product()
        } else {
            nums.iter().copied().sum()
        };

        total += result;
    }

    total
}

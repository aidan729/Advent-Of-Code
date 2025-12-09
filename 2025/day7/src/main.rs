use std::collections::{HashMap, VecDeque};

fn main() {
    let input = std::fs::read_to_string("src/input.txt").unwrap();
    println!("Part 1: {}", solve_part1(&input));
    println!("Part 2: {}", solve_part2(&input));
}

fn solve_part1(input: &str) -> usize {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    // find the starting position (S)
    let mut start_col = 0;
    for (col, &ch) in grid[0].iter().enumerate() {
        if ch == 'S' {
            start_col = col;
            break;
        }
    }

    let mut split_count = 0;
    let mut queue = VecDeque::new();
    let mut visited = std::collections::HashSet::new();

    // start with a beam at S (row 0)
    queue.push_back((0, start_col));

    while let Some((row, col)) = queue.pop_front() {
        // check if weve already processed this position
        if visited.contains(&(row, col)) {
            continue;
        }
        visited.insert((row, col));

        // Move down one row
        if row + 1 >= grid.len() {
            continue; // beam exits the manifold
        }

        let next_row = row + 1;
        let cell = grid[next_row][col];

        if cell == '^' {
            // hit a splitter count it and spawn two new beams
            split_count += 1;

            // spawn left beam (if not at left edge)
            if col > 0 {
                queue.push_back((next_row, col - 1));
            }

            // spawn right beam (if not at right edge)
            if col < grid[0].len() - 1 {
                queue.push_back((next_row, col + 1));
            }
        } else {
            // empty space continue down
            queue.push_back((next_row, col));
        }
    }

    split_count
}

fn solve_part2(input: &str) -> usize {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    // find the starting position (S)
    let mut start_col = 0;
    for (col, &ch) in grid[0].iter().enumerate() {
        if ch == 'S' {
            start_col = col;
            break;
        }
    }

    // count the number of paths to reach each position
    // paths[(row, col)] = number of distinct timelines that reach this position
    let mut paths: HashMap<(usize, usize), usize> = HashMap::new();
    paths.insert((0, start_col), 1);

    let mut queue = VecDeque::new();
    queue.push_back((0, start_col));

    while let Some((row, col)) = queue.pop_front() {
        let current_paths = *paths.get(&(row, col)).unwrap_or(&0);

        if current_paths == 0 {
            continue;
        }

        // move down one row
        if row + 1 >= grid.len() {
            continue; // exit the manifold
        }

        let next_row = row + 1;
        let cell = grid[next_row][col];

        if cell == '^' {
            // hit a splitter paths split into left and right

            // left path
            if col > 0 {
                let left_pos = (next_row, col - 1);
                let prev_count = *paths.get(&left_pos).unwrap_or(&0);
                if prev_count == 0 {
                    queue.push_back(left_pos);
                }
                paths.insert(left_pos, prev_count + current_paths);
            }

            // right path
            if col < grid[0].len() - 1 {
                let right_pos = (next_row, col + 1);
                let prev_count = *paths.get(&right_pos).unwrap_or(&0);
                if prev_count == 0 {
                    queue.push_back(right_pos);
                }
                paths.insert(right_pos, prev_count + current_paths);
            }
        } else {
            // empty space continue down
            let next_pos = (next_row, col);
            let prev_count = *paths.get(&next_pos).unwrap_or(&0);
            if prev_count == 0 {
                queue.push_back(next_pos);
            }
            paths.insert(next_pos, prev_count + current_paths);
        }
    }

    // sum all paths that reach the bottom row or exit the grid
    let bottom_row = grid.len() - 1;
    paths.iter()
        .filter(|((row, _), _)| *row == bottom_row)
        .map(|(_, count)| count)
        .sum()
}
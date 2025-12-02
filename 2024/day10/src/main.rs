use std::collections::{HashSet, VecDeque};
use std::fs;

type Pos = (usize, usize);
type Grid = Vec<Vec<u32>>;

fn main() {
    let input = fs::read_to_string("src/input.txt").expect("Failed to read input");
    let part1 = solve_part1(&input);
    let part2 = solve_part2(&input);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn parse_grid(input: &str) -> Grid {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap_or(u32::MAX))
                .collect()
        })
        .collect()
}

fn find_trailheads(grid: &Grid) -> Vec<Pos> {
    grid.iter()
        .enumerate()
        .flat_map(|(r, row)| {
            row.iter()
                .enumerate()
                .filter_map(move |(c, &height)| {
                    if height == 0 {
                        Some((r, c))
                    } else {
                        None
                    }
                })
        })
        .collect()
}

fn neighbors(pos: Pos, grid: &Grid) -> impl Iterator<Item = Pos> + '_ {
    let (r, c) = pos;
    let rows = grid.len();
    let cols = grid[0].len();

    [(0, 1), (1, 0), (0, -1), (-1, 0)]
        .into_iter()
        .filter_map(move |(dr, dc)| {
            let nr = r.checked_add_signed(dr)?;
            let nc = c.checked_add_signed(dc)?;
            if nr < rows && nc < cols {
                Some((nr, nc))
            } else {
                None
            }
        })
}

fn solve_part1(input: &str) -> usize {
    let grid = parse_grid(input);
    let trailheads = find_trailheads(&grid);

    trailheads
        .iter()
        .map(|&start| score_trailhead(start, &grid))
        .sum()
}

fn solve_part2(input: &str) -> usize {
    let grid = parse_grid(input);
    let trailheads = find_trailheads(&grid);

    trailheads
        .iter()
        .map(|&start| rate_trailhead(start, &grid))
        .sum()
}

fn score_trailhead(start: Pos, grid: &Grid) -> usize {
    // BFS to find all reachable 9s
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    let mut reachable_nines = HashSet::new();

    queue.push_back(start);
    visited.insert(start);

    while let Some(pos) = queue.pop_front() {
        let current_height = grid[pos.0][pos.1];

        if current_height == 9 {
            reachable_nines.insert(pos);
            continue;
        }

        for next_pos in neighbors(pos, grid) {
            let next_height = grid[next_pos.0][next_pos.1];

            // Must increase by exactly 1
            if next_height == current_height + 1 && !visited.contains(&next_pos) {
                visited.insert(next_pos);
                queue.push_back(next_pos);
            }
        }
    }

    reachable_nines.len()
}

fn rate_trailhead(start: Pos, grid: &Grid) -> usize {
    // DFS to count all distinct paths to 9s
    count_paths(start, grid)
}

fn count_paths(pos: Pos, grid: &Grid) -> usize {
    let current_height = grid[pos.0][pos.1];

    // Base case: reached a 9
    if current_height == 9 {
        return 1;
    }

    // Recursive case: sum paths through valid neighbors
    neighbors(pos, grid)
        .filter(|&next_pos| {
            let next_height = grid[next_pos.0][next_pos.1];
            next_height == current_height + 1
        })
        .map(|next_pos| count_paths(next_pos, grid))
        .sum()
}
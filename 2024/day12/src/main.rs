use std::collections::{HashSet, VecDeque};
use std::fs;

fn main() {
    let input = fs::read_to_string("src/input.txt").expect("Failed to read input");
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let part1 = solve_part1(&grid);
    let part2 = solve_part2(&grid);

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn solve_part1(grid: &[Vec<char>]) -> usize {
    let mut visited = HashSet::new();
    let mut total_price = 0;

    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if !visited.contains(&(row, col)) {
                let (area, perimeter) = flood_fill_area_perimeter(grid, row, col, &mut visited);
                total_price += area * perimeter;
            }
        }
    }

    total_price
}

fn solve_part2(grid: &[Vec<char>]) -> usize {
    let mut visited = HashSet::new();
    let mut total_price = 0;

    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if !visited.contains(&(row, col)) {
                let (area, sides) = flood_fill_area_sides(grid, row, col, &mut visited);
                total_price += area * sides;
            }
        }
    }

    total_price
}

fn flood_fill_area_perimeter(
    grid: &[Vec<char>],
    start_row: usize,
    start_col: usize,
    visited: &mut HashSet<(usize, usize)>,
) -> (usize, usize) {
    let plant_type = grid[start_row][start_col];
    let mut queue = VecDeque::new();
    queue.push_back((start_row, start_col));
    visited.insert((start_row, start_col));

    let mut area = 0;
    let mut perimeter = 0;

    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    while let Some((row, col)) = queue.pop_front() {
        area += 1;

        for (dr, dc) in directions.iter() {
            let new_row = row as i32 + dr;
            let new_col = col as i32 + dc;

            if new_row < 0 || new_row >= grid.len() as i32
                || new_col < 0 || new_col >= grid[0].len() as i32 {
                perimeter += 1;
                continue;
            }

            let new_row = new_row as usize;
            let new_col = new_col as usize;

            if grid[new_row][new_col] != plant_type {
                perimeter += 1;
            } else if !visited.contains(&(new_row, new_col)) {
                visited.insert((new_row, new_col));
                queue.push_back((new_row, new_col));
            }
        }
    }

    (area, perimeter)
}

fn flood_fill_area_sides(
    grid: &[Vec<char>],
    start_row: usize,
    start_col: usize,
    visited: &mut HashSet<(usize, usize)>,
) -> (usize, usize) {
    let plant_type = grid[start_row][start_col];
    let mut queue = VecDeque::new();
    queue.push_back((start_row, start_col));
    visited.insert((start_row, start_col));

    let mut region = HashSet::new();
    region.insert((start_row, start_col));

    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

    while let Some((row, col)) = queue.pop_front() {
        for (dr, dc) in directions.iter() {
            let new_row = row as i32 + dr;
            let new_col = col as i32 + dc;

            if new_row < 0 || new_row >= grid.len() as i32
                || new_col < 0 || new_col >= grid[0].len() as i32 {
                continue;
            }

            let new_row = new_row as usize;
            let new_col = new_col as usize;

            if grid[new_row][new_col] == plant_type && !visited.contains(&(new_row, new_col)) {
                visited.insert((new_row, new_col));
                region.insert((new_row, new_col));
                queue.push_back((new_row, new_col));
            }
        }
    }

    let area = region.len();
    let sides = count_corners(&region);

    (area, sides)
}

fn count_corners(region: &HashSet<(usize, usize)>) -> usize {
    let mut corners = 0;

    for &(row, col) in region {
        // Check all 4 possible corners for this cell
        // Each corner is determined by checking the adjacent cells in that direction

        // Top left corner
        let top = region.contains(&(row.wrapping_sub(1), col));
        let left = region.contains(&(row, col.wrapping_sub(1)));
        let top_left = region.contains(&(row.wrapping_sub(1), col.wrapping_sub(1)));

        // Outer corner: neither top nor left are in region
        if !top && !left {
            corners += 1;
        }
        // Inner corner: both top and left are in region, but diagonal is not
        if top && left && !top_left {
            corners += 1;
        }

        // Top right corner
        let right = region.contains(&(row, col + 1));
        let top_right = region.contains(&(row.wrapping_sub(1), col + 1));

        if !top && !right {
            corners += 1;
        }
        if top && right && !top_right {
            corners += 1;
        }

        // Bottom left corner
        let bottom = region.contains(&(row + 1, col));
        let bottom_left = region.contains(&(row + 1, col.wrapping_sub(1)));

        if !bottom && !left {
            corners += 1;
        }
        if bottom && left && !bottom_left {
            corners += 1;
        }

        // Bottom right corner
        let bottom_right = region.contains(&(row + 1, col + 1));

        if !bottom && !right {
            corners += 1;
        }
        if bottom && right && !bottom_right {
            corners += 1;
        }
    }

    corners
}

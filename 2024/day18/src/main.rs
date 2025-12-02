use std::collections::VecDeque;
use std::fs;

const GRID_SIZE: usize = 71;
const INITIAL_BYTES: usize = 1024;

type Pos = (usize, usize);

fn main() {
    let input = fs::read_to_string("src/input.txt").expect("Failed to read input");
    let part1 = solve_part1(&input);
    let part2 = solve_part2(&input);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn solve_part1(input: &str) -> usize {
    let coords = parse_coords(input);
    let mut grid = [[false; GRID_SIZE]; GRID_SIZE];

    // mark first INITIAL_BYTES as corrupted
    for &(x, y) in coords.iter().take(INITIAL_BYTES) {
        grid[y][x] = true;
    }

    bfs(&grid, (0, 0), (GRID_SIZE - 1, GRID_SIZE - 1)).unwrap_or(0)
}

fn solve_part2(input: &str) -> String {
    let coords = parse_coords(input);

    // binary search for first blocking byte
    let mut left = 0;
    let mut right = coords.len();

    while left < right {
        let mid = (left + right) / 2;

        // reset grid and add bytes up to mid
        let mut grid = [[false; GRID_SIZE]; GRID_SIZE];
        for &(x, y) in coords.iter().take(mid + 1) {
            grid[y][x] = true;
        }

        if bfs(&grid, (0, 0), (GRID_SIZE - 1, GRID_SIZE - 1)).is_none() {
            // path blocked, search left half
            right = mid;
        } else {
            // path exists, search right half
            left = mid + 1;
        }
    }

    let (x, y) = coords[left];
    format!("{},{}", x, y)
}

fn bfs(grid: &[[bool; GRID_SIZE]; GRID_SIZE], start: Pos, end: Pos) -> Option<usize> {
    let mut queue = VecDeque::new();
    let mut visited = [[false; GRID_SIZE]; GRID_SIZE];

    queue.push_back((start, 0));
    visited[start.1][start.0] = true;

    while let Some(((x, y), steps)) = queue.pop_front() {
        if (x, y) == end {
            return Some(steps);
        }

        // check 4 directions
        for (dx, dy) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let nx = x.wrapping_add_signed(dx);
            let ny = y.wrapping_add_signed(dy);

            if nx < GRID_SIZE
                && ny < GRID_SIZE
                && !grid[ny][nx]
                && !visited[ny][nx]
            {
                visited[ny][nx] = true;
                queue.push_back(((nx, ny), steps + 1));
            }
        }
    }

    None
}

fn parse_coords(input: &str) -> Vec<Pos> {
    input
        .lines()
        .filter_map(|line| {
            let mut parts = line.split(',');
            let x = parts.next()?.parse().ok()?;
            let y = parts.next()?.parse().ok()?;
            Some((x, y))
        })
        .collect()
}

use std::collections::{HashMap, VecDeque};
use std::fs;

type Pos = (i32, i32);

fn main() {
    let input = fs::read_to_string("src/input.txt").expect("Failed to read input");
    let part1 = solve_part1(&input);
    let part2 = solve_part2(&input);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn solve_part1(input: &str) -> usize {
    let (grid, start, _end) = parse_maze(input);
    let distances = bfs_distances(&grid, start);
    count_cheats(&distances, 2, 100)
}

fn solve_part2(input: &str) -> usize {
    let (grid, start, _end) = parse_maze(input);
    let distances = bfs_distances(&grid, start);
    count_cheats(&distances, 20, 100)
}

fn count_cheats(distances: &HashMap<Pos, usize>, max_cheat_dist: i32, min_saving: usize) -> usize {
    let mut count = 0;

    for (&pos, &dist_start) in distances {
        // enumerate all positions within manhattan distance of max_cheat_dist
        for dy in -max_cheat_dist..=max_cheat_dist {
            let remaining = max_cheat_dist - dy.abs();
            for dx in -remaining..=remaining {
                let cheat_end = (pos.0 + dx, pos.1 + dy);

                if let Some(&dist_end) = distances.get(&cheat_end) {
                    let cheat_cost = dx.abs() + dy.abs();

                    // can only cheat forward in time
                    if dist_end > dist_start {
                        let time_saved = dist_end - dist_start - cheat_cost as usize;
                        if time_saved >= min_saving {
                            count += 1;
                        }
                    }
                }
            }
        }
    }

    count
}

fn bfs_distances(grid: &[Vec<char>], start: Pos) -> HashMap<Pos, usize> {
    let mut distances = HashMap::new();
    let mut queue = VecDeque::new();

    queue.push_back((start, 0));
    distances.insert(start, 0);

    while let Some((pos, dist)) = queue.pop_front() {
        // try all 4 directions
        for (dx, dy) in [(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let next = (pos.0 + dx, pos.1 + dy);

            // check bounds and if its track
            if next.1 >= 0 && next.1 < grid.len() as i32
                && next.0 >= 0 && next.0 < grid[0].len() as i32 {
                let cell = grid[next.1 as usize][next.0 as usize];

                if (cell == '.' || cell == 'E') && !distances.contains_key(&next) {
                    distances.insert(next, dist + 1);
                    queue.push_back((next, dist + 1));
                }
            }
        }
    }

    distances
}

fn parse_maze(input: &str) -> (Vec<Vec<char>>, Pos, Pos) {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut start = (0, 0);
    let mut end = (0, 0);

    for (y, row) in grid.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            if cell == 'S' {
                start = (x as i32, y as i32);
            } else if cell == 'E' {
                end = (x as i32, y as i32);
            }
        }
    }

    (grid, start, end)
}

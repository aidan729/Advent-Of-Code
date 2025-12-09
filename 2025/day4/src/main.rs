use std::fs;

fn main() {
    let input = fs::read_to_string("src/input.txt").expect("Failed to read input");
    let part1 = solve_part1(&input);
    let part2 = solve_part2(&input);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn solve_part1(input: &str) -> usize {
    let grid: Vec<&[u8]> = input.lines().map(|line| line.as_bytes()).collect();

    let mut accessible = 0;

    for r in 0..grid.len() {
        for c in 0..grid[r].len() {
            if grid[r][c] == b'@' && count_neighbors(&grid, r, c) < 4 {
                accessible += 1;
            }
        }
    }

    accessible
}

fn solve_part2(input: &str) -> usize {
    // need mutable grid for part 2
    let mut grid: Vec<Vec<u8>> = input.lines().map(|line| line.bytes().collect()).collect();

    let mut total_removed = 0;

    // repeatedly remove accessible rolls until none left
    loop {
        // find all accessible rolls this round
        let mut to_remove = Vec::new();

        for r in 0..grid.len() {
            for c in 0..grid[r].len() {
                if grid[r][c] == b'@' && count_neighbors_mut(&grid, r, c) < 4 {
                    to_remove.push((r, c));
                }
            }
        }

        if to_remove.is_empty() {
            break;
        }

        // remove them all
        for (r, c) in to_remove.iter() {
            grid[*r][*c] = b'.';
        }

        total_removed += to_remove.len();
    }

    total_removed
}

fn count_neighbors(grid: &[&[u8]], r: usize, c: usize) -> usize {
    const DIRS: [(isize, isize); 8] = [
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1),           (0, 1),
        (1, -1),  (1, 0),  (1, 1),
    ];

    DIRS.iter()
        .filter(|&&(dr, dc)| {
            let nr = r.wrapping_add_signed(dr);
            let nc = c.wrapping_add_signed(dc);
            grid.get(nr)
                .and_then(|row| row.get(nc))
                .map_or(false, |&cell| cell == b'@')
        })
        .count()
}

fn count_neighbors_mut(grid: &[Vec<u8>], r: usize, c: usize) -> usize {
    const DIRS: [(isize, isize); 8] = [
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1),           (0, 1),
        (1, -1),  (1, 0),  (1, 1),
    ];

    DIRS.iter()
        .filter(|&&(dr, dc)| {
            let nr = r.wrapping_add_signed(dr);
            let nc = c.wrapping_add_signed(dc);
            grid.get(nr)
                .and_then(|row| row.get(nc))
                .map_or(false, |&cell| cell == b'@')
        })
        .count()
}

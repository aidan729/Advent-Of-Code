use std::collections::{BinaryHeap, HashMap, HashSet};
use std::fs;

type Pos = (usize, usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Dir {
    North,
    East,
    South,
    West,
}

impl Dir {
    fn turn_left(self) -> Self {
        match self {
            Dir::North => Dir::West,
            Dir::East => Dir::North,
            Dir::South => Dir::East,
            Dir::West => Dir::South,
        }
    }

    fn turn_right(self) -> Self {
        match self {
            Dir::North => Dir::East,
            Dir::East => Dir::South,
            Dir::South => Dir::West,
            Dir::West => Dir::North,
        }
    }

    fn forward(self, pos: Pos) -> Option<Pos> {
        let (r, c) = pos;
        match self {
            Dir::North => r.checked_sub(1).map(|nr| (nr, c)),
            Dir::South => Some((r + 1, c)),
            Dir::East => Some((r, c + 1)),
            Dir::West => c.checked_sub(1).map(|nc| (r, nc)),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct State {
    cost: usize,
    pos: Pos,
    dir: Dir,
}

// implement Ord for BinaryHeap (min heap by cost)
impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let input = fs::read_to_string("src/input.txt").expect("Failed to read input");
    let part1 = solve_part1(&input);
    let part2 = solve_part2(&input);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn solve_part1(input: &str) -> usize {
    let (grid, start, end) = parse_maze(input);
    dijkstra(&grid, start, end)
}

fn solve_part2(input: &str) -> usize {
    let (grid, start, end) = parse_maze(input);
    count_best_path_tiles(&grid, start, end)
}

fn parse_maze(input: &str) -> (Vec<Vec<char>>, Pos, Pos) {
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut start = (0, 0);
    let mut end = (0, 0);

    for (r, row) in grid.iter().enumerate() {
        for (c, &cell) in row.iter().enumerate() {
            if cell == 'S' {
                start = (r, c);
            } else if cell == 'E' {
                end = (r, c);
            }
        }
    }

    (grid, start, end)
}

fn dijkstra(grid: &[Vec<char>], start: Pos, end: Pos) -> usize {
    // state includes direction to avoid revisiting with different costs
    let mut heap = BinaryHeap::new();
    let mut dist: HashMap<(Pos, Dir), usize> = HashMap::new();

    // start facing east
    heap.push(State {
        cost: 0,
        pos: start,
        dir: Dir::East,
    });
    dist.insert((start, Dir::East), 0);

    while let Some(State { cost, pos, dir }) = heap.pop() {
        // reached end
        if pos == end {
            return cost;
        }

        // skip if weve found a better path
        if let Some(&best) = dist.get(&(pos, dir)) {
            if cost > best {
                continue;
            }
        }

        // three possible moves: forward, turn left, turn right

        // move forward (cost +1)
        if let Some(next_pos) = dir.forward(pos) {
            if next_pos.0 < grid.len()
                && next_pos.1 < grid[0].len()
                && grid[next_pos.0][next_pos.1] != '#'
            {
                let next_cost = cost + 1;
                let key = (next_pos, dir);

                if next_cost < *dist.get(&key).unwrap_or(&usize::MAX) {
                    dist.insert(key, next_cost);
                    heap.push(State {
                        cost: next_cost,
                        pos: next_pos,
                        dir,
                    });
                }
            }
        }

        // turn left (cost +1000)
        let left_dir = dir.turn_left();
        let left_cost = cost + 1000;
        let left_key = (pos, left_dir);

        if left_cost < *dist.get(&left_key).unwrap_or(&usize::MAX) {
            dist.insert(left_key, left_cost);
            heap.push(State {
                cost: left_cost,
                pos,
                dir: left_dir,
            });
        }

        // turn right (cost +1000)
        let right_dir = dir.turn_right();
        let right_cost = cost + 1000;
        let right_key = (pos, right_dir);

        if right_cost < *dist.get(&right_key).unwrap_or(&usize::MAX) {
            dist.insert(right_key, right_cost);
            heap.push(State {
                cost: right_cost,
                pos,
                dir: right_dir,
            });
        }
    }

    usize::MAX // no path found
}

fn count_best_path_tiles(grid: &[Vec<char>], start: Pos, end: Pos) -> usize {
    // modified dijkstra that tracks predecessors for all optimal paths
    let mut heap = BinaryHeap::new();
    let mut dist: HashMap<(Pos, Dir), usize> = HashMap::new();
    let mut predecessors: HashMap<(Pos, Dir), Vec<(Pos, Dir)>> = HashMap::new();

    heap.push(State {
        cost: 0,
        pos: start,
        dir: Dir::East,
    });
    dist.insert((start, Dir::East), 0);

    let mut min_cost_to_end = usize::MAX;

    while let Some(State { cost, pos, dir }) = heap.pop() {
        // early termination optimization (no more optimal paths)
        if cost > min_cost_to_end {
            break;
        }

        // reached end (record best cost)
        if pos == end {
            min_cost_to_end = min_cost_to_end.min(cost);
            continue; // keep going to find all paths with same cost
        }

        // skip if found better path
        if let Some(&best) = dist.get(&(pos, dir)) {
            if cost > best {
                continue;
            }
        }

        // explore neighbors
        let moves = [
            // forward
            (dir.forward(pos), dir, 1),
            // turn left
            (Some(pos), dir.turn_left(), 1000),
            // turn right
            (Some(pos), dir.turn_right(), 1000),
        ];

        for (next_pos_opt, next_dir, move_cost) in moves {
            if let Some(next_pos) = next_pos_opt {
                if next_pos.0 >= grid.len()
                    || next_pos.1 >= grid[0].len()
                    || grid[next_pos.0][next_pos.1] == '#'
                {
                    continue;
                }

                let next_cost = cost + move_cost;
                let key = (next_pos, next_dir);
                let current_best = *dist.get(&key).unwrap_or(&usize::MAX);

                if next_cost < current_best {
                    // found better path
                    dist.insert(key, next_cost);
                    predecessors.insert(key, vec![(pos, dir)]);
                    heap.push(State {
                        cost: next_cost,
                        pos: next_pos,
                        dir: next_dir,
                    });
                } else if next_cost == current_best {
                    // found equal path (add predecessor)
                    predecessors.entry(key).or_default().push((pos, dir));
                }
            }
        }
    }

    // backtrack from all end states with optimal cost
    let mut visited_tiles = HashSet::new();
    let mut stack = Vec::new();

    // find all end directions with optimal cost
    for dir in [Dir::North, Dir::East, Dir::South, Dir::West] {
        if let Some(&cost) = dist.get(&(end, dir)) {
            if cost == min_cost_to_end {
                stack.push((end, dir));
            }
        }
    }

    let mut visited_states = HashSet::new();

    while let Some(state) = stack.pop() {
        if !visited_states.insert(state) {
            continue;
        }

        visited_tiles.insert(state.0);

        if let Some(preds) = predecessors.get(&state) {
            for &pred in preds {
                stack.push(pred);
            }
        }
    }

    visited_tiles.len()
}

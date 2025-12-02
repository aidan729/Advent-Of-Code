use std::fs;

type Pos = (usize, usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Empty,
    Wall,
    Box,
    Robot,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum WideCell {
    Empty,
    Wall,
    BoxLeft,   // [
    BoxRight,  // ]
    Robot,
}

#[derive(Clone)]
struct Warehouse {
    grid: Vec<Vec<Cell>>,
    robot: Pos,
}

struct WideWarehouse {
    grid: Vec<Vec<WideCell>>,
    robot: Pos,
}

fn main() {
    let input = fs::read_to_string("src/input.txt").expect("Failed to read input");
    let (warehouse, moves) = parse_input(&input);

    let part1 = solve_part1(warehouse.clone(), &moves);
    let part2 = solve_part2(warehouse, &moves);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn parse_input(input: &str) -> (Warehouse, Vec<char>) {
    // handle both unix and windows line endings
    let normalized = input.replace("\r\n", "\n");
    let parts: Vec<&str> = normalized.split("\n\n").collect();
    let map_str = parts[0];
    let moves_str = if parts.len() > 1 { parts[1] } else { "" };

    let mut grid = Vec::new();
    let mut robot = (0, 0);

    for (r, line) in map_str.lines().enumerate() {
        let mut row = Vec::new();
        for (c, ch) in line.chars().enumerate() {
            let cell = match ch {
                '#' => Cell::Wall,
                '.' => Cell::Empty,
                'O' => Cell::Box,
                '@' => {
                    robot = (r, c);
                    Cell::Robot
                }
                _ => panic!("Unknown cell: {}", ch),
            };
            row.push(cell);
        }
        grid.push(row);
    }

    let moves = moves_str.chars().filter(|&c| c != '\n').collect();

    (Warehouse { grid, robot }, moves)
}

fn solve_part1(mut warehouse: Warehouse, moves: &[char]) -> usize {
    for &dir in moves {
        try_move(&mut warehouse, dir);
    }

    calculate_gps_sum(&warehouse)
}

fn try_move(warehouse: &mut Warehouse, dir: char) {
    let (dr, dc) = match dir {
        '^' => (-1, 0),
        'v' => (1, 0),
        '<' => (0, -1),
        '>' => (0, 1),
        _ => return,
    };

    let (r, c) = warehouse.robot;
    let nr = (r as isize + dr) as usize;
    let nc = (c as isize + dc) as usize;

    // check whats ahead without allocating
    match warehouse.grid[nr][nc] {
        Cell::Wall => return, // cant move into wall
        Cell::Empty => {
            // simple case just move robot
            warehouse.grid[r][c] = Cell::Empty;
            warehouse.grid[nr][nc] = Cell::Robot;
            warehouse.robot = (nr, nc);
        }
        Cell::Box => {
            // chain push find the end of box line
            let mut boxes_end = (nr, nc);
            loop {
                let next_r = (boxes_end.0 as isize + dr) as usize;
                let next_c = (boxes_end.1 as isize + dc) as usize;

                match warehouse.grid[next_r][next_c] {
                    Cell::Wall => return, // chain blocked by wall
                    Cell::Empty => {
                        // found space push entire chain
                        // only update endpoints instead of shifting all boxes
                        warehouse.grid[next_r][next_c] = Cell::Box;
                        warehouse.grid[nr][nc] = Cell::Robot;
                        warehouse.grid[r][c] = Cell::Empty;
                        warehouse.robot = (nr, nc);
                        return;
                    }
                    Cell::Box => {
                        // continue searching
                        boxes_end = (next_r, next_c);
                    }
                    Cell::Robot => unreachable!(),
                }
            }
        }
        Cell::Robot => unreachable!(),
    }
}

fn calculate_gps_sum(warehouse: &Warehouse) -> usize {
    warehouse
        .grid
        .iter()
        .enumerate()
        .flat_map(|(r, row)| {
            row.iter().enumerate().filter_map(move |(c, &cell)| {
                if cell == Cell::Box {
                    Some(100 * r + c)
                } else {
                    None
                }
            })
        })
        .sum()
}

fn solve_part2(warehouse: Warehouse, moves: &[char]) -> usize {
    let mut wide = scale_warehouse(&warehouse);

    for &dir in moves {
        try_move_wide(&mut wide, dir);
    }

    calculate_gps_sum_wide(&wide)
}

fn scale_warehouse(warehouse: &Warehouse) -> WideWarehouse {
    // directly build wide grid without intermediate allocations
    let mut wide_grid = Vec::new();
    let mut robot = (0, 0);

    for (r, row) in warehouse.grid.iter().enumerate() {
        let mut wide_row = Vec::new();
        for (c, &cell) in row.iter().enumerate() {
            let (left, right) = match cell {
                Cell::Wall => (WideCell::Wall, WideCell::Wall),
                Cell::Box => (WideCell::BoxLeft, WideCell::BoxRight),
                Cell::Empty => (WideCell::Empty, WideCell::Empty),
                Cell::Robot => {
                    robot = (r, c * 2);
                    (WideCell::Robot, WideCell::Empty)
                }
            };
            wide_row.push(left);
            wide_row.push(right);
        }
        wide_grid.push(wide_row);
    }

    WideWarehouse {
        grid: wide_grid,
        robot,
    }
}

fn try_move_wide(warehouse: &mut WideWarehouse, dir: char) {
    let (dr, dc) = match dir {
        '^' => (-1, 0),
        'v' => (1, 0),
        '<' => (0, -1),
        '>' => (0, 1),
        _ => return,
    };

    // horizontal moves are simpler same logic as part 1
    if dc != 0 {
        try_move_wide_horizontal(warehouse, dir, dr, dc);
    } else {
        // vertical moves can push multiple boxes at once
        try_move_wide_vertical(warehouse, dir, dr);
    }
}

fn try_move_wide_horizontal(warehouse: &mut WideWarehouse, _dir: char, dr: isize, dc: isize) {
    let (r, c) = warehouse.robot;
    let nr = (r as isize + dr) as usize;
    let nc = (c as isize + dc) as usize;

    match warehouse.grid[nr][nc] {
        WideCell::Wall => return,
        WideCell::Empty => {
            warehouse.grid[r][c] = WideCell::Empty;
            warehouse.grid[nr][nc] = WideCell::Robot;
            warehouse.robot = (nr, nc);
        }
        WideCell::BoxLeft | WideCell::BoxRight => {
            // find end of box chain
            let mut end_c = nc;
            loop {
                let next_c = (end_c as isize + dc) as usize;
                match warehouse.grid[nr][next_c] {
                    WideCell::Wall => return,
                    WideCell::Empty => {
                        // shift entire chain by swapping
                        if dc > 0 {
                            // moving right
                            for shift_c in ((nc + 1)..=next_c).rev() {
                                warehouse.grid[nr][shift_c] = warehouse.grid[nr][shift_c - 1];
                            }
                        } else {
                            // moving left
                            for shift_c in next_c..nc {
                                warehouse.grid[nr][shift_c] = warehouse.grid[nr][shift_c + 1];
                            }
                        }
                        warehouse.grid[r][c] = WideCell::Empty;
                        warehouse.grid[nr][nc] = WideCell::Robot;
                        warehouse.robot = (nr, nc);
                        return;
                    }
                    WideCell::BoxLeft | WideCell::BoxRight => {
                        end_c = next_c;
                    }
                    WideCell::Robot => unreachable!(),
                }
            }
        }
        WideCell::Robot => unreachable!(),
    }
}

fn try_move_wide_vertical(warehouse: &mut WideWarehouse, _dir: char, dr: isize) {
    let (r, c) = warehouse.robot;
    let nr = (r as isize + dr) as usize;

    match warehouse.grid[nr][c] {
        WideCell::Wall => return,
        WideCell::Empty => {
            warehouse.grid[r][c] = WideCell::Empty;
            warehouse.grid[nr][c] = WideCell::Robot;
            warehouse.robot = (nr, c);
        }
        WideCell::BoxLeft | WideCell::BoxRight => {
            // use BFS to find all connected boxes that need to move
            // cascade detection
            if can_push_vertical(warehouse, nr, c, dr) {
                do_push_vertical(warehouse, nr, c, dr);
                warehouse.grid[r][c] = WideCell::Empty;
                warehouse.grid[nr][c] = WideCell::Robot;
                warehouse.robot = (nr, c);
            }
        }
        WideCell::Robot => unreachable!(),
    }
}

fn can_push_vertical(warehouse: &WideWarehouse, row: usize, col: usize, dr: isize) -> bool {
    // can this box and all boxes it pushes move?
    let cell = warehouse.grid[row][col];

    let (left_col, right_col) = match cell {
        WideCell::BoxLeft => (col, col + 1),
        WideCell::BoxRight => (col - 1, col),
        _ => return true,
    };

    let next_row = (row as isize + dr) as usize;

    // check both halves of the box
    for check_col in [left_col, right_col] {
        match warehouse.grid[next_row][check_col] {
            WideCell::Wall => return false,
            WideCell::BoxLeft | WideCell::BoxRight => {
                if !can_push_vertical(warehouse, next_row, check_col, dr) {
                    return false;
                }
            }
            WideCell::Empty | WideCell::Robot => {}
        }
    }

    true
}

fn do_push_vertical(warehouse: &mut WideWarehouse, row: usize, col: usize, dr: isize) {
    let cell = warehouse.grid[row][col];

    let (left_col, right_col) = match cell {
        WideCell::BoxLeft => (col, col + 1),
        WideCell::BoxRight => (col - 1, col),
        _ => return,
    };

    let next_row = (row as isize + dr) as usize;

    // recursively push boxes above/below first
    for check_col in [left_col, right_col] {
        match warehouse.grid[next_row][check_col] {
            WideCell::BoxLeft | WideCell::BoxRight => {
                do_push_vertical(warehouse, next_row, check_col, dr);
            }
            _ => {}
        }
    }

    // now move this box
    warehouse.grid[next_row][left_col] = WideCell::BoxLeft;
    warehouse.grid[next_row][right_col] = WideCell::BoxRight;
    warehouse.grid[row][left_col] = WideCell::Empty;
    warehouse.grid[row][right_col] = WideCell::Empty;
}

fn calculate_gps_sum_wide(warehouse: &WideWarehouse) -> usize {
    warehouse
        .grid
        .iter()
        .enumerate()
        .flat_map(|(r, row)| {
            row.iter().enumerate().filter_map(move |(c, &cell)| {
                // only count left edge of boxes
                if cell == WideCell::BoxLeft {
                    Some(100 * r + c)
                } else {
                    None
                }
            })
        })
        .sum()
}

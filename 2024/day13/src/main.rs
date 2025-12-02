use std::fs;

#[derive(Debug)]
struct Machine {
    a_x: i64,
    a_y: i64,
    b_x: i64,
    b_y: i64,
    prize_x: i64,
    prize_y: i64,
}

fn main() {
    let input = fs::read_to_string("src/input.txt").expect("Failed to read input");
    let machines = parse_input(&input);

    let part1 = solve_part1(&machines);
    let part2 = solve_part2(&machines);

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn parse_input(input: &str) -> Vec<Machine> {
    let mut machines = Vec::new();
    let lines: Vec<&str> = input.lines().collect();

    let mut i = 0;
    while i < lines.len() {
        if lines[i].is_empty() {
            i += 1;
            continue;
        }

        // Parse Button A
        let a_line = lines[i];
        let a_parts: Vec<&str> = a_line.split(", ").collect();
        let a_x = a_parts[0].split("+").nth(1).unwrap().parse::<i64>().unwrap();
        let a_y = a_parts[1].split("+").nth(1).unwrap().parse::<i64>().unwrap();

        // Parse Button B
        let b_line = lines[i + 1];
        let b_parts: Vec<&str> = b_line.split(", ").collect();
        let b_x = b_parts[0].split("+").nth(1).unwrap().parse::<i64>().unwrap();
        let b_y = b_parts[1].split("+").nth(1).unwrap().parse::<i64>().unwrap();

        // Parse Prize
        let prize_line = lines[i + 2];
        let prize_parts: Vec<&str> = prize_line.split(", ").collect();
        let prize_x = prize_parts[0].split("=").nth(1).unwrap().parse::<i64>().unwrap();
        let prize_y = prize_parts[1].split("=").nth(1).unwrap().parse::<i64>().unwrap();

        machines.push(Machine {
            a_x,
            a_y,
            b_x,
            b_y,
            prize_x,
            prize_y,
        });

        i += 3;
    }

    machines
}

fn solve_part1(machines: &[Machine]) -> i64 {
    let mut total_tokens = 0;

    for machine in machines {
        if let Some(tokens) = find_min_tokens(machine, 100) {
            total_tokens += tokens;
        }
    }

    total_tokens
}

fn solve_part2(machines: &[Machine]) -> i64 {
    let mut total_tokens = 0;
    const OFFSET: i64 = 10_000_000_000_000;

    for machine in machines {
        let adjusted_machine = Machine {
            a_x: machine.a_x,
            a_y: machine.a_y,
            b_x: machine.b_x,
            b_y: machine.b_y,
            prize_x: machine.prize_x + OFFSET,
            prize_y: machine.prize_y + OFFSET,
        };

        if let Some(tokens) = find_min_tokens(&adjusted_machine, i64::MAX) {
            total_tokens += tokens;
        }
    }

    total_tokens
}

fn find_min_tokens(machine: &Machine, max_presses: i64) -> Option<i64> {
    // We need to solve:
    // a * a_x + b * b_x = prize_x
    // a * a_y + b * b_y = prize_y
    //
    // Using Cramer's rule:
    // determinant = a_x * b_y - a_y * b_x
    // a = (prize_x * b_y - prize_y * b_x) / determinant
    // b = (a_x * prize_y - a_y * prize_x) / determinant

    let det = machine.a_x * machine.b_y - machine.a_y * machine.b_x;

    if det == 0 {
        return None; // No unique solution
    }

    let a_num = machine.prize_x * machine.b_y - machine.prize_y * machine.b_x;
    let b_num = machine.a_x * machine.prize_y - machine.a_y * machine.prize_x;

    // Check if solutions are integers
    if a_num % det != 0 || b_num % det != 0 {
        return None;
    }

    let a = a_num / det;
    let b = b_num / det;

    // Check if solutions are non-negative and within limits
    if a < 0 || b < 0 || a > max_presses || b > max_presses {
        return None;
    }

    // Verify the solution
    if a * machine.a_x + b * machine.b_x != machine.prize_x
        || a * machine.a_y + b * machine.b_y != machine.prize_y
    {
        return None;
    }

    // Calculate tokens: 3 per A press, 1 per B press
    Some(a * 3 + b)
}

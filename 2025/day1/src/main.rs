use std::fs;

fn main() {
    let input = fs::read_to_string("src/input.txt").expect("Failed to read input");
    let part1 = solve_part1(&input);
    let part2 = solve_part2(&input);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn solve_part1(input: &str) -> usize {
    let mut position = 50; // starting position
    let mut zero_count = 0;

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        // parse direction and distance
        let direction = &line[0..1];
        let distance: i32 = line[1..].parse().expect("Failed to parse distance");

        // Rotate the dial
        position = match direction {
            "L" => {
                // left means subtract (toward lower numbers)
                (position - distance).rem_euclid(100)
            }
            "R" => {
                // right means add (toward higher numbers)
                (position + distance).rem_euclid(100)
            }
            _ => panic!("Unknown direction: {}", direction),
        };

        // check if we landed on 0
        if position == 0 {
            zero_count += 1;
        }
    }

    zero_count
}

fn solve_part2(input: &str) -> usize {
    let mut position = 50; // starting position
    let mut zero_count = 0;

    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }

        // parse direction and distance
        let direction = &line[0..1];
        let distance: i32 = line[1..].parse().expect("Failed to parse distance");

        // count how many times we pass through 0 during the rotation
        zero_count += count_zeros_in_rotation(position, direction, distance);

        // rotate the dial
        position = match direction {
            "L" => (position - distance).rem_euclid(100),
            "R" => (position + distance).rem_euclid(100),
            _ => panic!("Unknown direction: {}", direction),
        };
    }

    zero_count
}

fn count_zeros_in_rotation(start: i32, direction: &str, distance: i32) -> usize {
    // calculate the end position
    let end = match direction {
        "L" => (start - distance).rem_euclid(100),
        "R" => (start + distance).rem_euclid(100),
        _ => panic!("Unknown direction"),
    };

    // if distance is 0, no rotation happens
    if distance == 0 {
        return 0;
    }

    // if we start at 0 we dont count it, we only count positions we pass through or land on
    // so we need to count positions in the range start, end for R or end, start for L
    // this means we count 0 only if its strictly after start in the rotation direction

    let mut count = 0;

    match direction {
        "R" => {
            // moving right (increasing numbers)
            // each complete lap of 100 adds 1 zero
            count += (distance / 100) as usize;

            // check if we cross 0 in the partial lap
            // we count 0 if it's in the range start, end (> start and <= end)
            // considering wraparound
            if start < end {
                // no wrap cause 0 is only in range start, end if end == 0 and start < 0
                // but start is always 0-99 so this only works if end == 0 and start != 0
                if end == 0 {
                    count += 1;
                }
            } else if start > end {
                // we wrapped: start -> 99 -> 0 -> end
                // 0 is definitely in the range start, end when wrapping
                // UNLESS start == 0, but then start > end would be false
                count += 1;
            } else {
                // start == end means we did exact multiples of 100
                // already counted above
            }
        }
        "L" => {
            // moving left (decreasing numbers)
            count += (distance / 100) as usize;

            // check if we cross 0 in the partial lap
            // we count 0 if it's in range start, end going backwards
            // which is end, start going forwards, but excluding start
            if start > end {
                // no wrap we go from start down to end
                // 0 is in range end, start only if end == 0 (inclusive) and start > 0
                if end == 0 {
                    count += 1;
                }
            } else if start < end {
                // we wrapped: start -> 0 -> 99 -> end
                // but wait, if start == 0, we go 0 -> 99 -> ... -> end
                // we shouldn't count the starting 0
                // only count if start > 0 (then we pass through 0)
                if start > 0 {
                    count += 1;
                }
            } else {
                // start == end, exact multiples
            }
        }
        _ => panic!("Unknown direction"),
    }

    count
}
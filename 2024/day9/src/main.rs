use std::fs;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Block {
    File(usize),  // file ID
    Free,
}

fn main() {
    let input = fs::read_to_string("src/input.txt").expect("Failed to read input");
    let part1 = solve_part1(&input);
    let part2 = solve_part2(&input);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn parse_disk_map(input: &str) -> Vec<Block> {
    let digits: Vec<usize> = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();

    let mut blocks = Vec::new();
    let mut file_id = 0;

    for (i, &length) in digits.iter().enumerate() {
        let block = if i % 2 == 0 {
            // even indices are files
            let b = Block::File(file_id);
            file_id += 1;
            b
        } else {
            // ddd indices are free space
            Block::Free
        };

        blocks.extend(std::iter::repeat(block).take(length));
    }

    blocks
}

fn solve_part1(input: &str) -> usize {
    let mut disk = parse_disk_map(input);

    // compact by moving individual blocks
    loop {
        // find leftmost free space
        let Some(free_idx) = disk.iter().position(|&b| b == Block::Free) else {
            break;
        };

        // find rightmost file block
        let Some(file_idx) = disk.iter().rposition(|&b| matches!(b, Block::File(_))) else {
            break;
        };

        // ifq file is already to the left of free space were done
        if file_idx < free_idx {
            break;
        }

        // move the file block to the free space
        disk.swap(free_idx, file_idx);
    }

    checksum(&disk)
}

fn solve_part2(input: &str) -> usize {
    let mut disk = parse_disk_map(input);

    // find the highest file ID
    let max_file_id = disk
        .iter()
        .filter_map(|&b| if let Block::File(id) = b { Some(id) } else { None })
        .max()
        .unwrap_or(0);

    // process files in decreasing ID order
    for file_id in (0..=max_file_id).rev() {
        // find the files current position and size
        let file_positions: Vec<usize> = disk
            .iter()
            .enumerate()
            .filter_map(|(i, &b)| {
                if b == Block::File(file_id) {
                    Some(i)
                } else {
                    None
                }
            })
            .collect();

        if file_positions.is_empty() {
            continue;
        }

        let file_size = file_positions.len();
        let file_start = file_positions[0];

        // find leftmost contiguous free space that fits this file
        if let Some(free_start) = find_free_span(&disk, file_size, file_start) {
            // move the entire file
            for offset in 0..file_size {
                disk[free_start + offset] = Block::File(file_id);
                disk[file_start + offset] = Block::Free;
            }
        }
    }

    checksum(&disk)
}

fn find_free_span(disk: &[Block], size: usize, before: usize) -> Option<usize> {
    let mut start = 0;
    let mut count = 0;

    for (i, &block) in disk.iter().enumerate() {
        if i >= before {
            return None; // must be to the left of the file
        }

        if block == Block::Free {
            if count == 0 {
                start = i;
            }
            count += 1;
            if count >= size {
                return Some(start);
            }
        } else {
            count = 0;
        }
    }

    None
}

fn checksum(disk: &[Block]) -> usize {
    disk.iter()
        .enumerate()
        .filter_map(|(pos, &block)| {
            if let Block::File(id) = block {
                Some(pos * id)
            } else {
                None
            }
        })
        .sum()
}
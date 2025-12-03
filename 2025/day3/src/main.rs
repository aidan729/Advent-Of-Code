use std::fs;

fn main() {
    let input = fs::read_to_string("src/input.txt").expect("Failed to read input");
    let part1 = solve_part1(&input);
    let part2 = solve_part2(&input);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn solve_part1(input: &str) -> u32 {
    input.lines().map(max_joltage_2).sum()
}

fn solve_part2(input: &str) -> u64 {
    input.lines().map(|line| max_joltage_k(line, 12)).sum()
}

fn max_joltage_2(bank: &str) -> u32 {
    let bytes = bank.as_bytes();
    let n = bytes.len();

    // single pass right to left building suffix max on the fly
    // O(n)
    let mut suffix_max = bytes[n - 1];
    let mut result = 0u32;

    for i in (0..n - 1).rev() {
        let digit = (bytes[i] - b'0') as u32;
        let max_after = (suffix_max - b'0') as u32;
        result = result.max(digit * 10 + max_after);
        suffix_max = suffix_max.max(bytes[i]);
    }

    result
}

fn max_joltage_k(bank: &str, k: usize) -> u64 {
    let digits: Vec<u8> = bank.bytes().map(|b| b - b'0').collect();
    let n = digits.len();

    // greedy selection with monotonic stack optimization
    // instead of searching for max in range each time, maintain invariant:
    // result contains largest k digits seen so far in lexicographic order
    let mut stack = Vec::with_capacity(k);
    let mut to_skip = n - k; // how many digits we can afford to skip

    for &digit in &digits {
        // pop smaller digits if we can still skip them
        while !stack.is_empty() && *stack.last().unwrap() < digit && to_skip > 0 {
            stack.pop();
            to_skip -= 1;
        }

        stack.push(digit);
    }

    // take only first k digits (we may have pushed more than k)
    stack.truncate(k);

    // convert to u64 without intermediate allocations
    stack.iter().fold(0u64, |acc, &d| acc * 10 + d as u64)
}

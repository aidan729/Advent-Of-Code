use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn eval_expression1(nums: &[i64], ops: &[char]) -> i64 {
    let mut result = nums[0];
    for(i, &op) in ops.iter().enumerate() {
        match op {
            '+' => result += nums[i + 1],
            '*' => result *= nums[i + 1],
            _ => unreachable!(),
        }
    }
    result
}

fn generate_operator_combinations1(nums: &[i64], target: i64) -> bool {
    let num_ops = nums.len() - 1;
    // let ops = vec!['+', '*'];

    for bitmask in 0..(1 << num_ops) {
        let mut op_sequence = vec!['+'; num_ops];
        for i in 0..num_ops {
            if (bitmask & (1 << i)) != 0 {
                op_sequence[i] = '*';
            }
        }

        if eval_expression1(nums, &op_sequence) == target {
            return true;
        }
    }
    false
}

fn process_file1(filename: &str) -> i64 {
    let path = Path::new(filename);
    let file = File::open(path).expect("Unable to open file");
    let reader = io::BufReader::new(file);
    let mut total = 0;

    for line in reader.lines() {
        if let Ok(line) = line {
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() != 2 {
                continue;
            }

            let target: i64 = parts[0].trim().parse().expect("Invalid target value");
            let nums: Vec<i64> = parts[1]
                .trim()
                .split_whitespace()
                .map(|x| x.parse().expect("Invalid number"))
                .collect();
            
            if generate_operator_combinations1(&nums, target) {
                total += target;
            }
        }
    }
    total
}

fn can_generate_result(curr_sum: i64, idx: usize, target: i64, nums: &[i64]) -> bool {
    if idx == nums.len() {
        return curr_sum == target;
    }
    
    can_generate_result(curr_sum + nums[idx], idx + 1, target, nums) ||
    can_generate_result(curr_sum * nums[idx], idx + 1, target, nums) ||
    can_generate_result(curr_sum * 10_i64.pow(nums[idx].to_string().len() as u32) + nums[idx], idx + 1, target, nums)
}

fn calibration_result(equations: &[(i64, Vec<i64>)]) -> i64 {
    equations.iter()
        .filter(|(target, nums)| can_generate_result(nums[0], 1, *target, nums))
        .map(|(target, _)| target)
        .sum()
}

fn process_file2(filename: &str) -> i64 {
    let path = Path::new(filename);
    let file = File::open(path).expect("Unable to open file");
    let reader = io::BufReader::new(file);
    let mut equations = Vec::new();

    for line in reader.lines() {
        if let Ok(line) = line {
            let parts: Vec<&str> = line.split(':').collect();
            if parts.len() != 2 {
                continue;
            }
            
            let target: i64 = parts[0].trim().parse().expect("Invalid target value");
            let nums: Vec<i64> = parts[1]
                .trim()
                .split_whitespace()
                .map(|x| x.parse().expect("Invalid number"))
                .collect();
            
            equations.push((target, nums));
        }
    }
    calibration_result(&equations)
}


fn main() {
    let filename = "src/input.txt";
    let total = process_file1(filename);
    println!("Total Calibration Result Part 1: {}", total);
    let total = process_file2(filename);
    println!("Total Calibration Result Part 2: {}", total);
}
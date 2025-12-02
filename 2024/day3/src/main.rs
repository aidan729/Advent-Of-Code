use std::fs;

fn main() {
    // Read the entire file into memory
    let file_content = fs::read("src/input.txt").expect("Failed to read file");

    let mut curr_char_index = 0;
    let mut accumulator: i64 = 0;
    let mut active = true;

    let file_len = file_content.len();

    while curr_char_index < file_len {
        let remaining = &file_content[curr_char_index..];

        // Check for "do()"
        if remaining.starts_with(b"do()") {
            active = true;
            curr_char_index += 4;
            continue;
        }

        // Check for "don't()"
        if remaining.starts_with(b"don't()") {
            active = false;
            curr_char_index += 6;
            continue;
        }

        // Check for "mul("
        if remaining.starts_with(b"mul(") {
            let after_mul = &remaining[4..];
            if let Some((x_str, rest)) = parse_number(after_mul) {
                if let Some((y_str, rest)) = parse_number(&rest[1..]) { // Skip the ',' after first number
                    if rest.starts_with(b")") {
                        // Convert numbers and compute
                        if let (Ok(x), Ok(y)) = (x_str.parse::<i64>(), y_str.parse::<i64>()) {
                            let result = x * y;
                            if active {
                                accumulator += result;
                            }
                        }
                        curr_char_index += 4 + x_str.len() + 1 + y_str.len() + 1; // Advance past "mul(x,y)"
                        continue;
                    }
                }
            }
        }

        // Move to the next character
        curr_char_index += 1;
    }

    // Print the final result
    println!("{}", accumulator);
}

/// Parses a number from the start of the given byte slice and returns the number
/// as a string and the remaining slice.
fn parse_number(data: &[u8]) -> Option<(&str, &[u8])> {
    let len = data.len();
    let mut end_index = 0;

    // Find the first non-digit character
    while end_index < len && data[end_index].is_ascii_digit() {
        end_index += 1;
    }

    if end_index > 0 {
        Some((std::str::from_utf8(&data[..end_index]).ok()?, &data[end_index..]))
    } else {
        None
    }
}

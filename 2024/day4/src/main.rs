use std::fs;
use std::collections::HashMap;

fn count_word(grid: &[Vec<char>], word: &str) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();
    let word_chars: Vec<char> = word.chars().collect();
    let word_len = word_chars.len();
    let mut count = 0;

    let directions = [
        (0, 1),   // right
        (1, 0),   // down
        (1, 1),   // down-right diag
        (1, -1),  // down-left diag
        (0, -1),  // left
        (-1, 0),  // up
        (-1, -1), // up-left diag
        (-1, 1),  // up-right diag
    ];

    for row in 0..rows {
        for col in 0..cols {
            for &(dx, dy) in &directions {
                let mut matched = true;
                for i in 0..word_len {
                    let nx = row as isize + dx * i as isize;
                    let ny = col as isize + dy * i as isize;

                    if nx < 0 || nx >= rows as isize || ny < 0 || ny >= cols as isize {
                        matched = false;
                        break;
                    }

                    if grid[nx as usize][ny as usize] != word_chars[i] {
                        matched = false;
                        break;
                    }
                }
                if matched {
                    count += 1;
                }
            }
        }
    }
    count
}

fn is_cross(pos: (i32, i32), lut: &HashMap<(i32, i32), char>) -> bool {
    let (x, y) = pos;

    let diagonals = [
        (x - 1, y - 1),
        (x + 1, y - 1),
        (x - 1, y + 1),
        (x + 1, y + 1),
    ];

    if diagonals.iter().any(|&n| !lut.contains_key(&n)) {
        return false;
    }

    let chars: Vec<char> = diagonals.iter().map(|&n| lut.get(&n).unwrap()).cloned().collect();

    let valid_patters = ["MSMS", "SMSM", "MMSS", "SSMM"];
    let diag_str: String = chars.iter().collect();

    valid_patters.contains(&&diag_str.as_str())
}

fn count_x_mas(grid: &[Vec<char>]) -> usize {
    let mut lut: HashMap<(i32, i32), char> = HashMap::new();

    for (y, row) in grid.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            if c == 'X' || c == 'M' || c == 'A' || c == 'S' {
                lut.insert((x as i32, y as i32), c);
            }
        }
    }
    
    let mut count = 0;

    for (&(x, y), &c) in &lut {
        if c == 'A' && is_cross((x, y), &lut) {
            count += 1;
        }
    }

    count
}
fn main() {
    let input = fs::read_to_string("src/input.txt").expect("Failed to read input file");
    let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let word = "XMAS";
    let occurrences = count_word(&grid, word);
    println!("The Word '{}' appears {} times in the search", word, occurrences);

    let xmas_occurrences = count_x_mas(&grid);
    println!("The X-MAS Pattern appears {} times in the search", xmas_occurrences);
}

use std::fs;
use std::iter::from_fn;

fn parse_input(input: &str) -> (Vec<Vec<usize>>, Vec<Vec<usize>>) {
    let mut rules = vec![Vec::new(); 100];
    let mut updates = Vec::new();

    let mut line_iter = input.lines().into_iter();

    while let Some(line) = line_iter.next() {
        if line.is_empty() {
            break;
        }
        let mut parts = line.split('|');
        let x: usize = parts.next().unwrap().parse().unwrap();
        let y: usize = parts.next().unwrap().parse().unwrap();
        rules[x].push(y);
    }

    for line in line_iter {
        if !line.is_empty() {
            let mut bytes = line.bytes();
            updates.push(from_fn(|| {
                let mut num = Vec::new();
                while let Some(b) = bytes.next() {
                    if b == b',' {
                        break;
                    }
                    num.push(b);
                }
                if num.is_empty() {
                    None
                } else {
                    Some(String::from_utf8(num).unwrap().parse().unwrap())
                }
            }).collect());
        }
    }

    (rules, updates)
}

fn check_update(update: &Vec<usize>, rules: &Vec<Vec<usize>>) -> Option<usize> {
    for i in 0..update.len() - 1 {
        if !rules[update[i]].contains(&update[i + 1]) {
            return None;
        }
    }
    Some(update[update.len() / 2])
}

fn find_next_page(update: &Vec<usize>, rules: &Vec<Vec<usize>>) -> Option<usize> {
    for i in 0..update.len() {
        let rule = &rules[update[i]];
        let mut found = true;
        for n in 0..update.len() {
            if n == i {
                continue;
            }

            if rule.contains(&update[n]) {
                found = false;
                break;
            }
        }
        if found {
            return Some(i);
        }
    }
    None
}

fn reorder_update(update: &mut Vec<usize>, rules: &Vec<Vec<usize>>) -> usize {
    let mut result = 0;

    for _ in 0..=update.len() / 2 {
        if let Some(page_index) = find_next_page(update, rules) {
            result = update.remove(page_index);
        }
    }
    result
}

fn main() {
    let input = fs::read_to_string("src/input.txt").expect("Failed to read input file");
    let (rules, mut updates) = parse_input(&input);

    let part1_result: usize = updates
        .iter()
        .filter_map(|update| check_update(update, &rules))
        .sum();

    println!("The sum of middle pages of correctly ordered updates is: {}", part1_result);

    let part2_result: usize = updates
        .iter_mut()
        .filter(|update| check_update(update, &rules).is_none())
        .map(|update| reorder_update(update, &rules))
        .sum();

    println!("The sum of middle pages of reordered updates is: {}", part2_result);
}

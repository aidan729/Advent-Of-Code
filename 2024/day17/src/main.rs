use std::fs;

#[derive(Debug, Clone)]
struct Computer {
    a: i64,
    b: i64,
    c: i64,
    ip: usize,
    program: Vec<u8>,
    output: Vec<u8>,
}

impl Computer {
    fn new(a: i64, b: i64, c: i64, program: Vec<u8>) -> Self {
        Self {
            a,
            b,
            c,
            ip: 0,
            program,
            output: Vec::new(),
        }
    }

    fn combo_value(&self, operand: u8) -> i64 {
        match operand {
            0..=3 => operand as i64,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => unreachable!("Invalid combo operand: {}", operand),
        }
    }

    fn run(&mut self) {
        while self.ip < self.program.len() {
            let opcode = self.program[self.ip];
            let operand = self.program[self.ip + 1];

            match opcode {
                0 => {
                    // adv: A = A / 2^combo
                    let divisor = 1i64 << self.combo_value(operand);
                    self.a /= divisor;
                    self.ip += 2;
                }
                1 => {
                    // bxl: B = B XOR literal
                    self.b ^= operand as i64;
                    self.ip += 2;
                }
                2 => {
                    // bst: B = combo % 8
                    self.b = self.combo_value(operand) & 7;
                    self.ip += 2;
                }
                3 => {
                    // jnz: jump if A != 0
                    if self.a != 0 {
                        self.ip = operand as usize;
                    } else {
                        self.ip += 2;
                    }
                }
                4 => {
                    // bxc: B = B XOR C (operand ignored)
                    self.b ^= self.c;
                    self.ip += 2;
                }
                5 => {
                    // out: output combo % 8
                    self.output.push((self.combo_value(operand) & 7) as u8);
                    self.ip += 2;
                }
                6 => {
                    // bdv: B = A / 2^combo
                    let divisor = 1i64 << self.combo_value(operand);
                    self.b = self.a / divisor;
                    self.ip += 2;
                }
                7 => {
                    // cdv: C = A / 2^combo
                    let divisor = 1i64 << self.combo_value(operand);
                    self.c = self.a / divisor;
                    self.ip += 2;
                }
                _ => unreachable!("Invalid opcode: {}", opcode),
            }
        }
    }

    fn output_string(&self) -> String {
        self.output
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<_>>()
            .join(",")
    }
}

fn main() {
    let input = fs::read_to_string("src/input.txt").expect("Failed to read input");
    let part1 = solve_part1(&input);
    let part2 = solve_part2(&input);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn solve_part1(input: &str) -> String {
    let (a, b, c, program) = parse_input(input);
    let mut computer = Computer::new(a, b, c, program);
    computer.run();
    computer.output_string()
}

fn solve_part2(input: &str) -> i64 {
    let (_, b, c, program) = parse_input(input);

    // work backwards from the output
    // the program processes A in 3-bit chunks, outputting one value per iteration
    // and dividing A by 8 (right shift 3) each time
    // we can build A from the least significant chunks working backwards

    find_quine(0, 0, &program, b, c)
}

fn find_quine(a: i64, depth: usize, program: &[u8], b: i64, c: i64) -> i64 {
    // matched all outputs
    if depth == program.len() {
        // verify the complete output matches
        let mut computer = Computer::new(a, b, c, program.to_vec());
        computer.run();
        if computer.output == program {
            return a;
        }
        return -1;
    }

    // try all possible 3-bit values for this position
    // building from most significant bits
    for candidate in 0..8 {
        let test_a = (a << 3) | candidate;

        // run and check if output suffix matches expected suffix
        let mut computer = Computer::new(test_a, b, c, program.to_vec());
        computer.run();

        // check if the output matches the last (depth+1) elements of program
        let suffix_start = program.len() - depth - 1;
        let expected_suffix = &program[suffix_start..];

        if computer.output.len() >= expected_suffix.len()
            && &computer.output[computer.output.len() - expected_suffix.len()..] == expected_suffix
        {
            let result = find_quine(test_a, depth + 1, program, b, c);
            if result != -1 {
                return result;
            }
        }
    }

    -1
}

fn parse_input(input: &str) -> (i64, i64, i64, Vec<u8>) {
    let lines: Vec<&str> = input.lines().collect();

    let a = lines[0]
        .strip_prefix("Register A: ")
        .unwrap()
        .parse()
        .unwrap();
    let b = lines[1]
        .strip_prefix("Register B: ")
        .unwrap()
        .parse()
        .unwrap();
    let c = lines[2]
        .strip_prefix("Register C: ")
        .unwrap()
        .parse()
        .unwrap();

    let program = lines[4]
        .strip_prefix("Program: ")
        .unwrap()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();

    (a, b, c, program)
}

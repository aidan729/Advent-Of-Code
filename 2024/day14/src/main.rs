use std::fs;

#[derive(Debug, Clone, Copy)]
struct Robot {
    px: i32,
    py: i32,
    vx: i32,
    vy: i32,
}

const WIDTH: i32 = 101;
const HEIGHT: i32 = 103;

fn main() {
    let input = fs::read_to_string("src/input.txt").expect("Failed to read input");
    let robots = parse_robots(&input);

    let part1 = solve_part1(&robots);
    let part2 = solve_part2(&robots);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn parse_robots(input: &str) -> Vec<Robot> {
    input
        .lines()
        .filter_map(|line| {
            // parse "p=18,60 v=90,-17" using split black magic
            let (p, v) = line.split_once(" v=")?;
            let (px, py) = p.strip_prefix("p=")?.split_once(',')?;
            let (vx, vy) = v.split_once(',')?;

            Some(Robot {
                px: px.parse().ok()?,
                py: py.parse().ok()?,
                vx: vx.parse().ok()?,
                vy: vy.parse().ok()?,
            })
        })
        .collect()
}

fn solve_part1(robots: &[Robot]) -> usize {
    // we can compute final position directly without simulation
    // position after t seconds: (px + vx * t) mod WIDTH
    // This is O(n) instead of O(n * t) if we simulated each step

    let positions: Vec<(i32, i32)> = robots
        .iter()
        .map(|r| {
            // rem_euclid handles negative velocities correctly (wrapping)
            let x = (r.px + r.vx * 100).rem_euclid(WIDTH);
            let y = (r.py + r.vy * 100).rem_euclid(HEIGHT);
            (x, y)
        })
        .collect();

    // count robots in each quadrant
    // robots exactly in the middle (on dividing lines) dont count
    let mid_x = WIDTH / 2;
    let mid_y = HEIGHT / 2;

    let mut quadrants = [0usize; 4];

    for &(x, y) in &positions {
        // skip robots on the dividing lines
        if x == mid_x || y == mid_y {
            continue;
        }

        // bit manipulation trick to compute quadrant index
        // quadrant = (x > mid_x) as usize | ((y > mid_y) as usize) << 1
        // this maps to: TL=0, TR=1, BL=2, BR=3
        let quad = ((x > mid_x) as usize) | (((y > mid_y) as usize) << 1);
        quadrants[quad] += 1;
    }

    // safety factor is product of all quadrant counts
    quadrants.iter().product()
}

fn solve_part2(robots: &[Robot]) -> usize {
    // christmas tree = high clustering = low variance
    // instead of pattern matching detect when robots form tight cluster
    // variance minimization is O(n) per iteration vs expensive image analysis

    let mut min_variance = f64::MAX;
    let mut best_time = 0;

    // the pattern repeats after WIDTH * HEIGHT seconds (LCM of dimensions)
    // but it will appear much sooner (scan first 10k seconds)
    for t in 0..10000 {
        let positions: Vec<(i32, i32)> = robots
            .iter()
            .map(|r| {
                let x = (r.px + r.vx * t).rem_euclid(WIDTH);
                let y = (r.py + r.vy * t).rem_euclid(HEIGHT);
                (x, y)
            })
            .collect();

        // calculate variance as clustering metric
        // low variance = tight cluster = potential tree
        let variance = calculate_variance(&positions);

        if variance < min_variance {
            min_variance = variance;
            best_time = t;
        }
    }

    best_time as usize
}

fn calculate_variance(positions: &[(i32, i32)]) -> f64 {
    // bit of math flex: variance = E[X²] - E[X]²
    // computing both x and y variance simultaneously

    let n = positions.len() as f64;

    let (sum_x, sum_y, sum_x2, sum_y2) = positions
        .iter()
        .fold((0.0, 0.0, 0.0, 0.0), |(sx, sy, sx2, sy2), &(x, y)| {
            let xf = x as f64;
            let yf = y as f64;
            (sx + xf, sy + yf, sx2 + xf * xf, sy2 + yf * yf)
        });

    let mean_x = sum_x / n;
    let mean_y = sum_y / n;

    let var_x = (sum_x2 / n) - (mean_x * mean_x);
    let var_y = (sum_y2 / n) - (mean_y * mean_y);

    // total variance combines both dimensions
    var_x + var_y
}
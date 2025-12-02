use std::fs::File;
use std::collections::{HashMap, HashSet};
use std::io::{BufRead, BufReader};

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn main() {
    let input = File::open("src/input.txt").expect("Unable to open file");
    let reader = BufReader::new(input);

    let input: Vec<String> = reader.lines().map(|x| x.unwrap()).collect();

    let height = input.len();

    if height == 0 {
        println!("No input provided");
        return;
    }
    let width = input[0].len();

    let mut antennas: HashMap<char, Vec<(usize, usize)>> = HashMap::new();
    for (r, line) in input.iter().enumerate() {
        for (c, ch) in line.chars().enumerate() {
            if ch != '.' {
                antennas.entry(ch).or_default().push((r, c));
            }
        }
    }

    let mut antinodes = HashSet::new();


    //---------------------------------------------
    // Reflections of A and B across each other
    // A = (r1, c1) and B = (r2, c2)
    // Reflection of B across A = (2r1 - r2, 2c1 - c2)
    // Reflection of A across B = (2r2 - r1, 2c2 - c1)
    // --------------------------------------------

    for coords in antennas.values() {
        for i in 0..coords.len() {
            for j in i+1..coords.len() {
                let (r1, c1) = coords[i];
                let (r2, c2) = coords[j];

                // Reflection of B across A
                let ar = 2i64 * (r1 as i64) - (r2 as i64);
                let ac = 2i64 * (c1 as i64) - (c2 as i64);
                if ar >= 0 && ar < height as i64 && ac >= 0 && ac < width as i64 {
                    antinodes.insert((ar as usize, ac as usize));
                }

                // Reflection of A across B
                let br = 2i64 * (r2 as i64) - (r1 as i64);
                let bc = 2i64 * (c2 as i64) - (c1 as i64);
                if br >= 0 && br < height as i64 && bc >= 0 && bc < width as i64 {
                    antinodes.insert((br as usize, bc as usize));
                }
            }
        }
    }

    //---------------------------------------------
    // Part 2: Collinerar based antinodes
    //---------------------------------------------
    //
    // "An antinode occurs at any grid position exactly in line with
    //  at least two antennas of the same frequency, regardless of distance."
    //
    // Approach:
    // - For each pair of same-frequency antennas (A, B),
    //   compute the reduced direction vector (dr, dc).
    // - Start from A and move forward in steps of (dr, dc) until out of bounds,
    //   marking each in-bounds grid position as an antinode.
    // - Also move backward from A in steps of (-dr, -dc) until out of bounds.
    // - We do i < j to avoid repeating the exact same pair in reverse,
    //   but note we *do* run the line from each pair’s perspective.
    //   This is fine, because duplicates go into a HashSet.
    //
    // This will hit all collinear integer points in the grid that lie on
    // the line defined by (A, B).
    //

    // Convert grid size to i64 so we can compare with rr, cc safely.
    let height_i64 = height as i64;
    let width_i64 = width as i64;

    for coords in antennas.values() {
        if coords.len() < 2 {
            // Only one (or zero) antenna for this frequency => cannot form lines
            continue;
        }

        for i in 0..coords.len() {
            for j in i + 1..coords.len() {
                let (r1, c1) = coords[i];
                let (r2, c2) = coords[j];

                // If two antennas share the same position, skip
                if r1 == r2 && c1 == c2 {
                    continue;
                }

                // Compute the reduced slope (dr, dc)
                let mut dr = r2 as i64 - r1 as i64;
                let mut dc = c2 as i64 - c1 as i64;
                let g = gcd(dr, dc);
                dr /= g;
                dc /= g;

                // -- Move forward from (r1,c1) in direction (dr, dc) --
                let (mut rr, mut cc) = (r1 as i64, c1 as i64);
                while rr >= 0 && rr < height_i64 && cc >= 0 && cc < width_i64 {
                    antinodes.insert((rr as usize, cc as usize));
                    rr += dr;
                    cc += dc;
                }

                // -- Move backward from (r1,c1) in direction (-dr, -dc) --
                let (mut rr, mut cc) = (r1 as i64, c1 as i64);
                while rr >= 0 && rr < height_i64 && cc >= 0 && cc < width_i64 {
                    antinodes.insert((rr as usize, cc as usize));
                    rr -= dr;
                    cc -= dc;
                }
            }
        }
    }

    print!("{}", antinodes.len());
}
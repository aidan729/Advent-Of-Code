use std::collections::HashMap;

fn main() {
    let input = std::fs::read_to_string("src/input.txt").unwrap();
    println!("Part 1: {}", solve_part1(&input));
    println!("Part 2: {}", solve_part2(&input));
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point3D {
    x: i32,
    y: i32,
    z: i32,
}

impl Point3D {
    fn distance_squared(&self, other: &Point3D) -> i64 {
        let dx = (self.x - other.x) as i64;
        let dy = (self.y - other.y) as i64;
        let dz = (self.z - other.z) as i64;
        dx * dx + dy * dy + dz * dz
    }
}

// union find data structure
struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]); // path compression
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x == root_y {
            return false; // already in same set
        }

        // union by size
        if self.size[root_x] < self.size[root_y] {
            self.parent[root_x] = root_y;
            self.size[root_y] += self.size[root_x];
        } else {
            self.parent[root_y] = root_x;
            self.size[root_x] += self.size[root_y];
        }

        true
    }

    fn get_component_sizes(&mut self) -> Vec<usize> {
        let mut sizes = HashMap::new();
        for i in 0..self.parent.len() {
            let root = self.find(i);
            *sizes.entry(root).or_insert(0) += 1;
        }
        sizes.values().copied().collect()
    }
}

fn solve_part1(input: &str) -> usize {
    // parse all junction box positions
    let points: Vec<Point3D> = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let parts: Vec<i32> = line.split(',').map(|s| s.trim().parse().unwrap()).collect();
            Point3D {
                x: parts[0],
                y: parts[1],
                z: parts[2],
            }
        })
        .collect();

    let n = points.len();
    println!("Total junction boxes: {}", n);

    // generate all pairs with their distances
    let mut edges: Vec<(i64, usize, usize)> = Vec::new();
    for i in 0..n {
        for j in i + 1..n {
            let dist_sq = points[i].distance_squared(&points[j]);
            edges.push((dist_sq, i, j));
        }
    }

    // sort edges by distance
    edges.sort_unstable();

    // use union find to track connected components
    let mut uf = UnionFind::new(n);

    // make 1000 connections (attempt to connect 1000 pairs)
    let mut connections_attempted = 0;
    let mut successful_unions = 0;
    for (_, i, j) in edges {
        if connections_attempted >= 1000 {
            break;
        }
        connections_attempted += 1;

        if uf.union(i, j) {
            successful_unions += 1;
        }
    }

    println!("Connections attempted: {}", connections_attempted);
    println!("Successful unions: {}", successful_unions);

    // get all component sizes
    let mut sizes = uf.get_component_sizes();
    sizes.sort_unstable_by(|a, b| b.cmp(a)); // sort descending

    println!("Total components: {}", sizes.len());
    println!("Top 5 component sizes: {:?}", &sizes[..sizes.len().min(5)]);

    // multiply the three largest
    if sizes.len() >= 3 {
        sizes[0] * sizes[1] * sizes[2]
    } else {
        sizes.iter().product()
    }
}

fn solve_part2(input: &str) -> i32 {
    // parse all junction box positions
    let points: Vec<Point3D> = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let parts: Vec<i32> = line.split(',').map(|s| s.trim().parse().unwrap()).collect();
            Point3D {
                x: parts[0],
                y: parts[1],
                z: parts[2],
            }
        })
        .collect();

    let n = points.len();

    // generate all pairs with their distances
    let mut edges: Vec<(i64, usize, usize)> = Vec::new();
    for i in 0..n {
        for j in i + 1..n {
            let dist_sq = points[i].distance_squared(&points[j]);
            edges.push((dist_sq, i, j));
        }
    }

    // sort edges by distance
    edges.sort_unstable();

    // use union find to track connected components
    let mut uf = UnionFind::new(n);

    // keep connecting until we have only 1 component
    let mut last_connection = (0, 0);
    for (_, i, j) in edges {
        if uf.union(i, j) {
            last_connection = (i, j);

            // check if we have only 1 component left
            let sizes = uf.get_component_sizes();
            if sizes.len() == 1 {
                break;
            }
        }
    }

    // multiply the X coordinates of the last two junction boxes connected
    let (i, j) = last_connection;
    points[i].x * points[j].x
}

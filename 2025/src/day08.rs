use std::{ cmp::Reverse, collections::HashSet, fs };

pub fn solve() {
    let input = fs::read_to_string("inputs/day08.txt").expect("Failed to read input file");
    let solver = Day08::new(&input, 1000);
    println!("Part 1: {}", solver.part1());
    println!("Part 2: {}", solver.part2());
}
#[derive(Clone, PartialEq, Debug)]
struct Point {
    x: f64,
    y: f64,
    z: f64,
}

impl Point {
    fn distance(&self, other: &Point) -> f64 {
        (
            (self.x - other.x).powi(2) +
            (self.y - other.y).powi(2) +
            (self.z - other.z).powi(2)
        ).sqrt()
    }
}

struct Day08 {
    points: Vec<Point>,
    connections: u32,
    distances: Vec<(usize, usize, f64)>, // Precomputed sorted distances
}

impl Day08 {
    fn new(input: &str, connections: u32) -> Self {
        let points: Vec<Point> = input
            .lines()
            .map(|line| {
                let coords: Vec<f64> = line
                    .split(',')
                    .map(|s| s.parse().unwrap())
                    .collect();
                Point { x: coords[0], y: coords[1], z: coords[2] }
            })
            .collect();

        // Precompute and sort all distances
        let mut distances: Vec<(usize, usize, f64)> = Vec::new();
        for i in 0..points.len() {
            for j in i + 1..points.len() {
                distances.push((i, j, points[i].distance(&points[j])));
            }
        }
        distances.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());

        Day08 { points, connections, distances }
    }

    fn part1(&self) -> u64 {
        let mut conn = 0;
        let n = self.points.len();

        // Union-Find: parent[i] = parent of node i (or itself if root)
        let mut parent: Vec<usize> = (0..n).collect();
        let mut rank: Vec<usize> = vec![0; n];

        fn find(parent: &mut Vec<usize>, i: usize) -> usize {
            if parent[i] != i {
                parent[i] = find(parent, parent[i]); // Path compression
            }
            parent[i]
        }

        fn union(parent: &mut Vec<usize>, rank: &mut Vec<usize>, i: usize, j: usize) -> bool {
            let ri = find(parent, i);
            let rj = find(parent, j);
            if ri == rj {
                return false; // Already in same circuit
            }
            // Union by rank
            if rank[ri] < rank[rj] {
                parent[ri] = rj;
            } else if rank[ri] > rank[rj] {
                parent[rj] = ri;
            } else {
                parent[rj] = ri;
                rank[ri] += 1;
            }
            true
        }

        for &(i, j, _) in &self.distances {
            if conn >= self.connections {
                break;
            }

            // Always count the connection attempt
            conn += 1;

            // Try to merge circuits (may do nothing if already connected)
            union(&mut parent, &mut rank, i, j);
        }

        // Count circuit sizes
        let mut circuit_sizes: std::collections::HashMap<
            usize,
            usize
        > = std::collections::HashMap::new();
        for i in 0..n {
            let root = find(&mut parent, i);
            *circuit_sizes.entry(root).or_insert(0) += 1;
        }

        // Sort by size descending
        let mut sizes: Vec<usize> = circuit_sizes.values().cloned().collect();
        sizes.sort_by_key(|&s| Reverse(s));

        println!("Top circuits: {:?}", &sizes[..(3).min(sizes.len())]);

        let mut total = 1u64;
        for i in 0..(3).min(sizes.len()) {
            total *= sizes[i] as u64;
        }
        total
    }

    fn part2(&self) -> u64 {
        let n = self.points.len();

        // Union-Find: parent[i] = parent of node i (or itself if root)
        let mut parent: Vec<usize> = (0..n).collect();
        let mut rank: Vec<usize> = vec![0; n];

        fn find(parent: &mut Vec<usize>, i: usize) -> usize {
            if parent[i] != i {
                parent[i] = find(parent, parent[i]); // Path compression
            }
            parent[i]
        }

        fn union(parent: &mut Vec<usize>, rank: &mut Vec<usize>, i: usize, j: usize) -> bool {
            let ri = find(parent, i);
            let rj = find(parent, j);
            if ri == rj {
                return false; // Already in same circuit
            }
            // Union by rank
            if rank[ri] < rank[rj] {
                parent[ri] = rj;
            } else if rank[ri] > rank[rj] {
                parent[rj] = ri;
            } else {
                parent[rj] = ri;
                rank[ri] += 1;
            }
            true
        }

        // Count how many distinct circuits we have
        let mut num_circuits = n;
        let mut last_i = 0;
        let mut last_j = 0;

        for &(i, j, _) in &self.distances {
            // Stop when everything is in one circuit
            if num_circuits == 1 {
                break;
            }

            // Try to merge circuits - only succeeds if they were different
            if union(&mut parent, &mut rank, i, j) {
                num_circuits -= 1;
                last_i = i;
                last_j = j;
            }
        }

        // Return product of X coordinates of the last two connected points
        let x1 = self.points[last_i].x as u64;
        let x2 = self.points[last_j].x as u64;
        x1 * x2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_input() -> &'static str {
        r#"162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689"#
    }

    #[test]
    fn test_part1() {
        let solver = Day08::new(test_input(), 10);
        assert_eq!(solver.part1(), 40);
    }

    #[test]
    fn test_part2() {
        let solver = Day08::new(test_input(), 10);
        assert_eq!(solver.part2(), 25272);
    }
}

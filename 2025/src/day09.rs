use std::{ cmp::Reverse, collections::HashSet, fs };
use glam::I64Vec2;
use itertools::Itertools;
use nom::{
    IResult,
    Parser,
    bytes::complete::tag,
    character::complete::{ self, line_ending },
    multi::separated_list1,
    sequence::separated_pair,
};

pub fn solve() {
    let input = fs::read_to_string("inputs/day09.txt").expect("Failed to read input file");
    let solver = Day09::new(&input);
    println!("Part 1: {}", solver.part1());
    println!("Part 2: {}", solver.part2());
}
#[derive(Clone, PartialEq, Debug)]
struct Point {
    x: u64,
    y: u64,
}

impl Point {
    fn size(&self, other: &Point) -> u64 {
        ((self.x.abs_diff(other.x) as u64) + 1) * ((self.y.abs_diff(other.y) as u64) + 1)
    }
}

struct Day09 {
    lines: String,
    sizes: Vec<(Point, Point, u64)>, // Precomputed sorted sizes
}

impl Day09 {
    fn new(input: &str) -> Self {
        let points: Vec<Point> = input
            .lines()
            .map(|line| {
                let coords: Vec<u64> = line
                    .split(',')
                    .map(|s| s.parse().unwrap())
                    .collect();
                Point { x: coords[1], y: coords[0] }
            })
            .collect();

        // Precompute and sort all distances
        let mut sizes: Vec<(Point, Point, u64)> = Vec::new();
        for i in 0..points.len() {
            for j in i + 1..points.len() {
                sizes.push((points[i].clone(), points[j].clone(), points[i].size(&points[j])));
            }
        }
        sizes.sort_by(|a, b| Reverse(a.2).partial_cmp(&Reverse(b.2)).unwrap());

        Day09 { lines: input.to_string(), sizes }
    }

    fn part1(&self) -> u64 {
        self.sizes[0].2
    }

    fn part2(&self) -> u64 {
        let (_, red_tiles) = parse(&self.lines).unwrap();
        let lines = red_tiles
            .iter()
            .circular_tuple_windows()
            .collect::<Vec<(&I64Vec2, &I64Vec2)>>();
        let max_box = red_tiles
            .iter()
            .tuple_combinations()
            .map(|(a, b)| {
                let area = (a.x.abs_diff(b.x) + 1) * (a.y.abs_diff(b.y) + 1);
                (a, b, area)
            })
            .sorted_by_key(|v| v.2)
            .rev()
            .find(|(a, b, area)| {
                lines.iter().all(|(line_start, line_end)| {
                    // if line is to left
                    let left_of_rect = a.x.max(b.x) <= line_start.x.min(line_end.x);
                    let right_of_rect = a.x.min(b.x) >= line_start.x.max(line_end.x);
                    let above = a.y.max(b.y) <= line_start.y.min(line_end.y);
                    let below = a.y.min(b.y) >= line_start.y.max(line_end.y);
                    left_of_rect || right_of_rect || above || below
                })
            });
        max_box.unwrap().2
    }
}

fn parse(input: &str) -> IResult<&str, Vec<I64Vec2>> {
    separated_list1(
        line_ending,
        separated_pair(complete::i64, tag(","), complete::i64).map(|(x, y)| I64Vec2::new(x, y))
    ).parse(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_input() -> &'static str {
        r#"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3"#
    }

    #[test]
    fn test_part1() {
        let solver = Day09::new(test_input());
        assert_eq!(solver.part1(), 50);
    }

    #[test]
    fn test_part2() {
        let solver = Day09::new(test_input());
        assert_eq!(solver.part2(), 24);
    }
}

use std::fs;

pub fn solve() {
    let input = fs::read_to_string("inputs/day05.txt").expect("Failed to read input file");

    let mut solver = Day05::new(&input);
    println!("Part 1: {}", solver.part1());
    println!("Part 2: {}", solver.part2());
}

struct Day05 {
    ranges: Vec<(u64, u64)>,
    numbers: Vec<u64>,
}

impl Day05 {
    fn new(input: &str) -> Self {
        let lines: Vec<&str> = input.lines().collect();
        let mut ranges = Vec::new();
        let mut numbers = Vec::new();
        let mut parsing_ranges = true;

        for line in lines {
            if line.is_empty() {
                parsing_ranges = false;
                continue;
            }

            if parsing_ranges {
                let parts: Vec<&str> = line.split('-').collect();
                if parts.len() == 2 {
                    let start: u64 = parts[0].parse().expect("Failed to parse start");
                    let end: u64 = parts[1].parse().expect("Failed to parse end");
                    ranges.push((start, end));
                }
            } else {
                let num: u64 = line.parse().expect("Failed to parse number");
                numbers.push(num);
            }
        }

        Day05 { ranges, numbers }
    }

    fn is_in_range(&self, num: u64) -> bool {
        self.ranges.iter().any(|&(start, end)| num >= start && num <= end)
    }

    fn merge_ranges(&mut self) -> Vec<(u64, u64)> {
        self.ranges.sort_by_key(|&(start, _)| start);

        if self.ranges.is_empty() {
            return Vec::new();
        }

        let mut merged = Vec::new();
        let (mut current_start, mut current_end) = self.ranges[0];

        for &(start, end) in &self.ranges[1..] {
            if start > current_end + 1 {
                // No overlap, push current and start new range
                merged.push((current_start, current_end));
                current_start = start;
                current_end = end;
            } else {
                // Overlap or adjacent, extend current range
                current_end = current_end.max(end);
            }
        }

        // Don't forget the last range
        merged.push((current_start, current_end));
        merged
    }

    fn part1(&self) -> u64 {
        self.numbers
            .iter()
            .filter(|&&num| self.is_in_range(num))
            .count() as u64
    }

    fn part1_pattern(&self) -> Vec<char> {
        self.numbers
            .iter()
            .map(|&num| if self.is_in_range(num) { 'f' } else { 's' })
            .collect()
    }

    fn part2(&mut self) -> u64 {
        self.merge_ranges()
            .iter()
            .map(|(start, end)| end - start + 1)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_input() -> &'static str {
        r#"3-5
10-14
16-20
12-18

1
5
8
11
17
32"#
    }

    #[test]
    fn test_part1() {
        let solver = Day05::new(test_input());
        assert_eq!(solver.part1(), 3);
    }

    #[test]
    fn test_part1_pattern() {
        let solver = Day05::new(test_input());
        assert_eq!(solver.part1_pattern(), ['s', 'f', 's', 'f', 'f', 's']);
    }

    #[test]
    fn test_part2() {
        let mut solver = Day05::new(test_input());
        assert_eq!(solver.part2(), 14);
    }
}

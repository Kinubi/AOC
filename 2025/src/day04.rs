use core::num;
use std::fs;

pub fn solve() {
    let input = fs::read_to_string("inputs/day04.txt").expect("Failed to read input file");

    let solver = Day04::new(&input);
    println!("Part 1: {}", solver.part1());
    println!("Part 2: {}", solver.part2());
}

struct Day04 {
    lines: Vec<String>,
}

impl Day04 {
    fn new(input: &str) -> Self {
        let lines = input
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| line.to_string())
            .collect();

        Day04 { lines }
    }

    fn part1(&self) -> u64 {
        let grid: Vec<Vec<char>> = self.lines
            .iter()
            .map(|line| line.chars().collect())
            .collect();

        let rows = grid.len();
        let cols = grid[0].len();

        let mut result = grid.clone();

        // 8 directions: top-left, top, top-right, left, right, bottom-left, bottom, bottom-right
        let directions: [(i32, i32); 8] = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];
        let mut total_accessible = 0;

        for r in 0..rows {
            for c in 0..cols {
                if grid[r][c] == '@' {
                    // Count adjacent @ symbols
                    let mut neighbor_count = 0;
                    for (dr, dc) in directions.iter() {
                        let nr = (r as i32) + dr;
                        let nc = (c as i32) + dc;
                        if nr >= 0 && nr < (rows as i32) && nc >= 0 && nc < (cols as i32) {
                            if grid[nr as usize][nc as usize] == '@' {
                                neighbor_count += 1;
                            }
                        }
                    }

                    // Accessible if fewer than 4 neighbors - mark as x (accessed/removed)
                    if neighbor_count < 4 {
                        result[r][c] = 'x';
                        total_accessible += 1;
                    }
                }
            }
        }
        total_accessible
    }

    fn part1_pattern(&self) -> String {
        let grid: Vec<Vec<char>> = self.lines
            .iter()
            .map(|line| line.chars().collect())
            .collect();

        let rows = grid.len();
        let cols = grid[0].len();

        let mut result = grid.clone();

        // 8 directions: top-left, top, top-right, left, right, bottom-left, bottom, bottom-right
        let directions: [(i32, i32); 8] = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];

        for r in 0..rows {
            for c in 0..cols {
                if grid[r][c] == '@' {
                    // Count adjacent @ symbols
                    let mut neighbor_count = 0;
                    for (dr, dc) in directions.iter() {
                        let nr = (r as i32) + dr;
                        let nc = (c as i32) + dc;
                        if nr >= 0 && nr < (rows as i32) && nc >= 0 && nc < (cols as i32) {
                            if grid[nr as usize][nc as usize] == '@' {
                                neighbor_count += 1;
                            }
                        }
                    }

                    // Accessible if fewer than 4 neighbors - mark as x (accessed/removed)
                    if neighbor_count < 4 {
                        result[r][c] = 'x';
                    }
                }
            }
        }

        result
            .iter()
            .map(|row| row.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n")
    }

    fn part2(&self) -> u64 {
        let mut grid: Vec<Vec<char>> = self.lines
            .iter()
            .map(|line| line.chars().collect())
            .collect();

        let rows = grid.len();
        let cols = grid[0].len();

        let mut result = grid.clone();

        // 8 directions: top-left, top, top-right, left, right, bottom-left, bottom, bottom-right
        let directions: [(i32, i32); 8] = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];
        let mut incrimental_total = 100;
        let mut last_r = 0;
        let mut last_c = 0;
        let mut total_accessible = 0;
        while incrimental_total > 0 {
            incrimental_total = 0;

            for r in 0..rows {
                for c in 0..cols {
                    if grid[r][c] == 'x' {
                        result[r][c] = '.';
                    }
                    if grid[r][c] == '@' {
                        // Count adjacent @ symbols
                        let mut neighbor_count = 0;
                        for (dr, dc) in directions.iter() {
                            let nr = (r as i32) + dr;
                            let nc = (c as i32) + dc;
                            if nr >= 0 && nr < (rows as i32) && nc >= 0 && nc < (cols as i32) {
                                if grid[nr as usize][nc as usize] == '@' {
                                    neighbor_count += 1;
                                }
                            }
                        }

                        // Accessible if fewer than 4 neighbors - mark as x (accessed/removed)
                        if neighbor_count < 4 {
                            result[r][c] = 'x';
                            incrimental_total += 1;
                            last_r = r;
                            last_c = c;
                            total_accessible += 1;
                        }
                    }
                }
            }
            if incrimental_total == 0 {
                result[last_r][last_c] = 'x';
            }

            grid = result.clone();
        }

        total_accessible
    }

    fn part2_pattern(&self) -> String {
        let mut grid: Vec<Vec<char>> = self.lines
            .iter()
            .map(|line| line.chars().collect())
            .collect();

        let rows = grid.len();
        let cols = grid[0].len();

        let mut result = grid.clone();

        // 8 directions: top-left, top, top-right, left, right, bottom-left, bottom, bottom-right
        let directions: [(i32, i32); 8] = [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ];
        let mut incrimental_total = 100;
        let mut last_r = 0;
        let mut last_c = 0;
        while incrimental_total > 0 {
            incrimental_total = 0;

            for r in 0..rows {
                for c in 0..cols {
                    if grid[r][c] == 'x' {
                        result[r][c] = '.';
                    }
                    if grid[r][c] == '@' {
                        // Count adjacent @ symbols
                        let mut neighbor_count = 0;
                        for (dr, dc) in directions.iter() {
                            let nr = (r as i32) + dr;
                            let nc = (c as i32) + dc;
                            if nr >= 0 && nr < (rows as i32) && nc >= 0 && nc < (cols as i32) {
                                if grid[nr as usize][nc as usize] == '@' {
                                    neighbor_count += 1;
                                }
                            }
                        }

                        // Accessible if fewer than 4 neighbors - mark as x (accessed/removed)
                        if neighbor_count < 4 {
                            result[r][c] = 'x';
                            incrimental_total += 1;
                            last_r = r;
                            last_c = c;
                        }
                    }
                }
            }
            if incrimental_total == 0 {
                result[last_r][last_c] = 'x';
            }

            grid = result.clone();
        }

        grid.iter()
            .map(|row| row.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input =
            r#"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."#;
        let solver = Day04::new(input);
        assert_eq!(solver.part1(), 13);
    }

    #[test]
    fn test_part1_pattern() {
        let input =
            r#"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."#;
        let output =
            r#"..xx.xx@x.
x@@.@.@.@@
@@@@@.x.@@
@.@@@@..@.
x@.@@@@.@x
.@@@@@@@.@
.@.@.@.@@@
x.@@@.@@@@
.@@@@@@@@.
x.x.@@@.x."#;
        let solver = Day04::new(input);
        assert_eq!(solver.part1_pattern(), output);
    }

    #[test]
    fn test_part2() {
        let input =
            r#"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."#;

        let solver = Day04::new(input);
        assert_eq!(solver.part2(), 43);
    }

    fn test_part2_pattern() {
        let input =
            r#"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@."#;
        let output =
            r#"..........
..........
..........
...x@@....
...@@@@...
...@@@@@..
...@.@.@@.
...@@.@@@.
...@@@@@..
....@@@..."#;
        let solver = Day04::new(input);
        assert_eq!(solver.part2_pattern(), output);
    }
}

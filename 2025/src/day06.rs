use std::fs;

pub fn solve() {
    let input = fs::read_to_string("inputs/day06.txt").expect("Failed to read input file");
    let solver = Day06::new(&input);
    println!("Part 1: {}", solver.part1());
    println!("Part 2: {}", solver.part2());
}

struct Day06 {
    columns: Vec<Vec<String>>,
    raw_lines: Vec<String>,
}

impl Day06 {
    fn new(input: &str) -> Self {
        let lines: Vec<&str> = input.lines().collect();
        let num_columns = lines[0].split_whitespace().count();

        let mut columns: Vec<Vec<String>> = vec![Vec::new(); num_columns];
        for line in &lines {
            for (i, val) in line.split_whitespace().enumerate() {
                columns[i].push(val.to_string());
            }
        }

        let raw_lines = lines
            .iter()
            .map(|s| s.to_string())
            .collect();
        Day06 { columns, raw_lines }
    }

    fn part1(&self) -> u64 {
        self.columns
            .iter()
            .map(|col| {
                let op = col.last().unwrap().as_str();
                let nums: Vec<u64> = col[..col.len() - 1]
                    .iter()
                    .filter_map(|s| s.parse().ok())
                    .collect();

                match op {
                    "*" => nums.iter().product(),
                    "+" => nums.iter().sum(),
                    _ => 0,
                }
            })
            .sum()
    }

    fn part2(&self) -> u64 {
        let max_len = self.raw_lines
            .iter()
            .map(|s| s.len())
            .max()
            .unwrap_or(0);

        let lines: Vec<Vec<char>> = self.raw_lines
            .iter()
            .map(|s| {
                let mut chars: Vec<char> = s.chars().collect();
                chars.resize(max_len, ' ');
                chars
            })
            .collect();

        let op_row = lines.len() - 1;
        let mut total = 0;
        let mut col = 0;

        while col < max_len {
            let op_char = lines[op_row][col];
            if op_char != '*' && op_char != '+' {
                col += 1;
                continue;
            }

            // Find the end of this block (where all data rows have a space)
            let start_col = col;
            let mut end_col = col;
            while end_col < max_len && !(0..op_row).all(|row| lines[row][end_col] == ' ') {
                end_col += 1;
            }

            // Read columns right-to-left, rows top-to-bottom
            let nums: Vec<u64> = (start_col..end_col)
                .rev()
                .filter_map(|digit_col| {
                    let num_str: String = (0..op_row)
                        .filter_map(|row| {
                            let c = lines[row][digit_col];
                            if c.is_ascii_digit() {
                                Some(c)
                            } else {
                                None
                            }
                        })
                        .collect();
                    num_str.parse().ok()
                })
                .collect();

            total += match op_char {
                '*' => nums.iter().product(),
                '+' => nums.iter().sum(),
                _ => 0,
            };

            col = end_col;
        }
        total
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_input() -> &'static str {
        r#"123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  "#
    }

    #[test]
    fn test_part1() {
        let solver = Day06::new(test_input());
        assert_eq!(solver.part1(), 4277556);
    }

    #[test]
    fn test_part2() {
        let solver = Day06::new(test_input());
        assert_eq!(solver.part2(), 3263827);
    }
}

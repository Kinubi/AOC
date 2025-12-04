use std::fs;

pub fn solve() {
    let input = fs::read_to_string("inputs/day03.txt")
        .expect("Failed to read input file");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

pub fn part1(input: &str) -> u64 {
    let lines = input.lines().collect::<Vec<&str>>();
    let mut total: u64 = 0;
    for line in lines.iter() {
        if line.is_empty() {
            continue;
        }
        total += joltage_part_1(line);
    }
    total
}

pub fn part2(input: &str) -> u64 {
    let lines = input.lines().collect::<Vec<&str>>();
    let mut total: u64 = 0;
    for line in lines.iter() {
        if line.is_empty() {
            continue;
        }
        total += joltage_part_2(line);
    }
    total
}

// Find the two digits that form the largest concatenated value (first-occurring digit first)
fn joltage_part_1(num: &str) -> u64 {
    let digits: Vec<u64> = num.chars()
        .filter(|c| c.is_digit(10))
        .map(|c| c.to_digit(10).unwrap() as u64)
        .collect();
    
    let mut max_value = 0;
    
    for i in 0..digits.len() {
        for j in i + 1..digits.len() {
            // Concatenate with first digit first (preserving order)
            let value = digits[i] * 10 + digits[j];
            if value > max_value {
                max_value = value;
            }
        }
    }
    max_value
}

// Pick 12 digits maintaining order to form the largest possible number
fn joltage_part_2(num: &str) -> u64 {
    let digits: Vec<u32> = num.chars()
        .filter(|c| c.is_digit(10))
        .map(|c| c.to_digit(10).unwrap())
        .collect();
    
    let n = digits.len();
    let k = 12;
    let mut result: Vec<u32> = Vec::with_capacity(k);
    let mut start = 0; // Where to start looking for the next digit
    
    for i in 0..k {
        // For position i, we need to pick from start..=(n - (k - i))
        // because we need (k - i - 1) more digits after this one
        let end = n - (k - i - 1); // exclusive end, so we can pick up to end-1
        
        // Find the largest digit in range [start, end)
        let mut best_digit = 0;
        let mut best_pos = start;
        for j in start..end {
            if digits[j] > best_digit {
                best_digit = digits[j];
                best_pos = j;
            }
        }
        
        result.push(best_digit);
        start = best_pos + 1; // Next digit must come after this one
    }
    
    // Convert result to number
    result.iter().fold(0u64, |acc, &d| acc * 10 + d as u64)
}



#[cfg(test)]
mod tests {
    use super::*;
    


    #[test]
    fn test_part1() {
        let input = r#"987654321111111
811111111111119
234234234234278
818181911112111"#;
        assert_eq!(part1(input), 357);
    }
    #[test]
    fn test_part2() {
        let input = r#"987654321111111
811111111111119
234234234234278
818181911112111"#;
        assert_eq!(part2(input), 3121910778619);
    }
}

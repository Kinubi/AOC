use std::fs;

pub fn solve() {
    let input = fs::read_to_string("inputs/day02.txt")
        .expect("Failed to read input file");
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}

pub fn part1(input: &str) -> u64 {
    let lines = input.split(',').collect::<Vec<&str>>();
    let mut invalid_total: u64 = 0;
    
    for line in lines.iter() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let begin = line.split('-').next().unwrap().parse::<u64>()
            .expect("Failed to parse begin");
        let end = line.split('-').nth(1).unwrap().parse::<u64>()
            .expect("Failed to parse end");
        
        for num in begin..=end {
            if is_repeated_sequence(num) {
                invalid_total += num;
            }
        }
    }
    invalid_total
}

pub fn part2(input: &str) -> u64 {
    let lines = input.split(',').collect::<Vec<&str>>();
    let mut invalid_total: u64 = 0;
    
    for line in lines.iter() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let begin = line.split('-').next().unwrap().parse::<u64>()
            .expect("Failed to parse begin");
        let end = line.split('-').nth(1).unwrap().parse::<u64>()
            .expect("Failed to parse end");
        
        for num in begin..=end {
            if is_repeated_sequence_part2(num) {
                invalid_total += num;
            }
        }
    }
    invalid_total
}

// Check if a number is made of a sequence repeated twice
// e.g., 55 (5+5), 6464 (64+64), 123123 (123+123)
fn is_repeated_sequence(num: u64) -> bool {
    let s = num.to_string();
    let len = s.len();
    
    // Must have even length to be repeated twice
    if len % 2 != 0 {
        return false;
    }
    
    let half = len / 2;
    let first_half = &s[..half];
    let second_half = &s[half..];
    
    first_half == second_half
}

// Check if a number is made of a sequence repeated at least twice
// e.g., 111 (1 three times), 1212 (12 twice), 123123123 (123 three times)
fn is_repeated_sequence_part2(num: u64) -> bool {
    let s = num.to_string();
    let len = s.len();
    
    // Try all possible pattern lengths from 1 to len/2
    // (pattern must repeat at least twice, so max pattern length is len/2)
    for pattern_len in 1..=len / 2 {
        // Length must be divisible by pattern length
        if len % pattern_len != 0 {
            continue;
        }
        
        let pattern = &s[..pattern_len];
        let mut is_valid = true;
        
        // Check if all chunks match the pattern
        for i in (pattern_len..len).step_by(pattern_len) {
            if &s[i..i + pattern_len] != pattern {
                is_valid = false;
                break;
            }
        }
        
        if is_valid {
            return true;
        }
    }
    
    false
}



#[cfg(test)]
mod tests {
    use super::*;
    


    #[test]
    fn test_part1() {
        let input = r#"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124"#;
        assert_eq!(part1(input), 1227775554);
    }
    #[test]
    fn test_part2() {
        let input = r#"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,
1698522-1698528,446443-446449,38593856-38593862,565653-565659,
824824821-824824827,2121212118-2121212124"#;
        assert_eq!(part2(input), 4174379265);
    }
}

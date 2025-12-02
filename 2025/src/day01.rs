use std::fs;

pub fn solve() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

pub fn part1() -> i32 {
    let input = fs::read_to_string("inputs/day01.txt")
        .expect("Failed to read input file");
    
    let mut pointer = 50;
    let mut crossing_count = 0;
    let lower_bound = 0;
    let upper_bound = 99;
    let range = upper_bound - lower_bound + 1;
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        let dir = line.chars().next().unwrap();
        let mag = line.chars().skip(1).collect::<String>().parse::<i32>()
            .expect("Failed to parse magnitude");  
      match dir {
          'L' => {
              pointer = ((pointer - mag) % range + range) % range;
          }
          'R' => {
              pointer = (pointer + mag) % range;
          }
          _ => panic!("Unknown direction: {}", dir),
      }
      if pointer == 0 {
          crossing_count += 1;
      }
    }
    crossing_count
}

pub fn part2() -> i32 {
    let input = fs::read_to_string("inputs/day01.txt")
        .expect("Failed to read input file");
    
    let mut position: i32 = 50;
    let mut count = 0;
    
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        let dir = line.chars().next().unwrap();
        let mag = line.chars().skip(1).collect::<String>().parse::<i32>()
            .expect("Failed to parse magnitude");
        
        let rotation = if dir == 'L' { -mag } else { mag };
        
        if rotation >= 0 {
            position += rotation;
            count += position / 100;
            position %= 100;
        } else {
            if position == 0 {
                count += rotation / -100;
            } else if -rotation >= position {
                count += ((position + rotation) / -100) + 1;
            }
            position = (position + rotation + 10000) % 100;
        }
    }
    count
}

fn part2_with_input(input: &str) -> i32 {
    let mut position: i32 = 50;
    let mut count = 0;
    
    for line in input.lines() {
        if line.is_empty() {
            continue;
        }
        let dir = line.chars().next().unwrap();
        let mag = line.chars().skip(1).collect::<String>().parse::<i32>()
            .expect("Failed to parse magnitude");
        
        let rotation = if dir == 'L' { -mag } else { mag };
        
        if rotation >= 0 {
            position += rotation;
            count += position / 100;
            position %= 100;
        } else {
            if position == 0 {
                count += rotation / -100;
            } else if -rotation >= position {
                count += ((position + rotation) / -100) + 1;
            }
            position = (position + rotation + 10000) % 100;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let input = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82\n";
        assert_eq!(part2_with_input(input), 6);
    }
}

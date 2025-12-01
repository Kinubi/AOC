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
      let old_pointer = pointer;
      match dir {
          'L' => {
              pointer = ((pointer - mag) % range + range) % range;
              // Count crossings going left: we cross 0 when we wrap below 0
              if mag > old_pointer {
                  crossing_count += 1 + (mag - old_pointer - 1) / range;
              }
              // Also count landing exactly on 0
              if pointer == 0 {
                  crossing_count += 1;
              }
          }
          'R' => {
              pointer = (pointer + mag) % range;
              // Count crossings going right: we cross 0 when we wrap past 99
              if old_pointer + mag > upper_bound {
                  crossing_count += 1 + (old_pointer + mag - upper_bound - 1) / range;
              }
              // Also count landing exactly on 0
              if pointer == 0 {
                  crossing_count += 1;
              }
          }
          _ => panic!("Unknown direction: {}", dir),
      }
    }
    crossing_count
}



#[cfg(test)]
mod tests {
    use super::*;


}

use std::{ collections::{ HashMap, HashSet }, fs };

pub fn solve() {
    let input = fs::read_to_string("inputs/day07.txt").expect("Failed to read input file");
    let solver = Day07::new(&input);
    println!("Part 1: {}", solver.part1());
    println!("Part 2: {}", solver.part2());
}

type Count = usize;
type Position = usize;
struct Day07 {
    lines: String,
}

impl Day07 {
    fn new(input: &str) -> Self {
        Day07 { lines: input.to_string() }
    }

    fn part1(&self) -> String {
        let mut line_iter = self.lines.lines().enumerate();
        let s_position = line_iter
            .next()
            .unwrap()
            .1.chars()
            .position(|val| val == 'S')
            .unwrap();
        let (_, set): (HashSet<usize>, HashSet<(usize, usize)>) = line_iter.fold(
            (
                {
                    let mut set = HashSet::default();
                    set.insert(s_position);
                    set
                },
                HashSet::default(),
            ),
            |(positions, mut splitters), (y_index, line)| {
                let mut new_positions = HashSet::<usize>::default();
                for index in positions {
                    if line.as_bytes()[index] == ('^' as u8) {
                        new_positions.insert(index - 1);
                        new_positions.insert(index + 1);
                        splitters.insert((index, y_index));
                    } else {
                        new_positions.insert(index);
                    }
                }

                (new_positions, splitters)
            }
        );
        set.len().to_string()
    }

    fn part2(&self) -> String {
        let mut line_iter = self.lines.lines().enumerate();
        let s_position = line_iter
            .next()
            .unwrap()
            .1.chars()
            .position(|val| val == 'S')
            .unwrap();
        let map = line_iter.fold(
            {
                let mut m = HashMap::<usize, usize>::default();
                m.insert(s_position, 1);
                m
            },
            |positions, (index, line)| {
                let mut new_positions = HashMap::<usize, usize>::default();
                for (index, count) in positions {
                    if line.as_bytes()[index] == ('^' as u8) {
                        new_positions
                            .entry(index - 1)
                            .and_modify(|value| {
                                *value += count;
                            })
                            .or_insert(count);
                        new_positions
                            .entry(index + 1)
                            .and_modify(|value| {
                                *value += count;
                            })
                            .or_insert(count);
                    } else {
                        new_positions
                            .entry(index)
                            .and_modify(|value| {
                                *value += count;
                            })
                            .or_insert(count);
                    }
                }

                new_positions
            }
        );
        map.values().sum::<usize>().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_input() -> &'static str {
        r#".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
..............."#
    }

    #[test]
    fn test_part1() {
        let solver = Day07::new(test_input());
        assert_eq!(solver.part1(), "21");
    }

    #[test]
    fn test_part2() {
        let solver = Day07::new(test_input());
        assert_eq!(solver.part2(), "40");
    }
}

advent_of_code::solution!(7);

use std::collections::HashSet;
use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<usize> {
    let mut lines = input.lines();
    
    // Get starting position from the first line
    let first_line = lines.next()?;
    let s_position = first_line.chars().position(|c| c == 'S')?;

    let mut current_positions = HashSet::new();
    current_positions.insert(s_position);
    
    let mut splitters = HashSet::new();

    for (y_index, line) in lines.enumerate() {
        let mut next_positions = HashSet::new();
        let bytes = line.as_bytes();

        for &x in &current_positions {
            // Ensure we don't index out of bounds if the path spreads
            if x >= bytes.len() { continue; }

            if bytes[x] == b'^' {
                // Split logic: move left and right in the next row
                if x > 0 { next_positions.insert(x - 1); }
                if x + 1 < bytes.len() { next_positions.insert(x + 1); }
                
                // Record the splitter coordinate (x, y). 
                // y_index starts at 0 for the SECOND line of input.
                splitters.insert((x, y_index + 1));
            } else {
                // Keep moving straight down
                next_positions.insert(x);
            }
        }
        current_positions = next_positions;
    }

    Some(splitters.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut lines = input.lines();
    
    let first_line = lines.next()?;
    let s_pos = first_line.chars().position(|c| c == 'S')?;

    let mut current_paths = HashMap::new();
    current_paths.insert(s_pos, 1);

    for line in lines {
        let mut next_paths = HashMap::new();
        let bytes = line.as_bytes();

        for (x, count) in current_paths {
            if x >= bytes.len() { continue; }

            if bytes[x] == b'^' {
                if x > 0 {
                    *next_paths.entry(x - 1).or_insert(0) += count;
                }
                if x + 1 < bytes.len() {
                    *next_paths.entry(x + 1).or_insert(0) += count;
                }
            } else {
                *next_paths.entry(x).or_insert(0) += count;
            }
        }
        current_paths = next_paths;
    }

    Some(current_paths.values().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }
}

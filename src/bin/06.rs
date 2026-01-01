advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u64> {
    let lines: Vec<&str> = input.trim().lines().collect();
    let (op_line, data) = lines.split_last()?;
    let ops: Vec<&str> = op_line.split_whitespace().collect();
    let mut results: Vec<u64> = Vec::new();

    for (i, line) in data.iter().enumerate() {
        let nums: Vec<u64> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        if i == 0 {
            results = nums;
        } else {
            for (j, &n) in nums.iter().enumerate() {
                match ops.get(j).copied() {
                    Some("*") => results[j] *= n,
                    Some("+") => results[j] += n,
                    _ => continue,
                }
            }
        }
    }

    Some(results.into_iter().sum())
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines: Vec<&str> = input.lines().filter(|l| !l.is_empty()).collect();
    if lines.is_empty() { return None; }

    let (data, op_row) = lines.split_at(lines.len() - 1);
    let op_line = op_row[0];
    let max_w = data.iter().map(|l| l.len()).max()?;
    
    let grid: Vec<Vec<char>> = data.iter()
        .map(|l| l.chars().chain(std::iter::repeat(' ')).take(max_w).collect())
        .collect();

    let mut grand_total: u64 = 0;
    let mut x = max_w as i32 - 1;

    while x >= 0 {
        if grid.iter().all(|r| r[x as usize] == ' ') {
            x -= 1;
            continue;
        }

        let end_x = x;
        while x >= 0 && grid.iter().any(|r| r[x as usize] != ' ') {
            x -= 1;
        }
        let start_x = x + 1;

        let mut nums = Vec::new();
        for col in (start_x..=end_x).rev() {
            let mut val: u64 = 0;
            let mut has_digit = false;
            for row in 0..grid.len() {
                if let Some(d) = grid[row][col as usize].to_digit(10) {
                    val = val * 10 + d as u64;
                    has_digit = true;
                }
            }
            if has_digit {
                nums.push(val);
            }
        }

        let mut op = '+';
        for i in start_x..=end_x {
            if let Some(c) = op_line.chars().nth(i as usize) {
                if c == '*' || c == '+' {
                    op = c;
                    break;
                }
            }
        }

        let res: u64 = match op {
            '*' => nums.iter().product(),
            _ => nums.iter().sum(),
        };
        
        grand_total += res;
    }

    Some(grand_total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4277556));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3263827));
    }
}

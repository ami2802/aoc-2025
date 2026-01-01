advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u64> {
    let mut result: u64 = 0;

    for line in input.lines() {
        let digits: Vec<u32> = line
            .chars()
            .filter_map(|c| c.to_digit(10))
            .collect();

        let mut max_val = digits[0];
        let mut max_idx = 0;

        for (i, &v) in digits.iter().enumerate().skip(1) {
            if v > max_val {
                max_val = v;
                max_idx = i;
            }
        }
        let remaining: &[u32];
        let mut swap: bool = false;

        if max_idx == digits.len() - 1 {
            remaining = &digits[..max_idx];
            swap = true;
        } else {
            remaining = &digits[max_idx+1..];
        }


        let mut max_val2 = remaining[0];
        for &num in remaining.iter().skip(1) {
            if num > max_val2 {
                max_val2 = num;
            }
        }

        if swap {
            std::mem::swap(&mut max_val, &mut max_val2);
        }

        let combined = max_val * 10u32.pow((max_val2 as f32).log10().floor() as u32 + 1) + max_val2;
        result += combined as u64;

    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut result: u64 = 0;

    for line in input.lines() {
        let digits: Vec<u32> = line
            .chars()
            .filter_map(|c| c.to_digit(10))
            .collect();

        let need = 12;
        let mut keep: Vec<u32> = Vec::with_capacity(need);
        let mut drops = digits.len() - need;

        for &d in &digits {
            while drops > 0 && !keep.is_empty() && *keep.last().unwrap() < d {
                keep.pop();
                drops -= 1;
            }
            keep.push(d);
        }

        keep.truncate(need);

        let mut combined = 0u64;
        for d in keep {
            combined = combined * 10 + d as u64;
        }

        result += combined;
    }

    Some(result)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(357));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3121910778619));
    }
}

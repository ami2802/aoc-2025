advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
    let (ranges_text, ids_text) = input.split_once("\n\n").unwrap();

    let mut ranges: Vec<(u64, u64)> = ranges_text
        .lines()
        .map(|l| {
            let (a, b) = l.split_once('-').unwrap();
            let a = a.parse::<u64>().unwrap();
            let b = b.parse::<u64>().unwrap();
            (a.min(b), a.max(b))
        })
        .collect();

    ranges.sort_by_key(|r| r.0);
    let mut merged: Vec<(u64, u64)> = Vec::with_capacity(ranges.len());
    for (s, e) in ranges {
        if let Some(last) = merged.last_mut() {
            if s <= last.1 + 1 {
                if e > last.1 { last.1 = e; }
                continue;
            }
        }
        merged.push((s, e));
    }

    let mut ids: Vec<u64> = ids_text.lines().map(|l| l.parse::<u64>().unwrap()).collect();
    ids.sort_unstable();

    let mut present: u64 = 0;
    let mut i = 0usize;
    let n = ids.len();
    for &(s, e) in &merged {
        while i < n && ids[i] < s {
            i += 1;
        }
        while i < n && ids[i] <= e {
            present += 1;
            i += 1;
        }
        if i >= n { break; }
    }

    Some(present)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (ranges_text, _ids_text) = input.split_once("\n\n").unwrap();

    let mut ranges: Vec<(u64, u64)> = ranges_text
        .lines()
        .map(|l| {
            let (a, b) = l.split_once('-').unwrap();
            let a = a.parse::<u64>().unwrap();
            let b = b.parse::<u64>().unwrap();
            (a.min(b), a.max(b))
        })
        .collect();

    ranges.sort_by_key(|r| r.0);
    let mut merged: Vec<(u64, u64)> = Vec::with_capacity(ranges.len());
    for (s, e) in ranges {
        if let Some(last) = merged.last_mut() {
            if s <= last.1 + 1 {
                if e > last.1 {
                    last.1 = e;
                }
                continue;
            }
        }
        merged.push((s, e));
    }

    let total: u64 = merged.iter().map(|(s, e)| e - s + 1).sum();
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }
}

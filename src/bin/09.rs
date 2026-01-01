advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<u64> {
    let tiles: Vec<(i64, i64)> = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|line| {
            let mut parts = line.split(',').map(|s| s.trim().parse::<i64>().unwrap());
            (parts.next().unwrap(), parts.next().unwrap())
        })
        .collect();

    tiles
        .iter()
        .enumerate()
        .flat_map(|(i, a)| tiles[i + 1..].iter().map(move |b| (a, b)))
        .map(|(a, b)| {
            let width = a.0.abs_diff(b.0) + 1;
            let height = a.1.abs_diff(b.1) + 1;
            width * height
        })
        .max()
}

pub fn part_two(input: &str) -> Option<u64> {
    let tiles: Vec<(i64, i64)> = input
        .lines()
        .filter(|l| !l.is_empty())
        .map(|line| {
            let mut parts = line.split(',').map(|s| s.trim().parse::<i64>().unwrap());
            (parts.next().unwrap(), parts.next().unwrap())
        })
        .collect();

    let n = tiles.len();
    if n < 2 { return None; }

    let lines: Vec<((i64, i64), (i64, i64))> = (0..n)
        .map(|i| (tiles[i], tiles[(i + 1) % n]))
        .collect();

    let mut candidates = Vec::new();
    for i in 0..n {
        for j in i + 1..n {
            let a = tiles[i];
            let b = tiles[j];
            let area = (a.0.abs_diff(b.0) + 1) * (a.1.abs_diff(b.1) + 1);
            candidates.push((a, b, area));
        }
    }

    candidates.sort_unstable_by_key(|v| std::cmp::Reverse(v.2));

    for (a, b, area) in candidates {
        let min_x = a.0.min(b.0);
        let max_x = a.0.max(b.0);
        let min_y = a.1.min(b.1);
        let max_y = a.1.max(b.1);

        let safe = lines.iter().all(|&(p1, p2)| {
            let l_min_x = p1.0.min(p2.0);
            let l_max_x = p1.0.max(p2.0);
            let l_min_y = p1.1.min(p2.1);
            let l_max_y = p1.1.max(p2.1);

            max_x <= l_min_x || min_x >= l_max_x || max_y <= l_min_y || min_y >= l_max_y
        });

        if safe {
            return Some(area);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(50));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(24));
    }
}

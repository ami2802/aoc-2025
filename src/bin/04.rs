advent_of_code::solution!(4);

fn build_grid(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(|l| l.as_bytes().to_vec()).collect()
}

fn remove_rolls(grid: &mut [Vec<u8>]) -> usize {
    let directions: [(i32, i32); 8] = [
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1),           (0, 1),
        (1, -1),  (1, 0),  (1, 1),
    ];

    let height = grid.len();
    if height == 0 {
        return 0;
    }

    let mut to_remove: Vec<(usize, usize)> = Vec::new();

    for y in 0..height {
        let width = grid[y].len();
        for x in 0..width {
            if grid[y][x] != b'@' {
                continue;
            }
            let mut cnt = 0;
            for (dx, dy) in directions.iter() {
                let nx_i = x as i32 + dx;
                let ny_i = y as i32 + dy;
                if nx_i < 0 || ny_i < 0 {
                    continue;
                }
                let nx: usize = nx_i as usize;
                let ny: usize = ny_i as usize;
                if ny >= height || nx >= grid[ny].len() {
                    continue;
                }
                if grid[ny][nx] == b'@' {
                    cnt += 1;
                }
            }
            if cnt < 4 {
                to_remove.push((y, x));
            }
        }
    }

    for (y, x) in &to_remove {
        grid[*y][*x] = b'.';
    }

    to_remove.len()
}

pub fn part_one(input: &str) -> Option<u64> {
    // Build mutable grid and remove once
    let mut grid = build_grid(input);
    let removed = remove_rolls(&mut grid);
    Some(removed as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut grid = build_grid(input);
    let mut total_removed: u64 = 0;

    loop {
        let removed = remove_rolls(&mut grid);
        if removed == 0 {
            break;
        }
        total_removed += removed as u64;
    }

    Some(total_removed)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(43));
    }
}
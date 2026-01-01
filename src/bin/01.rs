advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u64> {
    let mut current: i32 = 50;
    let mut result: u64 = 0;
    for line in input.lines() {
        let direction = line.chars().next().unwrap();
        let distance: i32 = line[1..].parse().unwrap();
        if direction == 'L' {
            current -= distance;
        }
        else {
            current += distance;
        }

        current = (current + 100) % 100;
        if current == 0 {
            result += 1;
        }

    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut setting: isize = 50;
    let mut counter: u64 = 0;

    for line in input.lines() {
        let dir = &line[0..1];
        let distance: isize = line[1..].parse().unwrap();
        if dir == "R" {
            counter += ((setting + distance) / 100) as u64;
            setting = (setting + distance) % 100;
        } else {
            let abs_distance = distance;
            counter += ((abs_distance - setting + 99) / 100) as u64;
            setting = (setting - distance).rem_euclid(100);
        }
    }

    Some(counter)
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
        assert_eq!(result, Some(6));
    }
}

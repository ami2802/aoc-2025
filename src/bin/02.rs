advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u64> {
    let mut total = 0;

    for part in input.trim().split(',') {
        let bounds: Vec<u64> = part
            .split('-')
            .map(|s| s.parse::<u64>().unwrap())
            .collect();

        for n in bounds[0]..=bounds[1] {
            let s = n.to_string();
            let chars: Vec<char> = s.chars().collect();
            let mid = chars.len() / 2;
            let first_half: String = chars[..mid].iter().collect();
            let second_half: String = chars[mid..].iter().collect();
            if first_half == second_half {
                total += n;
            }
        }
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut total = 0;

    for part in input.trim().split(',') {
        let bounds: Vec<u64> = part
            .split('-')
            .map(|s| s.parse::<u64>().unwrap())
            .collect();

        for n in bounds[0]..=bounds[1] {
            let s = n.to_string();
            let chars: Vec<char> = s.chars().collect();
            let len = chars.len();
            let mut found = false;

            for chunk_size in 1..len {
                if len % chunk_size != 0 {
                    continue;
                }
                let chunks: Vec<&[char]> = chars.chunks(chunk_size).collect();
                if chunks.windows(2).all(|w| w[0] == w[1]) {
                    found = true;
                    break;
                }
            }
            if found {
                total += n;
            }
        }
    }
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1227775554));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4174379265));
    }
}

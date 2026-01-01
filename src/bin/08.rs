advent_of_code::solution!(8);

use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
struct Box(i64, i64, i64);

fn get_edges(input: &str) -> (Vec<Box>, Vec<(u64, usize, usize)>) {
    let boxes: Vec<Box> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let nums: Vec<i64> = line
                .split(',')
                .map(|num| num.trim().parse().expect("invalid num"))
                .collect();
            Box(nums[0], nums[1], nums[2])
        })
        .collect();

    let mut edges = Vec::new();
    for i in 0..boxes.len() {
        for j in i + 1..boxes.len() {
            let b1 = boxes[i];
            let b2 = boxes[j];
            let dist = b1.0.abs_diff(b2.0).pow(2)
                + b1.1.abs_diff(b2.1).pow(2)
                + b1.2.abs_diff(b2.2).pow(2);
            edges.push((dist, i, j));
        }
    }
    edges.sort_unstable_by_key(|e| e.0);
    (boxes, edges)
}

fn find(i: usize, parent: &mut Vec<usize>) -> usize {
    if parent[i] == i {
        i
    } else {
        parent[i] = find(parent[i], parent);
        parent[i]
    }
}

pub fn part_one(input: &str) -> Option<u64> {
    let (boxes, edges) = get_edges(input);
    let n = boxes.len();
    if n == 0 { return None; }

    let mut parent: Vec<usize> = (0..n).collect();
    let limit = if n == 20 { 10 } else { 1000 };

    for t in 0..limit.min(edges.len()) {
        let (_, i, j) = edges[t];
        let root_i = find(i, &mut parent);
        let root_j = find(j, &mut parent);
        if root_i != root_j {
            parent[root_i] = root_j;
        }
    }

    let mut set_sizes: HashMap<usize, u64> = HashMap::new();
    for i in 0..n {
        let root = find(i, &mut parent);
        *set_sizes.entry(root).or_insert(0) += 1;
    }

    let mut sizes: Vec<u64> = set_sizes.values().cloned().collect();
    sizes.sort_unstable_by(|a, b| b.cmp(a));

    Some(sizes.iter().take(3).product())
}

pub fn part_two(input: &str) -> Option<u64> {
    let (boxes, edges) = get_edges(input);
    let n = boxes.len();
    if n == 0 { return None; }

    let mut parent: Vec<usize> = (0..n).collect();
    let mut last_edge = (0, 0);

    for (_, i, j) in edges {
        let root_i = find(i, &mut parent);
        let root_j = find(j, &mut parent);
        if root_i != root_j {
            parent[root_i] = root_j;
            last_edge = (i, j);
        }
    }

    Some((boxes[last_edge.0].0 * boxes[last_edge.1].0) as u64)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(40));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(25272));
    }
}

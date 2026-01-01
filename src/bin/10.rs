advent_of_code::solution!(10);

use std::collections::VecDeque;

const MAX_BUTTONS: usize = 14;
const MAX_JOLTAGES: usize = 15;

type Column = [i32; MAX_JOLTAGES];

pub struct Machine {
    lights: u32,
    buttons: Vec<u32>,
    joltages: Vec<i32>,
}

struct Subspace {
    rank: usize,
    lcm: i32,
    rhs: Column,
    basis: Vec<Basis>,
}

#[derive(Clone, Copy)]
struct Basis {
    vs: Column,
}

fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 { a %= b; std::mem::swap(&mut a, &mut b); }
    a.abs()
}

fn get_lcm(a: i32, b: i32) -> i32 {
    if a == 0 || b == 0 { 0 } else { (a * b).abs() / gcd(a, b) }
}

fn parse_machine(line: &str) -> Machine {
    let tokens: Vec<_> = line.split_ascii_whitespace().collect();
    let last = tokens.len() - 1;
    let lights = tokens[0][1..tokens[0].len()-1]
        .bytes().enumerate()
        .fold(0, |l, (i, b)| l | (u32::from(b == b'#') << i));
    let buttons = tokens[1..last].iter().map(|t| {
        t.trim_matches(|c| c == '(' || c == ')')
            .split(',').filter(|s| !s.is_empty())
            .map(|s| s.parse::<u32>().unwrap())
            .fold(0, |b, i| b | (1 << i))
    }).collect();
    let joltages = tokens[last].trim_matches(|c| c == '{' || c == '}')
        .split(',').filter(|s| !s.is_empty())
        .map(|s| s.parse::<i32>().unwrap()).collect();
    Machine { lights, buttons, joltages }
}

pub fn part_one(input: &str) -> Option<u32> {
    let machines: Vec<Machine> = input.lines().map(parse_machine).collect();
    let mut total = 0;

    for machine in machines {
        let mut queue = VecDeque::new();
        queue.push_back((0u32, 0u32, machine.buttons.len())); // (pattern, pressed, limit)
        let mut visited = std::collections::HashSet::new();
        visited.insert(0u32);

        while let Some((pattern, pressed, limit)) = queue.pop_front() {
            if pattern == machine.lights {
                total += pressed;
                break;
            }
            for i in 0..limit {
                let next = pattern ^ machine.buttons[i];
                if !visited.contains(&next) {
                    visited.insert(next);
                    queue.push_back((next, pressed + 1, i));
                }
            }
        }
    }
    Some(total)
}

fn gaussian_elimination(machine: &Machine) -> Subspace {
    let width = machine.buttons.len();
    let height = machine.joltages.len();
    let mut eq = [[0; MAX_BUTTONS + 1]; MAX_JOLTAGES];

    for row in 0..height { eq[row][width] = machine.joltages[row]; }
    for col in 0..width {
        for row in 0..height {
            if (machine.buttons[col] >> row) & 1 == 1 { eq[row][col] = 1; }
        }
    }

    let mut rank = 0;
    let mut pivot_cols = Vec::new();
    for col in 0..width {
        if let Some(f) = (rank..height).find(|&r| eq[r][col] != 0) {
            eq.swap(rank, f);
            pivot_cols.push(col);
            
            for row in 0..height {
                if row != rank && eq[row][col] != 0 {
                    let p = eq[rank][col];
                    let coeff = eq[row][col];
                    let l = get_lcm(coeff.abs(), p.abs());
                    let x = l / coeff.abs();
                    let y = (l / p.abs()) * coeff.signum() * p.signum();
                    for c in col..=width { 
                        eq[row][c] = x * eq[row][c] - y * eq[rank][c]; 
                    }
                }
            }
            rank += 1;
        }
    }

    let l_val = (0..rank).fold(1, |acc, i| get_lcm(acc, eq[i][pivot_cols[i]]));
    let mut rhs = [0; MAX_JOLTAGES];
    for i in 0..rank {
        rhs[i] = eq[i][width] * (l_val / eq[i][pivot_cols[i]]);
    }

    let mut basis = Vec::new();
    for col in 0..width {
        if !pivot_cols.contains(&col) {
            let mut vs = [0; MAX_JOLTAGES];
            for i in 0..rank {
                vs[i] = eq[i][col] * (l_val / eq[i][pivot_cols[i]]);
            }
            basis.push(Basis { vs });
        }
    }

    Subspace { rank, lcm: l_val, rhs, basis }
}

fn solve_recursive(sub: &Subspace, rhs: Column, idx: usize, free_presses: i32) -> Option<i32> {
    if idx == sub.basis.len() {
        let mut pivot_total = 0;
        for r in 0..sub.rank {
            if rhs[r] < 0 || rhs[r] % sub.lcm != 0 { return None; }
            pivot_total += rhs[r] / sub.lcm;
        }
        return Some(free_presses + pivot_total);
    }

    let mut best: Option<i32> = None;
    
    // Standard AOC joltage puzzles don't require massive press counts.
    // 0 to 200 covers the vast majority of search spaces for these linear systems.
    for n in 0..=200 {
        let mut next_rhs = rhs;
        let mut possible = true;
        for r in 0..sub.rank {
            next_rhs[r] -= n * sub.basis[idx].vs[r];
            // Pruning: If vs[r] is positive and rhs is already negative, 
            // increasing n will only make it worse.
            if sub.basis[idx].vs[r] > 0 && next_rhs[r] < 0 {
                possible = false;
                break;
            }
        }
        
        if possible {
            if let Some(res) = solve_recursive(sub, next_rhs, idx + 1, free_presses + n) {
                if best.is_none() || res < best.unwrap() {
                    best = Some(res);
                }
            }
        } else {
            break; 
        }
    }
    best
}

pub fn part_two(input: &str) -> Option<i32> {
    let machines: Vec<Machine> = input.lines().map(parse_machine).collect();
    let mut total = 0;
    for m in machines {
        let sub = gaussian_elimination(&m);
        total += solve_recursive(&sub, sub.rhs, 0, 0).unwrap_or(0);
    }
    Some(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(33));
    }
}

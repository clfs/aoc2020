use std::collections::HashSet;
use std::io::{self, Read};

fn main() -> Result<(), io::Error> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let tape = parse(&input);
    let p1 = part1(&tape, 25).unwrap();
    let p2 = part2(&tape, p1).unwrap();

    println!("part 1: {}", p1);
    println!("part 2: {}", p2);

    Ok(())
}

fn parse(input: &str) -> Vec<u64> {
    input.lines().filter_map(|x| x.parse().ok()).collect()
}

fn part1(tape: &Vec<u64>, preamble_len: usize) -> Option<u64> {
    let mut seen = HashSet::new();
    let mut possible = HashSet::new();

    let (preamble, tail) = tape.split_at(preamble_len);

    // There's probably a cleaner way to insert than this.
    for x in preamble {
        seen.insert(*x);
        for y in preamble {
            possible.insert(*x + *y);
        }
    }

    // O(n^2) baby!!
    for t in tail {
        if !possible.contains(t) {
            return Some(*t);
        }
        for s in &seen {
            possible.insert(*t + s);
        }
        seen.insert(*t);
    }

    None
}

fn part2(tape: &Vec<u64>, target: u64) -> Option<u64> {
    for lo in 0..tape.len() - 1 {
        let mut sum = tape[lo];
        for hi in lo + 1..tape.len() {
            sum += tape[hi];
            if sum == target {
                return compute(tape.get(lo..=hi)?);
            } else if sum > target {
                break;
            }
        }
    }
    None
}

fn compute(s: &[u64]) -> Option<u64> {
    Some(s.iter().min()? + s.iter().max()?)
}

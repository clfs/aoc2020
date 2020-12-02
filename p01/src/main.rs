use std::collections::HashSet;
use std::io::{self, Read};

fn main() -> Result<(), io::Error> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let nums = parse(&input);

    println!("part 1: {:?}", part1(&nums));

    Ok(())
}

fn parse(input: &str) -> Vec<i32> {
    return input
        .split_whitespace()
        .filter_map(|x| x.parse().ok())
        .collect();
}

fn part1(nums: &Vec<i32>) -> Option<i32> {
    let mut seen = HashSet::new();
    for n in nums {
        if seen.contains(&(2020 - n)) {
            return Some(n * (2020 - n));
        }
        seen.insert(n);
    }
    None
}

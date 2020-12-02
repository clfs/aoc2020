use std::collections::HashSet;
use std::io::{self, Read};

use num::bigint::{BigInt, ToBigInt};

fn main() -> Result<(), io::Error> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let nums = parse(&input);

    println!("part 1: {}", part1(&nums).unwrap());
    println!("part 2: {}", part2(&nums).unwrap());

    Ok(())
}

fn parse(input: &str) -> Vec<BigInt> {
    return input
        .split_whitespace()
        .filter_map(|x| x.parse().ok())
        .collect();
}

fn part1(nums: &Vec<BigInt>) -> Option<BigInt> {
    let mut seen = HashSet::new();
    for n in nums {
        if seen.contains(&(2020 - n)) {
            return Some(n * (2020 - n));
        }
        seen.insert(n);
    }
    None
}

fn part2(nums: &Vec<BigInt>) -> Option<BigInt> {
    let mut v = nums.clone();
    v.sort_unstable();

    // The `num` create claims that `to_bigint` will always succeed when
    // converting from any integer or unsigned primitive.
    let target = 2020.to_bigint().unwrap();

    let len = v.len();
    for i in 0..len - 2 {
        let a = &v[i];
        let (mut start, mut end) = (i + 1, len - 1);
        while start < end {
            let (b, c) = (&v[start], &v[end]);
            let sum = a + b + c;

            // Matching on half-open ranges is unstable right now, apparently.
            // I'm also not sure if it'd work on BigInt regardless.
            if sum < target {
                start += 1;
            } else if sum == target {
                return Some(a * b * c);
            } else {
                end -= 1;
            }
        }
    }
    None
}

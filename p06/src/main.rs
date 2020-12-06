use std::collections::{HashMap, HashSet};
use std::io::{self, Read};

fn main() -> Result<(), io::Error> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    println!("part 1: {:?}", part1(&input));
    println!("part 2: {:?}", part2(&input));

    Ok(())
}

fn part1(input: &str) -> usize {
    let groups = input.split("\n\n").collect::<Vec<&str>>();
    let mut count: usize = 0;

    for g in groups {
        let mut h: HashSet<&u8> = g.as_bytes().into_iter().collect();
        h.remove(&10); // \n
        count += h.len();
    }

    count
}

fn part2(input: &str) -> usize {
    let groups = input.split("\n\n").collect::<Vec<&str>>();
    let mut count: usize = 0;

    for g in groups {
        let mut h = HashMap::new();
        let n_ppl = g.lines().count();

        for ch in g.chars() {
            *h.entry(ch).or_insert(0) += 1;
        }
        for (k, v) in &h {
            if *v == n_ppl && *k != '\n' {
                count += 1;
            }
        }
    }
    count
}

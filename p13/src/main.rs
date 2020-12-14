use std::io::{self, Read};

fn main() -> Result<(), io::Error> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let (ts, buses) = parse(&input).unwrap();

    println!("part 1: {:?}", part1(ts, &buses));

    Ok(())
}

fn parse(input: &str) -> Option<(u64, Vec<u64>)> {
    let mut lines = input.lines();
    let ts = lines.next()?.parse().unwrap();
    let buses = lines
        .next()?
        .split(",")
        .filter_map(|x| x.parse().ok())
        .collect();
    Some((ts, buses))
}

fn part1(ts: u64, buses: &[u64]) -> Option<u64> {
    let mut cur = ts;
    loop {
        for b in buses {
            if cur % *b == 0 {
                return Some(b * (cur - ts));
            }
        }
        cur += 1;
    }
}

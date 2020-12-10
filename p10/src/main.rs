use std::io::{self, Read};

fn main() -> Result<(), io::Error> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let chargers = parse(&input);

    println!("part 1: {:?}", part1(&chargers));

    Ok(())
}

fn parse(input: &str) -> Vec<u32> {
    let mut x: Vec<u32> = input.lines().filter_map(|x| x.parse().ok()).collect();
    x.sort_unstable();
    x
}

fn part1(chargers: &[u32]) -> Option<u32> {
    let vals = chargers.iter();
    let next_vals = chargers.iter().skip(1);

    let diffs: Vec<u32> = vals.zip(next_vals).map(|(cur, next)| next - cur).collect();

    let (mut ones, mut threes) = (0, 0);
    for d in diffs {
        match d {
            1 => ones += 1,
            3 => threes += 1,
            _ => (),
        }
    }

    // Adjust for the first and last hop.
    threes += 1;
    match chargers.first()? {
        1 => ones += 1,
        3 => threes += 1,
        _ => (),
    }

    Some(ones * threes)
}

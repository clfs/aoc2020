use std::collections::HashSet;
use std::error::Error;
use std::io::{self, Read, Write};

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;

    Ok(())
}

fn part1(input: &str) -> Result<(), Box<dyn Error>> {
    let mut seen = HashSet::new();
    for line in input.lines() {
        let n: i32 = line.parse()?;
        if seen.contains(&(2020 - n)) {
            writeln!(io::stdout(), "{}", n * (2020 - n))?;
            return Ok(());
        }
        seen.insert(n);
    }
    Ok(())
}

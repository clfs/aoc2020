use std::io::{self, Read};

use regex::Regex;

struct Entry {
    policy: Policy,
    password: String,
}

struct Policy {
    letter: String,
    min: u32,
    max: u32,
}

impl Entry {
    fn is_valid(&self) -> bool {
        let count = self.password.matches(&self.policy.letter).count();
        return (self.policy.min as usize <= count) && (count <= self.policy.max as usize);
    }
}

fn main() -> Result<(), io::Error> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let entries = parse(&input);

    println!("part 1: {}", part1(entries));

    Ok(())
}

fn parse(input: &str) -> Vec<Entry> {
    return input.lines().filter_map(|x| parse_line(x)).collect();
}

fn parse_line(line: &str) -> Option<Entry> {
    // 10-15 w: wglmwwwrnnzgwhhwvvd
    // min: 10
    // max: 15
    // letter: w
    // password: wglmwwwrnnzgwhhwvvd
    let re = Regex::new(
        r"(?P<min>\d{1,2})-(?P<max>\d{1,2})[ ](?P<letter>[a-z]):[ ](?P<password>[a-z]+)",
    )
    .unwrap();
    let caps = re.captures(line)?;

    return Some(Entry {
        policy: Policy {
            letter: caps["letter"].to_string(),
            min: caps["min"].parse().unwrap(),
            max: caps["max"].parse().unwrap(),
        },
        password: caps["password"].to_string(),
    });
}

fn part1(entries: Vec<Entry>) -> usize {
    entries.into_iter().filter(|x| x.is_valid()).count()
}

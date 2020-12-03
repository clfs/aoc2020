use std::io::{self, Read};

use regex::Regex;

struct Entry {
    policy: Policy,
    password: String,
}

struct Policy {
    letter: String,
    low: u32,
    high: u32,
}

impl Entry {
    fn is_valid(&self) -> bool {
        let count = self.password.matches(&self.policy.letter).count();
        return (self.policy.low as usize <= count) && (count <= self.policy.high as usize);
    }

    fn is_valid_strict(&self) -> bool {
        let left = self
            .password
            .chars()
            .nth((self.policy.low - 1) as usize)
            .unwrap()
            .to_string();
        let right = self
            .password
            .chars()
            .nth((self.policy.high - 1) as usize)
            .unwrap()
            .to_string();
        return (left == self.policy.letter) ^ (right == self.policy.letter);
    }
}

fn main() -> Result<(), io::Error> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let entries = parse(&input);

    println!("part 1: {}", part1(&entries));
    println!("part 2: {}", part2(&entries));

    Ok(())
}

fn parse(input: &str) -> Vec<Entry> {
    return input.lines().filter_map(|x| parse_line(x)).collect();
}

fn parse_line(line: &str) -> Option<Entry> {
    // 10-15 w: wglmwwwrnnzgwhhwvvd
    // low: 10
    // high: 15
    // letter: w
    // password: wglmwwwrnnzgwhhwvvd
    let re = Regex::new(
        r"(?P<low>\d{1,2})-(?P<high>\d{1,2})[ ](?P<letter>[a-z]):[ ](?P<password>[a-z]+)",
    )
    .unwrap();
    let caps = re.captures(line)?;

    return Some(Entry {
        policy: Policy {
            letter: caps["letter"].to_string(),
            low: caps["low"].parse().unwrap(),
            high: caps["high"].parse().unwrap(),
        },
        password: caps["password"].to_string(),
    });
}

fn part1(entries: &Vec<Entry>) -> usize {
    entries.into_iter().filter(|x| x.is_valid()).count()
}

fn part2(entries: &Vec<Entry>) -> usize {
    entries.into_iter().filter(|x| x.is_valid_strict()).count()
}

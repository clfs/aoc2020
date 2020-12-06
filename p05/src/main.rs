use std::str::FromStr;
use std::{
    io::{self, Read},
    num::ParseIntError,
};

fn main() -> Result<(), io::Error> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    println!("part 1: {:?}", part1(&input));
    println!("part 2: {:?}", part2(&input));

    Ok(())
}

struct Seat {
    row: u32,
    col: u32,
}

impl Seat {
    fn id(&self) -> u32 {
        self.row * 8 + self.col
    }
}

impl FromStr for Seat {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let b = s
            .replace("F", "0")
            .replace("B", "1")
            .replace("R", "1")
            .replace("L", "0");

        let (fb, rl) = b.split_at(7);
        let result = Seat {
            row: u32::from_str_radix(fb, 2)?,
            col: u32::from_str_radix(rl, 2)?,
        };
        Ok(result)
    }
}

fn part1(input: &str) -> Option<u32> {
    input
        .lines()
        .filter_map(|x| x.parse::<Seat>().ok())
        .map(|x| x.id())
        .max()
}

fn part2(input: &str) -> Option<u32> {
    let mut ids = input
        .lines()
        .filter_map(|x| x.parse::<Seat>().ok())
        .map(|x| x.id())
        .collect::<Vec<u32>>();
    ids.sort_unstable();

    let (min, max) = (ids.first()?, ids.last()?);

    let actual_sum: u32 = ids.iter().sum();
    let target_sum: u32 = ((max - min + 1) * (min + max)) / 2;

    Some(target_sum - actual_sum)
}

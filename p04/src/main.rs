use lazy_static::lazy_static;
use regex::Regex;
use std::default::Default;
use std::io::{self, Read};
use std::str::FromStr;

lazy_static! {
    static ref KEY_VAL_RE: Regex = Regex::new(r"^(?P<key>.+?):(?P<val>.+)").unwrap();
    static ref HEIGHT_RE: Regex = Regex::new(r"^(?P<magnitude>\d+)(?P<unit>cm|in)$").unwrap();
    static ref HAIR_COLOR_RE: Regex = Regex::new(r"^#[a-f0-9]{6}$").unwrap();
    static ref EYE_COLOR_RE: Regex = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
    static ref PASSPORT_ID_RE: Regex = Regex::new(r"^\d{9}$").unwrap();
}

#[derive(Default, Debug)]
struct Passport {
    birth_year: Option<u32>,
    issue_year: Option<u32>,
    expiry_year: Option<u32>,
    height: Option<Height>,
    hair_color: Option<String>,
    eye_color: Option<String>,
    passport_id: Option<String>,
}

#[derive(Debug)]
enum Height {
    Cm(u32),
    In(u32),
}

impl FromStr for Height {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let captures = HEIGHT_RE
            .captures(s)
            .ok_or_else(|| format!("invalid height {}", s))?;
        let magnitude = captures
            .name("magnitude")
            .ok_or_else(|| "no magnitude found")?
            .as_str()
            .parse::<u32>()
            .unwrap(); // lol
        let unit = captures
            .name("unit")
            .ok_or_else(|| "no unit found")?
            .as_str();
        match unit {
            "cm" => Ok(Height::Cm(magnitude)),
            "in" => Ok(Height::In(magnitude)),
            _ => Err(format!("invalid unit {}", unit)),
        }
    }
}

impl FromStr for Passport {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut passport = Passport::default();
        for field in s.split_whitespace() {
            let captures = KEY_VAL_RE
                .captures(field)
                .ok_or_else(|| format!("invalid field {}", field))?;
            let key = captures.name("key").ok_or_else(|| "no key found")?.as_str();
            let val = captures.name("val").ok_or_else(|| "no val found")?.as_str();
            match key {
                "byr" => passport.birth_year = val.parse().ok(),
                "iyr" => passport.issue_year = val.parse().ok(),
                "eyr" => passport.expiry_year = val.parse().ok(),
                "hgt" => passport.height = val.parse().ok(),
                "hcl" => passport.hair_color = Some(val.to_string()),
                "ecl" => passport.eye_color = Some(val.to_string()),
                "pid" => passport.passport_id = Some(val.to_string()),
                _ => (),
            }
        }
        Ok(passport)
    }
}

impl Passport {
    fn is_valid_inner(&self) -> Option<bool> {
        let valid = (1920..=2002).contains(&self.birth_year?)
            && (2010..=2020).contains(&self.issue_year?)
            && (2020..=2030).contains(&self.expiry_year?)
            && match &self.height.as_ref()? {
                Height::Cm(v) => (150..=193).contains(v),
                Height::In(v) => (59..=76).contains(v),
            }
            && HAIR_COLOR_RE.is_match(&self.hair_color.as_ref()?)
            && EYE_COLOR_RE.is_match(&self.eye_color.as_ref()?)
            && PASSPORT_ID_RE.is_match(&self.passport_id.as_ref()?);
        Some(valid)
    }

    fn is_valid(&self) -> bool {
        match self.is_valid_inner() {
            Some(v) => v,
            None => false,
        }
    }
}

fn main() -> Result<(), io::Error> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let passports = parse(&input);

    println!("part 1: {}", part1(&input));
    println!("part 2: {}", part2(&passports));

    Ok(())
}

fn parse(input: &str) -> Vec<Passport> {
    return input.split("\n\n").filter_map(|x| x.parse().ok()).collect();
}

fn part1(input: &str) -> i32 {
    let mut n = 0;
    let required = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    for p in input.split("\n\n") {
        let mut x = 0;
        for r in &required {
            if p.contains(r) {
                x += 1;
            }
        }
        if x == required.len() {
            n += 1;
        }
    }
    n
}

fn part2(passports: &Vec<Passport>) -> usize {
    return passports.iter().filter(|x| x.is_valid()).count();
}

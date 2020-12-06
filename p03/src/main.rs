use std::io::{self, Read};

struct Geology {
    field: Vec<bool>,
    w: usize,
}

impl Geology {
    fn get(&self, x: usize, y: usize) -> Option<&bool> {
        return self.field.get((self.w * y) + (x % self.w));
    }
}

fn main() -> Result<(), io::Error> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let geology = parse(&input).unwrap();

    println!("part 1: {:?}", part1(&geology, 3, 1));
    println!("part 2: {:?}", part2(&geology));

    Ok(())
}

fn parse(input: &str) -> Option<Geology> {
    let w = input.lines().next()?.len();
    let mut field = Vec::new();

    for c in input.as_bytes() {
        match c {
            b'#' => field.push(true),
            b'.' => field.push(false),
            _ => (),
        }
    }

    Some(Geology { field: field, w: w })
}

fn part1(g: &Geology, delta_x: usize, delta_y: usize) -> i64 {
    let mut n_trees = 0;
    let (mut x, mut y) = (0, 0);
    while let Some(is_tree) = g.get(x, y) {
        match is_tree {
            true => n_trees += 1,
            false => (),
        }
        x += delta_x;
        y += delta_y;
    }
    n_trees
}

fn part2(g: &Geology) -> i64 {
    part1(&g, 1, 1) * part1(&g, 3, 1) * part1(&g, 5, 1) * part1(&g, 7, 1) * part1(&g, 1, 2)
}

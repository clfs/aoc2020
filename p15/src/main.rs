use std::collections::HashMap;

fn main() {
    let input = vec![16, 12, 1, 0, 15, 7, 11];
    println!("part 1: {:?}", part1(&input, 2020));
    println!("part 2: {:?}", part1(&input, 30000000));
}

// Part 1 worked for Part 2 without any modifications!
fn part1(input: &[u32], target: usize) -> Option<usize> {
    // This maps a number to a turn it was said.
    let mut h = HashMap::new();

    // This stores the return value from the most recent `.insert`.
    let mut prev = None;

    // Load the input. Remember, the game is 1-indexed.
    for (i, n) in input.iter().enumerate() {
        prev = h.insert(*n as usize, i + 1);
    }

    // Now, run the game, but stop before the last step.
    for turn in input.len() + 1..target {
        prev = match prev {
            // "If that was the first time the number has been spoken,
            // the current player says 0."
            None => h.insert(0, turn),
            // "Otherwise, the number had been spoken before; the current
            // player announces how many turns apart the number is from
            // when it was previously spoken."
            Some(v) => h.insert((turn - 1) - v, turn),
        };
    }

    // Run the last step.
    match prev {
        None => Some(0),
        Some(v) => Some((target - 1) - v),
    }
}

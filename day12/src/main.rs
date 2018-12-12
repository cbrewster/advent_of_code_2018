use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::error::Error;

type PotId = i32;
type Patterns = HashMap<String, bool>;
type Pots = HashSet<PotId>;

fn main() -> Result<(), Box<Error>> {
    let input = include_str!("../input.txt").trim();
    let initial_state = parse_initial_state(&input)?;
    let patterns = parse_patterns(&input)?;

    if patterns.len() != 32 {
        return Err("32 patterns mut be provided!".into());
    }

    // Part 1
    run(&initial_state, &patterns, 20);
    // Part 2
    run(&initial_state, &patterns, 50);
    run(&initial_state, &patterns, 500);
    run(&initial_state, &patterns, 5000);
    run(&initial_state, &patterns, 50000);
    // Running with 50000000000 generations will take forever,
    // had to recognize the pattern with smaller inputs.
    // Result: 2250000000120

    Ok(())
}

fn run(initial_state: &Pots, patterns: &Patterns, generations: usize) {
    let mut current_generation: Pots = initial_state.clone();

    for _ in 0..generations {
        let min = current_generation.iter().min().cloned().unwrap_or(0) - 5;
        let max = current_generation.iter().max().cloned().unwrap_or(0) + 5;
        current_generation = (min..=max)
            .filter_map(|id| {
                if patterns.get(&get_pattern(&current_generation, id)).cloned().unwrap_or(false) {
                    Some(id)
                } else {
                    None
                }
            }).collect();
    }

    let sum: i32 = current_generation.iter().sum();
    println!("The sum of the pot numbers with plants after {} generations is {}", generations, sum);
}

fn get_pattern(pots: &Pots, id: PotId) -> String {
    let mut pattern = String::new();
    for curr in id-2..=id+2 {
        if pots.contains(&curr) {
            pattern.push('#');
        } else {
            pattern.push('.');
        }
    }
    pattern
}

fn parse_initial_state(input: &str) -> Result<Pots, Box<Error>> {
    let re = Regex::new(r"initial state: (?P<state>[.#]+)")?;

    let captures = match re.captures(input) {
        Some(captures) => captures,
        None => return Err("Could not parse initial state.".into()),
    };

    Ok(captures["state"]
        .chars()
        .enumerate()
        .filter(|(_, c)| *c == '#')
        .map(|(id, _)| id as i32)
        .collect())
}

fn parse_patterns(input: &str) -> Result<Patterns, Box<Error>> {
    let re = Regex::new(r"(?P<pattern>[#.]{5}) => (?P<result>[#.])")?;

    Ok(input.split("\n")
        .filter_map(|line| re.captures(line))
        .map(|captures| {
            let result = captures["result"] == *"#";
            (captures["pattern"].into(), result)
        })
        .collect())
}

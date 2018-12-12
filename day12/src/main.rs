use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::error::Error;

type PotId = i64;
type Patterns = HashMap<String, bool>;
type Pots = HashSet<PotId>;

fn main() -> Result<(), Box<Error>> {
    let input = include_str!("../input.txt").trim();
    let initial_state = parse_initial_state(&input)?;
    let patterns = parse_patterns(&input)?;

    if patterns.len() != 32 {
        return Err("32 patterns mut be provided!".into());
    }

    part1(&initial_state, &patterns);
    part2(&initial_state, &patterns);

    Ok(())
}

fn part1(initial_state: &Pots, patterns: &Patterns) {
    let sum = compute_final(initial_state, patterns, 20);
    println!(
        "The sum of the pot numbers with plants after 20 generations is {}",
        sum
    );
}

fn part2(initial_state: &Pots, patterns: &Patterns) {
    let sum = compute_final(initial_state, patterns, 50000000000);
    println!(
        "The sum of the pot numbers with plants after 50000000000 generations is {}",
        sum
    );
}

fn compute_final(initial_state: &Pots, patterns: &Patterns, generations: usize) -> i64 {
    let mut current_generation: Pots = initial_state.clone();
    let mut last_score = get_score(&current_generation);
    let mut last_diff = 0;
    let mut same_diff = 0;

    for generation in 0..generations {
        let min = current_generation.iter().min().cloned().unwrap_or(0) - 5;
        let max = current_generation.iter().max().cloned().unwrap_or(0) + 5;
        current_generation = (min..=max)
            .filter_map(|id| {
                if patterns
                    .get(&get_pattern(&current_generation, id))
                    .cloned()
                    .unwrap_or(false)
                {
                    Some(id)
                } else {
                    None
                }
            })
            .collect();

        let score = get_score(&current_generation);
        let diff = score - last_score;

        if diff == last_diff {
            same_diff += 1;
        } else {
            same_diff = 0;
        }

        if same_diff > 5 {
            return score + diff * (generations - generation + 1) as i64;
        }

        last_score = score;
        last_diff = diff;
    }
    last_score
}

fn get_score(pots: &Pots) -> i64 {
    pots.iter().sum()
}

fn get_pattern(pots: &Pots, id: PotId) -> String {
    let mut pattern = String::new();
    for curr in id - 2..=id + 2 {
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
        .map(|(id, _)| id as i64)
        .collect())
}

fn parse_patterns(input: &str) -> Result<Patterns, Box<Error>> {
    let re = Regex::new(r"(?P<pattern>[#.]{5}) => (?P<result>[#.])")?;

    Ok(input
        .split("\n")
        .filter_map(|line| re.captures(line))
        .map(|captures| {
            let result = captures["result"] == *"#";
            (captures["pattern"].into(), result)
        })
        .collect())
}

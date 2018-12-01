use std::io::prelude::*;

use std::collections::HashSet;
use std::fs::File;
use std::io;

fn main() -> Result<(), io::Error> {
    let mut input_file = File::open("input.txt")?;
    let mut input = String::new();
    input_file.read_to_string(&mut input)?;

    let part_one_result = part_one(&input);
    println!("The frequency of part one is {}", part_one_result);

    let part_two_result = part_two(&input);
    println!("The frequency of part two is {}", part_two_result);

    Ok(())
}

fn part_one(input: &str) -> isize {
    input
        .split_whitespace()
        .filter_map(|freq_change| freq_change.parse::<isize>().ok())
        .sum()
}

fn part_two(input: &str) -> isize {
    let mut seen_frequencies = HashSet::new();

    let freq_changes = input
        .split_whitespace()
        .filter_map(|freq_change| freq_change.parse::<isize>().ok())
        .cycle();

    let mut current_freq = 0isize;
    seen_frequencies.insert(current_freq);
    for freq_change in freq_changes {
        current_freq += freq_change;
        if !seen_frequencies.insert(current_freq) {
            break;
        }
    }

    current_freq
}

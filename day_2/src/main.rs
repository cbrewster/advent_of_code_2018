use std::collections::HashMap;

fn main() {
    let input = include_str!("../input.txt");

    let checksum = compute_checksum(&input);

    println!("The checksum is {}.", checksum);

    let common_letters = find_common_letters(&input);

    println!(
        "The correct ID have these letters in common {}.",
        common_letters
    );
}

fn compute_checksum(input: &str) -> usize {
    let (twos, threes) = input
        .split_whitespace()
        .map(compute_letter_freq)
        .fold((0, 0), |acc, (twos, threes)| (acc.0 + twos, acc.1 + threes));

    twos * threes
}

fn compute_letter_freq(input: &str) -> (usize, usize) {
    let mut frequencies = HashMap::new();
    for c in input.chars() {
        frequencies.entry(c).and_modify(|c| *c += 1).or_insert(1);
    }

    let (mut twos, mut threes) = (0, 0);
    for (_, count) in frequencies {
        if count == 3 {
            threes = 1;
        } else if count == 2 {
            twos = 1;
        }
    }

    (twos, threes)
}

fn find_common_letters(input: &str) -> String {
    let ids = input.split_whitespace().collect::<Vec<_>>();

    for (index, id) in ids.iter().enumerate() {
        for other_id in &ids[index + 1..] {
            let mut difference = 0;
            let mut different_char = '\0';
            for (char1, char2) in id.chars().zip(other_id.chars()) {
                if char1 != char2 {
                    difference += 1;
                    if difference > 1 {
                        break;
                    };
                    different_char = char1;
                }
            }
            if difference == 1 {
                return id.to_owned().replace(different_char, "");
            }
        }
    }

    "".into()
}

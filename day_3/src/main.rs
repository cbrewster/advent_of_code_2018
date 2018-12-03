use std::collections::{HashMap, HashSet};

#[derive(Debug, Eq, PartialEq)]
struct Claim {
    id: usize,
    position: (usize, usize),
    size: (usize, usize),
}

impl Claim {
    fn to_sections(&self) -> Vec<(usize, usize)> {
        let mut sections = Vec::new();
        for x_offset in 0..self.size.0 {
            for y_offset in 0..self.size.1 {
                sections.push((self.position.0 + x_offset, self.position.1 + y_offset))
            }
        }
        sections
    }

    fn is_overlapping(&self, other: &Claim) -> bool {
        !(self.position.0 + self.size.0 < other.position.0
            || self.position.0 > other.position.0 + other.size.0
            || self.position.1 + self.size.1 < other.position.1
            || self.position.1 > other.position.1 + other.size.1)
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let claims = parse_claims(&input);
    let overlap = compute_overlap(&claims);
    println!("The overlap is {}", overlap);
    let best_claim = find_best_claim(&claims);
    println!("The best claim is {}", best_claim.unwrap().id);
}

fn compute_overlap(claims: &[Claim]) -> usize {
    let fabric =
        claims
            .iter()
            .flat_map(Claim::to_sections)
            .fold(HashMap::new(), |mut fabric, section| {
                fabric
                    .entry(section)
                    .and_modify(|count| *count += 1)
                    .or_insert(1);
                fabric
            });

    fabric.values().fold(0, |mut acc, count| {
        if *count > 1 {
            acc += 1
        }
        acc
    })
}

fn find_best_claim(claims: &[Claim]) -> Option<&Claim> {
    let mut overlapping = HashSet::new();
    for (index, claim) in claims.iter().enumerate() {
        for other_claim in &claims[index + 1..] {
            if claim.is_overlapping(other_claim) {
                overlapping.insert(claim.id);
                overlapping.insert(other_claim.id);
            }
        }
    }

    claims.iter().find(|claim| !overlapping.contains(&claim.id))
}

fn parse_claims(input: &str) -> Vec<Claim> {
    input
        .split('\n')
        .filter(|c| !c.is_empty())
        .filter_map(|claim| parse_claim(claim))
        .collect()
}

fn parse_claim(input: &str) -> Option<Claim> {
    let (id, rest) = split(input, '@')?;
    let id = id[1..].trim().parse::<usize>().ok()?;

    let (position, size) = split(rest, ':')?;
    let (x, y) = split(position.trim(), ',')?;
    let x = x.trim().parse::<usize>().expect("Failed x");
    let y = y.trim().parse::<usize>().expect("Failed y");

    let (width, height) = split(size.trim(), 'x')?;
    let width = width.trim().parse::<usize>().expect("Failed width");
    let height = height.trim().parse::<usize>().expect("Failed height");

    Some(Claim {
        id,
        position: (x, y),
        size: (width, height),
    })
}

fn split(input: &str, delimeter: char) -> Option<(&str, &str)> {
    let (start, end) = input.split_at(input.find(delimeter)?);
    Some((start, &end[1..]))
}

mod test {
    #[test]
    fn test_parse() {
        use super::{parse_claim, Claim};

        let claim = parse_claim("#1 @ 2,3: 4x5");
        assert_eq!(
            claim,
            Some(Claim {
                id: 1,
                position: (2, 3),
                size: (4, 5)
            })
        )
    }
}

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("../input.txt").trim();
    let license = input
        .split_whitespace()
        .map(|value| value.parse::<u32>())
        .collect::<Result<Vec<_>, _>>()?;

    let metadata_sum = part1(&license);
    println!("The sum of the metadata in the tree is {}.", metadata_sum);

    let root_value = part2(&license);
    println!("The value of the root node is {}.", root_value);

    Ok(())
}

fn part1(license: &[u32]) -> u32 {
    let mut input_stream = license.iter().cloned();
    let head = get_node(&mut input_stream);

    head.tree_metadata_sum()
}

fn part2(license: &[u32]) -> u32 {
    let mut input_stream = license.iter().cloned();
    let head = get_node(&mut input_stream);

    head.value()
}

fn get_node<I>(input: &mut I) -> Node
where
    I: Iterator<Item = u32>,
{
    let child_count = input.next().unwrap();
    let metadata_count = input.next().unwrap();
    let mut node = Node::new(child_count, metadata_count);

    for _ in 0..child_count {
        node.children.push(get_node(input));
    }

    for _ in 0..metadata_count {
        node.metadata.push(input.next().unwrap());
    }

    node
}

#[derive(Debug)]
struct Node {
    child_count: u32,
    metadata_count: u32,
    children: Vec<Node>,
    metadata: Vec<u32>,
}

impl Node {
    fn new(child_count: u32, metadata_count: u32) -> Node {
        Node {
            child_count,
            metadata_count,
            children: Vec::new(),
            metadata: Vec::new(),
        }
    }

    fn metadata_sum(&self) -> u32 {
        self.metadata.iter().sum()
    }

    fn tree_metadata_sum(&self) -> u32 {
        self.metadata_sum()
            + self
                .children
                .iter()
                .map(|child| child.tree_metadata_sum())
                .sum::<u32>()
    }

    fn value(&self) -> u32 {
        if self.child_count == 0 {
            self.metadata_sum()
        } else {
            self.metadata
                .iter()
                .filter(|entry| **entry > 0)
                .filter_map(|entry| self.children.get(*entry as usize - 1))
                .map(|node| node.value())
                .sum()
        }
    }
}

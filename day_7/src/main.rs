use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::error::Error;

type Step = char;

type DependencyTree = HashMap<Step, Vec<Step>>;

fn main() -> Result<(), Box<Error>> {
    let input = include_str!("../input.txt").trim();
    let re = Regex::new(
        r"Step (?P<prereq>[A-Z]) must be finished before step (?P<step>[A-Z]) can begin.",
    )?;

    let steps = input
        .split("\n")
        .map(|line| {
            let caps = match re.captures(line) {
                Some(caps) => caps,
                None => return Err(format!("Unrecognized step: {}.", line)),
            };
            let prereq = caps["prereq"].as_bytes()[0] as Step;
            let step = caps["step"].as_bytes()[0] as Step;

            Ok((prereq, step))
        })
        .collect::<Result<Vec<_>, _>>()?;

    let dependencies: DependencyTree =
        steps
            .iter()
            .fold(HashMap::new(), |mut tree, (prereq, step)| {
                tree.entry(*step).or_default().push(*prereq);
                tree.entry(*prereq).or_default();
                tree
            });

    let order = part1(&dependencies).iter().collect::<String>();
    println!("Order: {:?}", order);

    let time = part2(&dependencies);
    println!("The sleigh will be built in {} seconds!", time);

    Ok(())
}

fn part1(dependencies: &DependencyTree) -> Vec<Step> {
    let mut completed_steps: HashSet<Step> = HashSet::new();
    let mut order = Vec::new();
    loop {
        let next_steps = compute_next_steps(dependencies, &completed_steps, None);
        match next_steps.first() {
            None => break,
            Some(&step) => {
                order.push(step);
                completed_steps.insert(step);
            }
        }
    }
    order
}

fn part2(dependencies: &DependencyTree) -> u32 {
    let mut completed_steps: HashSet<Step> = HashSet::new();
    let mut worker_pool: Vec<Worker> = Vec::new();
    let mut time = 0;
    loop {
        for worker in &mut worker_pool {
            worker.time_remaining -= 1;
            if worker.time_remaining == 0 {
                completed_steps.insert(worker.current_step);
            }
        }
        worker_pool.retain(|worker| worker.time_remaining > 0);

        let next_steps = compute_next_steps(dependencies, &completed_steps, Some(&worker_pool));
        let open_slots = 4 - worker_pool.len();
        let steps = next_steps.iter().take(open_slots);

        for step in steps {
            worker_pool.push(Worker {
                current_step: *step,
                time_remaining: duration(*step),
            });
        }

        if worker_pool.is_empty() {
            break;
        }

        time += 1;
    }

    time
}

fn compute_next_steps(
    dependencies: &DependencyTree,
    completes_steps: &HashSet<Step>,
    workers: Option<&[Worker]>,
) -> Vec<Step> {
    let mut next_steps = dependencies
        .iter()
        .filter(|(step, _)| !completes_steps.contains(step))
        .filter(|(step, _)| {
            workers
                .map(|workers| !workers.iter().any(|worker| worker.current_step == **step))
                .unwrap_or(true)
        })
        .filter(|(_, prereqs)| {
            prereqs
                .iter()
                .map(|prereq| completes_steps.contains(prereq))
                .all(|completed| completed)
        })
        .map(|(step, _)| *step)
        .collect::<Vec<_>>();
    next_steps.sort();
    next_steps
}

struct Worker {
    current_step: Step,
    time_remaining: u32,
}

fn duration(step: Step) -> u32 {
    (60 + step as u8 - 64) as u32
}

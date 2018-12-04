use chrono::prelude::*;
use std::collections::HashMap;

#[derive(Debug)]
struct Entry {
    date_time: NaiveDateTime,
    event: Event,
}

#[derive(Debug)]
enum Event {
    BeginsShift(usize),
    FallsAsleep,
    WakesUp,
}

impl Entry {
    fn new(input: &str) -> Option<Entry> {
        let mut split = input.split(']');
        let date_time =
            NaiveDateTime::parse_from_str(&split.next()?[1..], "%Y-%m-%d %H:%M").ok()?;

        let event = match split.next()?.trim() {
            "falls asleep" => Event::FallsAsleep,
            "wakes up" => Event::WakesUp,
            event => {
                let id = event.split_whitespace().nth(1)?[1..].parse().ok()?;
                Event::BeginsShift(id)
            }
        };

        Some(Entry { date_time, event })
    }
}

struct GuardEntry {
    minutes_asleep: [usize; 60],
}

impl GuardEntry {
    fn new() -> GuardEntry {
        GuardEntry {
            minutes_asleep: [0; 60],
        }
    }

    fn minutes_asleep(&self) -> usize {
        self.minutes_asleep.iter().sum()
    }

    fn best_minute(&self) -> (usize, usize) {
        self.minutes_asleep.iter().cloned().enumerate().max_by_key(|(_, count)| *count).unwrap()
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let mut entries = input
        .split('\n')
        .filter(|line| !line.is_empty())
        .filter_map(|line| Entry::new(line))
        .collect::<Vec<Entry>>();

    entries.sort_by(|a, b| a.date_time.cmp(&b.date_time));
    let mut guards = HashMap::new();

    let mut current_guard = None;
    let mut asleep_at = None;
    for entry in entries {
        match entry.event {
            Event::BeginsShift(guard) => {
                current_guard = Some(guards.entry(guard).or_insert(GuardEntry::new()));
            }
            Event::FallsAsleep => {
                asleep_at = Some(entry.date_time.time());
            }
            Event::WakesUp => {
                if let Some(asleep_at) = asleep_at {
                    if let Some(current_guard) = &mut current_guard {
                        let current_time = entry.date_time.time();
                        for i in asleep_at.minute()..current_time.minute() {
                            current_guard.minutes_asleep[i as usize] += 1;
                        }
                    }
                }
            }
        }
    }

    let best_guard = guards.iter().max_by_key(|(_, guard)| guard.minutes_asleep()).unwrap();
    println!("Part 1: Best guard: {:?} {:?}", best_guard.0, best_guard.1.minutes_asleep());
    let best_minute = guards.get(&best_guard.0).unwrap().best_minute();
    println!("Part 1: Best guard x Best Minute = {}", best_guard.0 * best_minute.0);

    let best_guard = guards.iter().max_by_key(|(_, guard)| guard.best_minute().1).unwrap();
    println!("Part 2: Best guard: {:?} {:?}", best_guard.0, best_guard.1.minutes_asleep());
    let best_minute = guards.get(&best_guard.0).unwrap().best_minute();
    println!("Part 2: Best guard x Best Minute = {}", best_guard.0 * best_minute.0);
}

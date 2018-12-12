use std::collections::HashMap;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("../input.txt").trim();

    let coordinates: Vec<(u32, u32)> = input
        .split("\n")
        .map(|line| {
            let mut split = line.split(", ");
            (
                split.next().unwrap().parse().unwrap(),
                split.next().unwrap().parse().unwrap(),
            )
        })
        .collect();

    let bounds = coordinates
        .iter()
        .fold(None, |bounds, coordinate| match bounds {
            None => Some((coordinate.0, coordinate.1, coordinate.0, coordinate.1)),
            Some(bounds) => {
                let min_x = bounds.0.min(coordinate.0);
                let min_y = bounds.1.min(coordinate.1);
                let max_x = bounds.2.max(coordinate.0);
                let max_y = bounds.3.max(coordinate.1);
                Some((min_x, min_y, max_x, max_y))
            }
        });

    let (min_x, min_y, max_x, max_y) = match bounds {
        Some(bounds) => bounds,
        None => return Err("Could not find bounds!".into()),
    };

    let mut grid: HashMap<usize, u32> = HashMap::new();

    for x in min_x..=max_x {
        for y in min_y..=max_y {
            let distances = coordinates
                .iter()
                .map(|coord| distance(*coord, (x, y)))
                .collect::<Vec<_>>();
            let min_dist = distances.iter().min().unwrap();
            let closest = distances
                .iter()
                .enumerate()
                .filter(|(_, distance)| *distance == min_dist)
                .collect::<Vec<_>>();
            if closest.len() == 1 {
                *grid.entry(closest[0].0).or_default() += 1;
            }
        }
    }

    let biggest_area = grid
        .iter()
        .max_by_key(|(_, dist)| *dist)
        .expect("Could not find coordinate with largest area");

    println!("Biggest Area: {}", biggest_area.1);

    // Part 2: Find the number of locations where the sum of the distance from that location to
    // all coordinates is less that 10000

    let mut area = 0;

    for x in min_x..=max_x {
        for y in min_y..=max_y {
            let total_distance: u32 = coordinates
                .iter()
                .map(|coord| distance(*coord, (x, y)))
                .sum();
            if total_distance < 10000 {
                area += 1;
            }
        }
    }

    println!(
        "There are {} locations with a total distance to all coordinates of less than 10000",
        area
    );

    Ok(())
}

fn distance(a: (u32, u32), b: (u32, u32)) -> u32 {
    ((a.0 as i32 - b.0 as i32).abs() + (a.1 as i32 - b.1 as i32).abs()) as u32
}

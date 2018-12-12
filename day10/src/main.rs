use regex::Regex;
use std::error::Error;

#[derive(Debug)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Velocity {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Point {
    pos: Position,
    vel: Velocity,
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("../input.txt").trim();
    let points = parse_input(&input)?;

    part1(points);

    Ok(())
}

fn parse_input(input: &str) -> Result<Vec<Point>, Box<dyn Error>> {
    let re = Regex::new(r"position=<\s*(?P<pos_x>[-0-9]+),\s*(?P<pos_y>[-0-9]+)> velocity=<\s*(?P<vel_x>[-0-9]+),\s*(?P<vel_y>[-0-9]+)>")?;

    let points = input
        .split("\n")
        .map(|line| {
            let captures = match re.captures(line) {
                Some(captures) => captures,
                None => return Err(format!("Invalid input: {}", line).into()),
            };

            let pos_x = captures["pos_x"].parse()?;
            let pos_y = captures["pos_y"].parse()?;
            let vel_x = captures["vel_x"].parse()?;
            let vel_y = captures["vel_y"].parse()?;

            Ok(Point {
                pos: Position { x: pos_x, y: pos_y },
                vel: Velocity { x: vel_x, y: vel_y },
            })
        })
        .collect::<Result<Vec<Point>, Box<Error>>>()?;

    Ok(points)
}

fn part1(mut points: Vec<Point>) {
    // These bounds worked for my input, might need to be adjusted for other inputs?
    for second in 1..=50000 {
        for point in &mut points {
            point.pos.x += point.vel.x;
            point.pos.y += point.vel.y;
        }
        print_sky(&points, second);
    }
}

fn print_sky(points: &[Point], second: u32) {
    let bounds: Option<(i32, i32, i32, i32)> =
        points.iter().fold(None, |bounds, point| match bounds {
            Some(bounds) => Some((
                bounds.0.min(point.pos.x),
                bounds.1.max(point.pos.x),
                bounds.2.min(point.pos.y),
                bounds.3.max(point.pos.y),
            )),
            None => Some((point.pos.x, point.pos.x, point.pos.y, point.pos.y)),
        });

    if let Some((min_x, max_x, min_y, max_y)) = bounds {
        let width = (min_x - max_x).abs() as usize;
        let height = (min_y - max_y).abs() as usize;

        // This condition worked for my input, might need adjusted for other inputs?
        if height > 15 {
            return;
        }

        let mut grid = vec![vec!['.'; width + 1]; height + 1];
        for point in points {
            let x = (point.pos.x - min_x) as usize;
            let y = (point.pos.y - min_y) as usize;
            *grid.get_mut(y).unwrap().get_mut(x).unwrap() = '#';
        }

        let output = grid.join(&'\n').iter().collect::<String>();

        println!("Second: {}\n{}", second, output);
    }
}

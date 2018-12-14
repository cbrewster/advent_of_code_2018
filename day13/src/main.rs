use std::collections::BTreeMap;
use std::error::Error;

type Tracks = Vec<Vec<Track>>;

#[derive(Debug, Clone, Copy)]
enum Track {
    Empty,
    Vertical,
    Horizontal,
    Intersection,
    CornerForward,
    CornerBackward,
}

impl Track {
    fn from_char(c: char) -> Result<Track, Box<dyn Error>> {
        match c {
            ' ' => Ok(Track::Empty),
            '|' | '^' | 'v' => Ok(Track::Vertical),
            '-' | '>' | '<' => Ok(Track::Horizontal),
            '+' => Ok(Track::Intersection),
            '/' => Ok(Track::CornerForward),
            '\\' => Ok(Track::CornerBackward),
            _ => Err(format!("Invalid track character: {}", c).into()),
        }
    }
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Clone, Copy)]
enum Movement {
    Right,
    Left,
    Straight,
}

impl Movement {
    fn next(self) -> Movement {
        match self {
            Movement::Left => Movement::Straight,
            Movement::Straight => Movement::Right,
            Movement::Right => Movement::Left,
        }
    }

    fn new_direction(self, old_direction: Direction) -> Direction {
        match (self, old_direction) {
            (Movement::Straight, direction) => direction,

            (Movement::Right, Direction::Up) => Direction::Right,
            (Movement::Right, Direction::Down) => Direction::Left,
            (Movement::Right, Direction::Left) => Direction::Up,
            (Movement::Right, Direction::Right) => Direction::Down,

            (Movement::Left, Direction::Up) => Direction::Left,
            (Movement::Left, Direction::Down) => Direction::Right,
            (Movement::Left, Direction::Left) => Direction::Down,
            (Movement::Left, Direction::Right) => Direction::Up,
        }
    }
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Clone)]
struct Cart {
    y: usize,
    x: usize,
    direction: Direction,
    next_decision: Movement,
}

impl Cart {
    fn new(x: usize, y: usize, direction: Direction) -> Cart {
        Cart {
            y,
            x,
            direction,
            next_decision: Movement::Left,
        }
    }

    fn from_char(c: char, x: usize, y: usize) -> Option<Cart> {
        match c {
            '>' => Some(Cart::new(x, y, Direction::Right)),
            '<' => Some(Cart::new(x, y, Direction::Left)),
            '^' => Some(Cart::new(x, y, Direction::Up)),
            'v' => Some(Cart::new(x, y, Direction::Down)),
            _ => None,
        }
    }

    fn move_cart(&mut self) {
        match self.direction {
            Direction::Up => self.y -= 1,
            Direction::Down => self.y += 1,
            Direction::Right => self.x += 1,
            Direction::Left => self.x -= 1,
        }
    }

    fn update_direction(&mut self, track: Track) {
        let movement = match track {
            Track::Vertical | Track::Horizontal => Movement::Straight,
            Track::Intersection => {
                let movement = self.next_decision;
                self.next_decision = self.next_decision.next();
                movement
            }
            Track::CornerBackward => match self.direction {
                Direction::Up | Direction::Down => Movement::Left,
                Direction::Left | Direction::Right => Movement::Right,
            },
            Track::CornerForward => match self.direction {
                Direction::Up | Direction::Down => Movement::Right,
                Direction::Left | Direction::Right => Movement::Left,
            },
            _ => panic!("Bad things happened!"),
        };
        self.direction = movement.new_direction(self.direction);
    }
}

#[derive(Debug, Clone)]
struct Crop {
    carts: BTreeMap<(usize, usize), Cart>,
    tracks: Tracks,
}

impl Crop {
    fn new(carts: BTreeMap<(usize, usize), Cart>, tracks: Tracks) -> Crop {
        Crop { carts, tracks }
    }

    /// Returns the crash location if a crash occurred.
    fn tick(&mut self) -> Option<(usize, usize)> {
        // Not very happy with this solution, but it works and I don't have enough time to clean it
        // up :(
        let mut new_carts = BTreeMap::new();
        let mut crash: Option<(usize, usize)> = None;
        let mut carts_vec: Vec<_> = self.carts.values().rev().collect();

        while let Some(cart) = carts_vec.pop() {
            let mut new_cart = cart.clone();
            new_cart.move_cart();
            let track = self.tracks[new_cart.y][new_cart.x];
            new_cart.update_direction(track);

            let new_position = (new_cart.y, new_cart.x);
            if let Some(pos) = carts_vec.iter().position(|cart| new_position == (cart.y, cart.x))
            {
                carts_vec.remove(pos);
                crash = Some(new_position);
            } else if new_carts.remove(&new_position).is_some() {
                crash = Some(new_position);
            } else {
                new_carts.insert(new_position, new_cart);
            }
        }
        if crash.is_some() {
            assert_eq!(self.carts.len() - 2, new_carts.len());
        }

        self.carts = new_carts;

        crash
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let input = include_str!("../input.txt");
    let mut tracks: Tracks = Vec::new();
    let mut carts = BTreeMap::new();

    for (y, line) in input.split('\n').enumerate() {
        let mut row = Vec::new();
        for (x, c) in line.chars().enumerate() {
            row.push(Track::from_char(c)?);
            if let Some(cart) = Cart::from_char(c, x, y) {
                carts.insert((cart.y, cart.x), cart);
            }
        }
        tracks.push(row);
    }

    let crop = Crop::new(carts, tracks);
    part1(crop.clone());
    part2(crop);

    Ok(())
}

fn part1(mut crop: Crop) {
    let mut crash = None;
    while crash.is_none() {
        crash = crop.tick();
    }

    let crash_location = crash.unwrap();
    println!(
        "The first crash occurs at: {},{}",
        crash_location.1, crash_location.0
    );
}

fn part2(mut crop: Crop) {
    while crop.carts.len() > 1 {
        crop.tick();
    }

    let (position, _) = crop.carts.iter().next().unwrap();
    println!(
        "The last cart is at: {},{}",
        position.1, position.0
    );
}

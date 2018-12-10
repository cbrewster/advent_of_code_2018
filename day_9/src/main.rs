use regex::Regex;
use std::error::Error;

fn main() -> Result<(), Box<Error>> {
    let input = include_str!("../input.txt").trim();
    let re =
        Regex::new(r"(?P<players>\d+) players; last marble is worth (?P<last_marble>\d+) points")?;
    let captures = match re.captures(input) {
        Some(captures) => captures,
        None => return Err("Invalid input!".into()),
    };

    let players = captures["players"].parse::<u32>()?;
    let last_marble = captures["last_marble"].parse::<u32>()?;

    println!(
        "Playing marbles with {} players and a last marble of {}!",
        players, last_marble
    );

    let high_score = play_game(players, last_marble);
    println!("The high score is {}", high_score);

    println!(
        "Playing marbles with {} players and a last marble of {}!",
        players,
        last_marble * 100
    );

    let high_score = play_game(players, last_marble * 100);
    println!("The high score is {}", high_score);

    Ok(())
}

fn play_game(player_count: u32, last_marble: u32) -> u32 {
    let mut players = (1..=player_count).map(|id| Player::new(id)).collect::<Vec<_>>();
    let mut circle = Circle::new(last_marble as usize);
    for (player_id, value) in (0..player_count).cycle().zip(1..=last_marble) {
        if let Some(score) = circle.play_turn(value) {
            players[player_id as usize].score += score;
        }
    }

    players.iter().map(|player| player.score).max().unwrap()
}

struct Player {
    _id: u32,
    score: u32,
}

impl Player {
    fn new(id: u32) -> Player {
        Player { _id: id, score: 0 }
    }
}

type MarbleId = usize;
struct Marble {
    value: u32,
    clockwise: MarbleId,
    counter_clockwise: MarbleId,
}

impl Marble {
    fn new(value: u32, clockwise: MarbleId, counter_clockwise: MarbleId) -> Marble {
        Marble {
            value,
            clockwise,
            counter_clockwise,
        }
    }
}

struct Circle {
    marbles: Vec<Marble>,
    current_marble: MarbleId,
}

impl Circle {
    fn new(max: usize) -> Circle {
        let mut marbles = Vec::with_capacity(max);
        marbles.push(Marble::new(0, 0, 0));
        Circle {
            marbles,
            current_marble: 0,
        }
    }

    fn play_turn(&mut self, value: u32) -> Option<u32> {
        if value % 23 != 0 {
            let marble_before = self.clockwise(1);
            self.current_marble = self.insert_after(value, marble_before);
            None
        } else {
            let marble_to_remove = self.counter_clockwise(7);
            self.current_marble = self.marbles[marble_to_remove].clockwise;
            self.remove(marble_to_remove);
            Some(value + self.marbles[marble_to_remove].value)
        }
    }

    fn clockwise(&self, distance: usize) -> MarbleId {
        let mut result = self.current_marble;
        for _ in 0..distance {
            result = self.marbles[result].clockwise;
        }
        result
    }

    fn counter_clockwise(&self, distance: usize) -> MarbleId {
        let mut result = self.current_marble;
        for _ in 0..distance {
            result = self.marbles[result].counter_clockwise;
        }
        result
    }

    fn insert_after(&mut self, value: u32, marble: MarbleId) -> MarbleId {
        let before = marble;
        let after = self.marbles[marble].clockwise;
        let marble = Marble::new(value, after, before);
        self.marbles.push(marble);
        let id = self.marbles.len() - 1;
        self.marbles[before].clockwise = id;
        self.marbles[after].counter_clockwise = id;
        id
    }

    fn remove(&mut self, marble: MarbleId) {
        let before = self.marbles[marble].counter_clockwise;
        let after = self.marbles[marble].clockwise;

        self.marbles[before].clockwise = after;
        self.marbles[after].counter_clockwise = before;
    }
}

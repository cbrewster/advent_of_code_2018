const INPUT: &str = "652601";

fn main() {
    part1();
    part2();
}

fn part1() {
    let input = INPUT.parse().unwrap();
    let recipes = Recipes::new().skip(input).take(10);
    println!("The 10 scores after {} recipes are {}", INPUT, to_output(recipes));
}

fn part2() {
    let count = find_pattern(INPUT);

    println!("The pattern {} appears after {} recipes.", INPUT, count);
}

fn find_pattern(input: &str) -> usize {
    let length = input.len() as u32;
    let input: usize = INPUT.parse().unwrap();

    let mut recipes = Recipes::new();
    let mut pattern = 0;
    let mut count = 0;

    for i in 1..=length {
        let score = recipes.next().unwrap();
        pattern += score * 10usize.pow(length-i);
    }

    while pattern != input {
        let score = recipes.next().unwrap();
        pattern = ((pattern * 10) + score) % 10usize.pow(length);
        count += 1;
    }

    count
}

type RecipeId = usize;
type Score = usize;

struct Recipes {
    elves: [RecipeId; 2],
    recipes: Vec<Score>,
    current_index: usize,
}

impl Recipes {
    fn new() -> Recipes {
        Recipes {
            recipes: vec![3, 7],
            elves: [0, 1],
            current_index: 0,
        }
    }
}

impl Iterator for Recipes {
    type Item = Score;

    fn next(&mut self) -> Option<Score> {
        if let Some(score) = self.recipes.get(self.current_index) {
            self.current_index += 1;
            return Some(*score);
        }

        let sum = self.recipes[self.elves[0]] + self.recipes[self.elves[1]];
        let tens = sum / 10;
        let ones = sum % 10;

        if tens > 0 {
            self.recipes.push(tens);
        }
        self.recipes.push(ones);

        self.elves[0] = (self.elves[0] + self.recipes[self.elves[0]] + 1) % self.recipes.len();
        self.elves[1] = (self.elves[1] + self.recipes[self.elves[1]] + 1) % self.recipes.len();

        let recipe = self.recipes.get(self.current_index).cloned();
        self.current_index += 1;
        recipe
    }
}

fn to_output<I: Iterator<Item = Score>>(scores: I) -> String {
    scores.map(|score| score.to_string()).collect()
}

#[cfg(test)]
mod tests {
    use super::{to_output, Recipes};
    #[test]
    fn after_9() {
        let recipes = Recipes::new().skip(9).take(10);

        assert_eq!(to_output(recipes), "5158916779");
    }

    #[test]
    fn after_5() {
        let recipes = Recipes::new().skip(5).take(10);

        assert_eq!(to_output(recipes), "0124515891");
    }

    #[test]
    fn after_18() {
        let recipes = Recipes::new().skip(18).take(10);

        assert_eq!(to_output(recipes), "9251071085");
    }

    #[test]
    fn after_2018() {
        let recipes = Recipes::new().skip(2018).take(10);

        assert_eq!(to_output(recipes), "5941429882");
    }
}

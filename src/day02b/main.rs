use anyhow::anyhow;
use std::{fs::File, io::BufRead, io::BufReader};

enum Shape {
    Rock,
    Paper,
    Scissors,
}

enum Outcome {
    Win,
    Loss,
    Draw,
}

impl Shape {
    pub fn play(&self, other: &Shape) -> Outcome {
        match (self, other) {
            (Shape::Rock, Shape::Rock)
            | (Shape::Paper, Shape::Paper)
            | (Shape::Scissors, Shape::Scissors) => Outcome::Draw,
            (Shape::Rock, Shape::Paper) => Outcome::Loss,
            (Shape::Rock, Shape::Scissors) => Outcome::Win,
            (Shape::Paper, Shape::Rock) => Outcome::Win,
            (Shape::Paper, Shape::Scissors) => Outcome::Loss,
            (Shape::Scissors, Shape::Rock) => Outcome::Loss,
            (Shape::Scissors, Shape::Paper) => Outcome::Win,
        }
    }
    pub fn get_shape_for_outcome(&self, outcome: &Outcome) -> Shape {
        match (self, outcome) {
            (Shape::Rock, Outcome::Draw) => Shape::Rock,
            (Shape::Paper, Outcome::Draw) => Shape::Paper,
            (Shape::Scissors, Outcome::Draw) => Shape::Scissors,
            (Shape::Rock, Outcome::Loss) => Shape::Scissors,
            (Shape::Rock, Outcome::Win) => Shape::Paper,
            (Shape::Paper, Outcome::Loss) => Shape::Rock,
            (Shape::Paper, Outcome::Win) => Shape::Scissors,
            (Shape::Scissors, Outcome::Loss) => Shape::Paper,
            (Shape::Scissors, Outcome::Win) => Shape::Rock,
        }
    }
    pub fn score(&self) -> u32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }
}

impl TryFrom<char> for Shape {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' => Ok(Shape::Rock),
            'B' => Ok(Shape::Paper),
            'C' => Ok(Shape::Scissors),
            _ => Err(anyhow!("invalid shape: {}", value)),
        }
    }
}

impl TryFrom<char> for Outcome {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'X' => Ok(Outcome::Loss),
            'Y' => Ok(Outcome::Draw),
            'Z' => Ok(Outcome::Win),
            _ => Err(anyhow!("invalid outcome: {}", value)),
        }
    }
}

impl Outcome {
    fn score(&self) -> u32 {
        match self {
            Outcome::Loss => 0,
            Outcome::Draw => 3,
            Outcome::Win => 6,
        }
    }
}

fn score_round(opponent_shape: &Shape, your_shape: &Shape) -> u32 {
    your_shape.play(opponent_shape).score() + your_shape.score()
}

fn main() -> anyhow::Result<()> {
    let mut total: u32 = 0;

    let file = File::open("input/day02.txt")?;

    for line in BufReader::new(file).lines().flatten() {
        let mut chars = line.chars();

        let opponent: Option<Shape> = chars.next().and_then(|c| Shape::try_from(c).ok());
        let _ = chars.next();
        let outcome: Option<Outcome> = chars.next().and_then(|c| Outcome::try_from(c).ok());

        let (Some(opponent), Some(outcome)) = (opponent, outcome) else {
            return Err(anyhow!("Failed parsing"));
        };

        let your_shape = opponent.get_shape_for_outcome(&outcome);

        let score = score_round(&opponent, &your_shape);

        total += score;
    }

    println!("{}", total);

    Ok(())
}

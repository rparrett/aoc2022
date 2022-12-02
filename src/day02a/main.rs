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
            'A' | 'X' => Ok(Shape::Rock),
            'B' | 'Y' => Ok(Shape::Paper),
            'C' | 'Z' => Ok(Shape::Scissors),
            _ => Err(anyhow!("invalid shape: {}", value)),
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
        let you: Option<Shape> = chars.next().and_then(|c| Shape::try_from(c).ok());

        let (Some(opponent), Some(you)) = (opponent, you) else {
            return Err(anyhow!("Failed parsing"));
        };

        let score = score_round(&opponent, &you);

        total += score;
    }

    println!("{}", total);

    Ok(())
}

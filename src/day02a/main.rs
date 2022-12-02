use anyhow::anyhow;
use std::{cmp::Ordering, fs::File, io::BufRead, io::BufReader};

#[derive(Eq, PartialEq)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    pub fn from_opponent_choice(choice: &str) -> anyhow::Result<Shape> {
        match choice {
            "A" => Ok(Shape::Rock),
            "B" => Ok(Shape::Paper),
            "C" => Ok(Shape::Scissors),
            _ => Err(anyhow!("invalid opponent choice: {}", choice)),
        }
    }
    pub fn from_your_choice(choice: &str) -> anyhow::Result<Shape> {
        match choice {
            "X" => Ok(Shape::Rock),
            "Y" => Ok(Shape::Paper),
            "Z" => Ok(Shape::Scissors),
            _ => Err(anyhow!("invalid choice: {}", choice)),
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
impl Ord for Shape {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Shape::Rock, Shape::Rock)
            | (Shape::Paper, Shape::Paper)
            | (Shape::Scissors, Shape::Scissors) => Ordering::Equal,
            (Shape::Rock, Shape::Paper) => Ordering::Less,
            (Shape::Rock, Shape::Scissors) => Ordering::Greater,
            (Shape::Paper, Shape::Rock) => Ordering::Greater,
            (Shape::Paper, Shape::Scissors) => Ordering::Less,
            (Shape::Scissors, Shape::Rock) => Ordering::Less,
            (Shape::Scissors, Shape::Paper) => Ordering::Greater,
        }
    }
}
impl PartialOrd for Shape {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn score_round(opponent_choice: &Shape, your_choice: &Shape) -> u32 {
    (match your_choice.cmp(opponent_choice) {
        Ordering::Less => 0,
        Ordering::Equal => 3,
        Ordering::Greater => 6,
    }) + your_choice.score()
}

fn main() -> anyhow::Result<()> {
    let mut total: u32 = 0;

    let file = File::open("input/day02.txt")?;

    for line in BufReader::new(file).lines().flatten() {
        let (opponent, yours) = line
            .split_once(' ')
            .ok_or_else(|| anyhow!("parsing failed"))?;

        let opponent_choice = Shape::from_opponent_choice(opponent)?;
        let your_choice = Shape::from_your_choice(yours)?;

        let score = score_round(&opponent_choice, &your_choice);

        total += score;
    }

    println!("{}", total);

    Ok(())
}

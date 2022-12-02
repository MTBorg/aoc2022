#[derive(Debug)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug)]
enum Outcome {
    Win,
    Loss,
    Draw,
}

use Outcome::*;
use Shape::*;

impl Outcome {
    fn score(&self) -> u32 {
        match self {
            Win => 6,
            Loss => 0,
            Draw => 3,
        }
    }
}

impl std::convert::From<&str> for Outcome {
    fn from(s: &str) -> Self {
        match s {
            "X" => Loss,
            "Y" => Draw,
            "Z" => Win,
            _ => panic!("Unrecognized character"),
        }
    }
}

impl Shape {
    fn score(&self) -> u32 {
        match self {
            Rock => 1,
            Paper => 2,
            Scissors => 3,
        }
    }

    fn play(&self, other: &Self) -> Outcome {
        match (self, other) {
            (Rock, Scissors) | (Paper, Rock) | (Scissors, Paper) => Win,
            (Rock, Rock) | (Paper, Paper) | (Scissors, Scissors) => Draw,
            _ => Loss,
        }
    }

    fn play_for_outcome(&self, expected_outcome: &Outcome) -> Shape {
        match (self, expected_outcome) {
            (Rock, Win) => Paper,
            (Rock, Loss) => Scissors,
            (Rock, Draw) => Rock,

            (Paper, Win) => Scissors,
            (Paper, Loss) => Rock,
            (Paper, Draw) => Paper,

            (Scissors, Win) => Rock,
            (Scissors, Loss) => Paper,
            (Scissors, Draw) => Scissors,
        }
    }
}

impl std::convert::From<&str> for Shape {
    fn from(s: &str) -> Self {
        match s {
            "A" | "X" => Rock,
            "B" | "Y" => Paper,
            "C" | "Z" => Scissors,
            _ => panic!("Unrecognized character"),
        }
    }
}

fn main() {
    let input: String = std::fs::read_to_string("input.txt").unwrap();

    let guide: Vec<(Shape, Shape)> = input
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<&str>>())
        .map(|v| (Shape::from(v[0]), Shape::from(v[1])))
        .collect();

    let my_score: u32 = guide
        .iter()
        .map(|(other, me)| me.play(other).score() + me.score())
        .sum();

    println!("Part1: {my_score}");

    let guide: Vec<(Shape, Outcome)> = input
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<&str>>())
        .map(|v| (Shape::from(v[0]), Outcome::from(v[1])))
        .collect();

    let my_score: u32 = guide
        .iter()
        .map(|(other, expected_outcome)| {
            other.play_for_outcome(expected_outcome).score() + expected_outcome.score()
        })
        .sum();

    println!("Part2: {my_score}");
}

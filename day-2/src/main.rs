use clap::Parser;
use std::fs;

#[derive(PartialEq, PartialOrd, Debug, Hash, Eq, Ord, Clone)]
enum Shape {
    Paper,
    Scissor,
    Rock,
}

#[derive(Debug, PartialEq)]
enum Outcome {
    Win,
    Loss,
    Draw,
}

#[derive(Debug)]
struct Matchup {
    required_outcome: Outcome,
    enemy: Shape,
}

#[derive(Parser, Debug)]
struct Args {
    input_file: String,
}

fn main() {
    let input_file = Args::parse().input_file;
    let data = fs::read_to_string(input_file).expect("This file doesn't exist!");
    let parsed = parse_input(data);
    let final_score = compute_total_score(parsed);

    println!("Result:{}", final_score);
}

fn parse_shape(input_char: &str) -> Shape {
    match input_char {
        "A" => Shape::Rock,
        "B" => Shape::Paper,
        "C" => Shape::Scissor,
        _ => panic!("Unknown action"),
    }
}

fn parse_required_outcome(input_char: &str) -> Outcome {
    match input_char {
        "X" => Outcome::Loss,
        "Y" => Outcome::Draw,
        "Z" => Outcome::Win,
        _ => panic!("Unknown outcome"),
    }
}

fn get_required_shape(matchup: &Matchup) -> Shape {
    match matchup.required_outcome {
        Outcome::Draw => matchup.enemy.clone(), // A draw happens when both shapes are the same
        Outcome::Loss => action_wins_against(&matchup.enemy), // A win happens when the action wins against the enemy's shape
        Outcome::Win => action_loses_against(&matchup.enemy), // A loss happens when the action loses against the enemy's shape
    }
}

fn parse_input(input: String) -> Vec<Matchup> {
    input
        .trim()
        .split("\n")
        .into_iter()
        .map(|input_line| {
            let to_string = String::from(input_line);
            let (enemy, required_outcome) = to_string.split_once(" ").unwrap();

            Matchup {
                enemy: parse_shape(enemy),
                required_outcome: parse_required_outcome(required_outcome),
            }
        })
        .collect()
}

fn action_wins_against(shape: &Shape) -> Shape {
    match shape {
        Shape::Paper => Shape::Rock,
        Shape::Rock => Shape::Scissor,
        Shape::Scissor => Shape::Paper,
    }
}

fn action_loses_against(shape: &Shape) -> Shape {
    match shape {
        Shape::Paper => Shape::Scissor,
        Shape::Rock => Shape::Paper,
        Shape::Scissor => Shape::Rock,
    }
}

fn get_base_round_score(shape: &Shape) -> usize {
    match shape {
        Shape::Rock => 1,
        Shape::Paper => 2,
        Shape::Scissor => 3,
    }
}

fn get_outcome_round_score(outcome: &Outcome) -> usize {
    match outcome {
        Outcome::Loss => 0,
        Outcome::Draw => 3,
        Outcome::Win => 6,
    }
}

fn compute_round_score(matchup: &Matchup) -> usize {
    let shape = get_required_shape(matchup);

    get_outcome_round_score(&matchup.required_outcome) + get_base_round_score(&shape)
}

fn compute_total_score(matchups: Vec<Matchup>) -> usize {
    matchups
        .iter()
        .fold(0, |score, matchup| compute_round_score(matchup) + score)
}

#[cfg(test)]
mod tests {
    use crate::{compute_total_score, parse_input};

    #[test]
    fn test_example() {
        assert_eq!(
            compute_total_score(parse_input(String::from("A Y\nB X\nC Z"))),
            15
        );
    }
}

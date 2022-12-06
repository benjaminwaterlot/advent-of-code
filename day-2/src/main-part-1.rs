use clap::Parser;
use std::fs;

#[derive(PartialEq, PartialOrd, Debug, Hash, Eq, Ord)]
enum Action {
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
    myself: Action,
    enemy: Action,
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

    println!("{}", final_score);
}

fn parse_action(input_char: &str) -> Action {
    match input_char {
        "A" | "X" => Action::Rock,
        "B" | "Y" => Action::Paper,
        "C" | "Z" => Action::Scissor,
        _ => panic!("Unknown action"),
    }
}

fn parse_input(input: String) -> Vec<Matchup> {
    input
        .trim()
        .split("\n")
        .into_iter()
        .map(|input_line| {
            let to_string = String::from(input_line);
            let (enemy, myself) = to_string.split_once(" ").unwrap();

            Matchup {
                enemy: parse_action(enemy),
                myself: parse_action(myself),
            }
        })
        .collect()
}

fn action_wins_against(action: &Action) -> Action {
    match action {
        Action::Paper => Action::Rock,
        Action::Rock => Action::Scissor,
        Action::Scissor => Action::Paper,
    }
}

fn get_base_round_score(action: &Action) -> usize {
    match action {
        Action::Rock => 1,
        Action::Paper => 2,
        Action::Scissor => 3,
    }
}

fn get_outcome_round_score(action: &Outcome) -> usize {
    match action {
        Outcome::Loss => 0,
        Outcome::Draw => 3,
        Outcome::Win => 6,
    }
}

fn get_round_outcome(matchup: &Matchup) -> Outcome {
    if matchup.enemy == matchup.myself {
        return Outcome::Draw;
    }

    if action_wins_against(&matchup.myself) == matchup.enemy {
        return Outcome::Win;
    } else {
        return Outcome::Loss;
    }
}

fn compute_round_score(matchup: &Matchup) -> usize {
    let outcome = get_round_outcome(&matchup);

    get_outcome_round_score(&outcome) + get_base_round_score(&matchup.myself)
}

fn compute_total_score(matchups: Vec<Matchup>) -> usize {
    matchups
        .iter()
        .fold(0, |score, matchup| compute_round_score(matchup) + score)
}

#[cfg(test)]
mod tests {
    use crate::{compute_total_score, get_round_outcome, parse_input, Action, Matchup, Outcome};

    #[test]
    fn test_parse_fn() {
        let parsed = parse_input(String::from("A Y\nB X\nC Z"));

        assert_eq!(parsed.len(), 3);
    }

    #[test]
    fn test_round_outcome() {
        assert_eq!(
            get_round_outcome(&Matchup {
                enemy: Action::Paper,
                myself: Action::Rock
            }),
            Outcome::Loss
        );

        assert_eq!(
            get_round_outcome(&Matchup {
                enemy: Action::Rock,
                myself: Action::Rock
            }),
            Outcome::Draw
        );
    }

    #[test]
    fn test_example() {
        assert_eq!(
            compute_total_score(parse_input(String::from("A Y\nB X\nC Z"))),
            15
        );
    }
}

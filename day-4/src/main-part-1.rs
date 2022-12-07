use clap::Parser;
use std::fs;

#[derive(Parser, Debug)]
struct Args {
    input_file: String,
}

type Assignment = [usize; 2];
type Pair = [Assignment; 2];

fn assignment_overlaps(pair: &Pair) -> bool {
    let first = pair[0];
    let second = pair[1];

    (first[0] <= second[0] && first[1] >= second[1])
        || (second[0] <= first[0] && second[1] >= first[1])
}

fn parse_input(input: String) -> Vec<Pair> {
    input
        .trim()
        .split("\n")
        .map(|line| {
            let assignments = line.split_once(",").unwrap();

            [assignments.0, assignments.1].map(|assignment| {
                let rooms = assignment.split_once("-").unwrap();
                [
                    str::parse::<usize>(rooms.0).unwrap(),
                    str::parse::<usize>(rooms.1).unwrap(),
                ]
            })
        })
        .collect::<Vec<Pair>>()
}

fn main() {
    let data = fs::read_to_string(Args::parse().input_file).expect("This file doesn't exist!");
    let pairs = parse_input(data);

    let count = pairs
        .iter()
        .fold(0, |count, pair| match assignment_overlaps(pair) {
            true => count + 1,
            false => count,
        });

    println!("{}", count);
}

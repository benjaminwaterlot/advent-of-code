use clap::Parser;
use std::{collections::HashSet, fs};

#[derive(Parser, Debug)]
struct Args {
    input_file: String,
}

type Section = isize;
type Assignment = [Section; 2];
type Pair = [Assignment; 2];

fn do_assignments_overlap(pair: &Pair) -> bool {
    let first = pair[0];
    let second = pair[1];

    let mut hash = HashSet::new();

    for section in first[0]..=first[1] {
        hash.insert(section);
    }

    for section in second[0]..=second[1] {
        if hash.contains(&section) {
            return true;
        }
    }

    return false;
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
                    str::parse::<Section>(rooms.0).unwrap(),
                    str::parse::<Section>(rooms.1).unwrap(),
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
        .fold(0, |count, pair| match do_assignments_overlap(pair) {
            true => count + 1,
            false => count,
        });

    println!("{}", count);
}

#[cfg(test)]
mod tests {
    use crate::do_assignments_overlap;

    #[test]
    fn test_overlaps() {
        assert_eq!(do_assignments_overlap(&[[2, 3], [3, 4]]), true);
        assert_eq!(do_assignments_overlap(&[[2, 4], [4, 4]]), true);
        assert_eq!(do_assignments_overlap(&[[2, 3], [4, 8]]), false);
        assert_eq!(do_assignments_overlap(&[[49, 49], [10, 50]]), true);
        assert_eq!(do_assignments_overlap(&[[10, 50], [49, 49],]), true);
    }
}

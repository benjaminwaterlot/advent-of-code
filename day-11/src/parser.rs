use crate::{Monkey, Operation, Target};
use clap::Parser;
use std::fs;

#[derive(Parser)]
struct Args {
    input_file: String,
}

fn parse_monkey(monkey_string: &str) -> Monkey {
    let [_, items_line, operation_line, test_line, target_line_true, target_line_false]: [String;
        6] = monkey_string
        .to_string()
        .splitn(6, "\n")
        .map(|l| l.to_string())
        .collect::<Vec<String>>()
        .try_into()
        .expect("Didn't have 6 lines");

    let items = items_line
        .split_once(":")
        .unwrap()
        .1
        .split(", ")
        .map(|item| item.trim().parse::<isize>().expect("unable to parse num"))
        .collect();

    let operation: Operation = if operation_line.contains("old * old") {
        Operation::Square
    } else if operation_line.contains("*") {
        Operation::Multiply(
            operation_line
                .split_once("*")
                .unwrap()
                .1
                .trim()
                .parse::<isize>()
                .unwrap(),
        )
    } else {
        Operation::Add(
            operation_line
                .split_once("+")
                .unwrap()
                .1
                .trim()
                .parse::<isize>()
                .unwrap(),
        )
    };

    let test: isize = test_line
        .split_whitespace()
        .last()
        .unwrap()
        .parse()
        .unwrap();

    let target: Target = (
        target_line_true
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<isize>()
            .unwrap(),
        target_line_false
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<isize>()
            .unwrap(),
    );

    return Monkey {
        items,
        operation,
        test,
        target,
    };
}

pub fn parse_input() -> Vec<Monkey> {
    let input = fs::read_to_string(Args::parse().input_file).unwrap();

    let monkeys: Vec<Monkey> = input.split("\n\n").map(parse_monkey).collect();

    return monkeys;
}

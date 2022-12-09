use clap::Parser;
use std::fs;

#[derive(Parser)]
struct Args {
    input_file: String,
}

type Crate = char;
type Stack = Vec<Crate>;

#[derive(Debug)]
struct Instruction {
    quantity: usize,
    from_stack: usize,
    to_stack: usize,
}

fn main() {
    let input = get_input();

    let (stacks_input, instructions_input) = input.split_once("\n\n").unwrap();

    let stacks: Vec<Stack> = parse_crates(stacks_input);
    let instructions: Vec<Instruction> = parse_instructions(instructions_input);

    let stacks_part_2: Vec<Stack> = stacks.clone();

    let res_part_1: String = get_top_crates(&move_crates_with_9000(stacks, &instructions));
    println!("part 1:{res_part_1:#?}");

    let res_part_2: String = get_top_crates(&move_crates_with_9001(stacks_part_2, &instructions));
    println!("part 2:{res_part_2:#?}");
}

fn get_top_crates(stacks: &Vec<Stack>) -> String {
    stacks.iter().filter_map(|stack| stack.last()).collect()
}

fn get_input() -> String {
    let input_file = Args::parse().input_file;
    fs::read_to_string(input_file).expect("This file doesn't exist!")
}

fn move_crates_with_9000(mut stacks: Vec<Stack>, instructions: &Vec<Instruction>) -> Vec<Stack> {
    for instruction in instructions {
        for _ in 0..instruction.quantity {
            if let Some(crate_id) = stacks[instruction.from_stack - 1].pop() {
                stacks[instruction.to_stack - 1].push(crate_id);
            }
        }
    }

    stacks
}

fn move_crates_with_9001(mut stacks: Vec<Stack>, instructions: &Vec<Instruction>) -> Vec<Stack> {
    for instruction in instructions {
        let from_stack = &mut stacks[instruction.from_stack - 1];

        let moved_crates: Vec<char> = from_stack
            .splice(
                (from_stack.len() - instruction.quantity)..from_stack.len(),
                [],
            )
            .collect();

        let to_stack = &mut stacks[instruction.to_stack - 1];

        to_stack.splice((to_stack.len())..(to_stack.len()), moved_crates);
    }

    stacks
}

fn parse_instructions(instructions_input: &str) -> Vec<Instruction> {
    let lines = instructions_input.trim().to_string();

    lines
        .split("\n")
        .map(|line| {
            let line_parts: Vec<String> = line.split(" ").map(|l| l.to_string()).collect();

            Instruction {
                quantity: str::parse::<usize>(&line_parts[1]).unwrap(),
                from_stack: str::parse::<usize>(&line_parts[3]).unwrap(),
                to_stack: str::parse::<usize>(&line_parts[5]).unwrap(),
            }
        })
        .collect()
}

fn parse_crates(stacks_input: &str) -> Vec<Stack> {
    let lines: Vec<String> = stacks_input
        .split("\n")
        .map(|line| line.to_string())
        .collect();

    let indexing_line = lines.iter().find(|line| line.starts_with(" 1")).unwrap();
    let stacks_count = indexing_line.split("   ").collect::<Vec<&str>>().len();

    let mut stacks: Vec<Stack> = vec![];

    for crate_name in 1..=stacks_count {
        stacks.push(vec![]);

        for line in &lines {
            if line.starts_with(" 1") {
                break;
            }

            let crate_value: Crate = line.chars().nth(1 + (crate_name - 1) * 4).unwrap();
            if crate_value == ' ' {
                continue;
            }

            stacks[crate_name - 1].insert(0, crate_value);
        }
    }

    stacks
}

use clap::Parser;
use std::{fs, vec};

#[derive(Parser)]
struct Args {
    input_file: String,
}

#[derive(Debug)]
enum Instruction {
    Noop,
    AddX(isize),
}

fn main() {
    let cycles = to_cycles(parse_input(get_input()));

    let res_part_1 = get_results(&cycles);
    println!("Part 1: {res_part_1}");

    let res_part_2 = to_crt(cycles);
    println!("Part 2:\n\n{res_part_2}");
}

fn get_input() -> String {
    let input_file = Args::parse().input_file;
    fs::read_to_string(input_file).expect("This file doesn't exist!")
}

fn parse_input(input: String) -> Vec<Instruction> {
    input
        .split("\n")
        .map(|l| {
            let line = l.to_string();
            let mut parts = line.split(" ");

            match parts.next().unwrap() {
                "noop" => Instruction::Noop,
                "addx" => Instruction::AddX(parts.next().unwrap().to_string().parse().unwrap()),
                _ => panic!(),
            }
        })
        .collect()
}

fn to_crt(cycles: Vec<isize>) -> String {
    cycles
        .iter()
        .take(240)
        .enumerate()
        .map(|(index, x_register)| {
            let mut pixel = String::from("");
            let idx = isize::try_from(index).unwrap_or(0);
            let cycle: isize = idx + 1;
            let is_printable = (idx % 40 - x_register).abs() <= 1;

            match is_printable {
                true => pixel.push('#'),
                false => pixel.push('.'),
            };

            if cycle % 40 == 0 {
                pixel.push('\n');
            }

            pixel
        })
        .collect()
}

fn to_cycles(instructions: Vec<Instruction>) -> Vec<isize> {
    let mut cycles = vec![1];

    for instruction in instructions {
        let last_value = cycles.last().unwrap_or(&1);

        match instruction {
            Instruction::Noop => cycles.push(*last_value),
            Instruction::AddX(x) => {
                cycles.append(&mut vec![*last_value, *last_value + x]);
            }
        }
    }

    cycles
}

fn get_results(cycles: &Vec<isize>) -> isize {
    cycles
        .iter()
        .enumerate()
        .fold(0, |count, (index, x_register)| {
            let cycle = index + 1;

            match cycle {
                20 | 60 | 100 | 140 | 180 | 220 => {
                    count + (isize::try_from(cycle).unwrap() * x_register)
                }
                _ => count,
            }
        })
}

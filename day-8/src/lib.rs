use clap::Parser;
use std::fs;

#[derive(Parser)]
struct Args {
    input_file: String,
}

pub fn get_input() -> String {
    let input_file = Args::parse().input_file;
    fs::read_to_string(input_file)
        .expect("This file doesn't exist!")
        .trim()
        .to_string()
}

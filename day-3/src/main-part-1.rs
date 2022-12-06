use clap::Parser;
use std::{collections::HashSet, fs};

#[derive(Parser, Debug)]
struct Args {
    input_file: String,
}

fn get_item_value(item: char) -> u32 {
    if item >= 'a' && item <= 'z' {
        item.to_digit(36).unwrap() - 9
    } else {
        item.to_digit(36).unwrap() + 17
    }
}

#[derive(Debug)]
struct Rucksack {
    compartment_1: Vec<u32>,
    compartment_2: Vec<u32>,
}

fn main() {
    let data = fs::read_to_string(Args::parse().input_file).expect("This file doesn't exist!");

    let count = data.trim().split("\n").fold(0, |count, rucksack| {
        let chars = rucksack
            .to_string()
            .chars()
            .map(get_item_value)
            .collect::<Vec<u32>>();

        let (compartment_1, compartment_2) = chars.split_at(chars.len() / 2);

        let rucksack_struct = Rucksack {
            compartment_1: compartment_1.to_vec(),
            compartment_2: compartment_2.to_vec(),
        };

        count + find_duplicate(rucksack_struct)
    });

    println!("{}", count);
}

fn find_duplicate(rucksack: Rucksack) -> u32 {
    let mut items = HashSet::new();

    for item in rucksack.compartment_1 {
        items.insert(item);
    }

    *rucksack
        .compartment_2
        .iter()
        .find(|item| items.contains(item))
        .expect("No duplicate!")
}

#[cfg(test)]
mod tests {
    use crate::get_item_value;

    #[test]
    fn test_get_item_value() {
        assert_eq!(get_item_value('a'), 1);
        assert_eq!(get_item_value('A'), 27);
    }
}

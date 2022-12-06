use clap::Parser;
use std::{collections::HashSet, fs};

#[derive(Parser, Debug)]
struct Args {
    input_file: String,
}

type Rucksack = Vec<u32>;
type Group = (Rucksack, Rucksack, Rucksack);

fn get_item_value(item: char) -> u32 {
    if item >= 'a' && item <= 'z' {
        item.to_digit(36).unwrap() - 9
    } else {
        item.to_digit(36).unwrap() + 17
    }
}

fn main() {
    let data = fs::read_to_string(Args::parse().input_file).expect("This file doesn't exist!");

    let groups = get_groups(data);

    let test = groups
        .iter()
        .map(|group| find_duplicate(group))
        .fold(0, |count, val| count + val);

    println!("{:?}", test)
}

fn get_rucksack(rucksack: String) -> Rucksack {
    rucksack.chars().map(get_item_value).collect::<Vec<u32>>()
}

fn get_groups(data: String) -> Vec<Group> {
    let mut groups: Vec<Group> = vec![];
    let mut lines = data.split("\n").map(|line| line.to_string());

    while let (Some(x), Some(y), Some(z)) = (lines.next(), lines.next(), lines.next()) {
        groups.push((get_rucksack(x), get_rucksack(y), get_rucksack(z)));
    }

    groups
}

fn turn_to_hashset(rucksack: Rucksack) -> HashSet<u32> {
    let mut hashset = HashSet::new();

    for item in rucksack {
        hashset.insert(item);
    }

    hashset
}

fn find_duplicate(group: &Group) -> u32 {
    let hashgroup = (
        turn_to_hashset(group.0.clone()),
        turn_to_hashset(group.1.clone()),
        turn_to_hashset(group.2.clone()),
    );

    for item in hashgroup.0 {
        if hashgroup.1.contains(&item) && hashgroup.2.contains(&item) {
            return item;
        }
    }

    panic!("No badge");
}

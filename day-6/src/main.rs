use std::collections::{vec_deque, HashSet, VecDeque};

use clap::Parser;

#[derive(Parser)]
struct Args {
    buffer: String,
}

fn main() {
    let input: String = Args::parse().buffer;
    let initial = input.chars().take(4).collect::<Vec<char>>();

    let mut deq = VecDeque::from(initial);

    println!("buffer: {input:?}");

    // for (index, char) in input.chars().skip(4).enumerate() {
    //     let set = HashSet::from();
    //     // for char in input {

    //     // }
    // }

    for (index, char) in input.chars().enumerate() {
        let set = HashSet::new();

        set.insert(value)
        // for char in input {

        // }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_true() {
        assert_eq!(true, false);
    }

    #[test]
    fn test_false() {
        assert_eq!(true, false);
    }
}

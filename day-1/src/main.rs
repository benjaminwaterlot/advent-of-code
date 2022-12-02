use clap::Parser;
use std::fs::read_to_string;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Src of the input file
    #[arg(long)]
    input: String,
}

fn main() {
    let args = Args::parse();
    let res = elfs(args.input);
    println!("{}", res);
}

fn elfs(src: String) -> usize {
    let full_str = read_to_string(src).unwrap();

    let mut count_by_elf: Vec<usize> = full_str
        .split("\n\n")
        .map(|elf_str| {
            elf_str.split("\n").fold(0, |sum_calories, calory| {
                str::parse::<usize>(calory).unwrap() + sum_calories
            })
        })
        .collect();

    count_by_elf.sort();
    count_by_elf.into_iter().rev().take(3).sum()
}

#[cfg(test)]
mod tests {
    use crate::elfs;

    #[test]
    fn test_three_stronger_elfs() {
        assert_eq!(elfs("./src/input.txt".to_string()), 206582);
    }
}

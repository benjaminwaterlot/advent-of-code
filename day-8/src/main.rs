use clap::Parser;
use std::{collections::HashMap, fs};

#[derive(Parser)]
struct Args {
    input_file: String,
}

pub fn get_input() -> String {
    let input_file = Args::parse().input_file;
    fs::read_to_string(input_file).expect("This file doesn't exist!")
}

type Tree = isize;
type Grid = HashMap<Coords, Tree>;

#[derive(Eq, Hash, PartialEq, Debug)]
struct Coords {
    x: isize,
    y: isize,
}

#[derive(Debug)]
struct Forest {
    size: isize,
    trees: Grid,
}

impl Forest {
    fn get_tree(&self, coords: &Coords) -> isize {
        *self.trees.get(coords).unwrap()
    }

    fn is_tree_visible(&self, coords: &Coords) -> bool {
        let tree = self.get_tree(coords);

        return (0..coords.x).all(|x| self.get_tree(&Coords { x, y: coords.y }) < tree)
            || ((coords.x + 1)..self.size)
                .all(|x| self.get_tree(&Coords { x, y: coords.y }) < tree)
            || (0..coords.y).all(|y| self.get_tree(&Coords { x: coords.x, y }) < tree)
            || ((coords.y + 1)..self.size)
                .all(|y| self.get_tree(&Coords { x: coords.x, y }) < tree);
    }

    fn get_tree_scenic_score(&self, coords: &Coords) -> isize {
        let mut score = 0;
        let mut temp_score = 0;
        let considered = self.get_tree(coords);

        for x in (0..coords.x).rev() {
            score += 1;

            if self.get_tree(&Coords { x, y: coords.y }) >= considered {
                println!(
                    "{}> {}",
                    self.get_tree(&Coords { x, y: coords.y }),
                    considered
                );
                break;
            }
        }

        for x in (coords.x + 1)..self.size {
            temp_score += 1;

            if self.get_tree(&Coords { x, y: coords.y }) >= considered {
                break;
            }
        }

        score *= temp_score;
        temp_score = 0;

        for y in (0..coords.y).rev() {
            temp_score += 1;

            if self.get_tree(&&Coords { x: coords.x, y }) >= considered {
                break;
            }
        }

        score *= temp_score;
        temp_score = 0;

        for y in ((coords.y + 1)..self.size).rev() {
            temp_score += 1;

            if self.get_tree(&Coords { x: coords.x, y }) >= considered {
                break;
            }
        }

        score *= temp_score;

        return score;
    }

    fn visible_trees_count(&self) -> isize {
        self.trees
            .iter()
            .fold(0, |count, (tree, _)| match self.is_tree_visible(tree) {
                true => count + 1,
                false => count,
            })
    }

    fn from_input(input: String) -> Forest {
        let mut forest = Forest {
            size: input.split("\n").collect::<Vec<&str>>().len() as isize,
            trees: Grid::new(),
        };

        for (y, line) in input.split("\n").map(|line| line.to_string()).enumerate() {
            for (x, char) in line.chars().enumerate() {
                forest.trees.insert(
                    Coords {
                        x: x.try_into().unwrap(),
                        y: y.try_into().unwrap(),
                    },
                    str::parse(&char.to_string()).unwrap(),
                );
            }
        }

        forest
    }
}

fn main() {
    let forest = Forest::from_input(day_8::get_input());

    println!("count:{}", forest.visible_trees_count());
    println!(
        "trees:{}",
        forest.get_tree_scenic_score(&Coords { x: 2, y: 3 })
    );
}

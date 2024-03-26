use crate::entities::{Cell, Grid, Point};
use clap::Parser;
use std::{collections::HashMap, fs};

#[derive(Parser, Debug)]
struct Args {
    input_file: String,
}

pub fn parse_input() -> Grid {
    let input_file = Args::parse().input_file;
    let data = fs::read_to_string(input_file).unwrap();

    let mut hash: HashMap<Point, Cell> = HashMap::new();
    let mut start: Option<Point> = None;
    let mut end: Option<Point> = None;

    for (y, line) in data.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let cell = match c {
                'S' => Cell::START,
                'E' => Cell::END,
                c => Cell::MOUTAIN(c as isize - 'a' as isize),
            };

            let point = Point {
                x: x as isize,
                y: y as isize,
            };

            if cell == Cell::START {
                start = Some(point);
            } else if cell == Cell::END {
                end = Some(point);
            }

            hash.insert(point, cell);
        }
    }

    Grid::new(hash, start.unwrap(), end.unwrap())
}

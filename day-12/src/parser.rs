use crate::{
    args::get_args,
    entities::{Cell, Grid, Point},
};
use clap::Parser;
use std::{collections::HashMap, fs};

#[derive(Parser, Debug)]
struct Args {
    input_file: String,
}
pub trait ToElevation {
    fn to_elevation(self) -> isize;
}

impl ToElevation for char {
    fn to_elevation(self) -> isize {
        self as isize - 'a' as isize
    }
}

pub fn parse_input(start_from_any_plane: bool) -> Grid {
    let data = fs::read_to_string(get_args().input_file).unwrap();

    let width = data.lines().next().unwrap().len();
    let height = data.lines().count();

    println!(
        "Map of width {width} and height {height} ({} cells)",
        width * height
    );

    let mut hash: HashMap<Point, Cell> = HashMap::new();
    let mut starts: Vec<Point> = vec![];
    let mut end: Option<Point> = None;

    for (y, line) in data.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let cell = match c {
                'S' => Cell::START,
                'E' => Cell::END,
                c => Cell::PATH(c.to_elevation()),
            };

            let point = Point {
                x: x as isize,
                y: y as isize,
            };

            match cell {
                Cell::START => starts.push(point),
                Cell::END => end = Some(point),
                Cell::PATH(0) if start_from_any_plane => starts.push(point),
                _ => (),
            };

            hash.insert(point, cell);
        }
    }

    if starts.len() == 0 {
        panic!("No start in this map");
    }

    Grid::new(hash, starts, end.expect("No end in this map"))
}

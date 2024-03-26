mod args;
mod entities;
mod parser;

use std::time::Instant;

use crate::args::get_args;

fn main() {
    let game_grid = parser::parse_input(get_args().start_from_any_plane);

    let now = Instant::now();

    let mut solutions: Vec<isize> = game_grid.get_shortest_path_from_starts_to_end();

    solutions.sort();

    println!("found path {:?} in {:?}", solutions[0], now.elapsed());
}

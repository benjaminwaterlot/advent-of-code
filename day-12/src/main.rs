mod entities;
mod parser;
use pathfinding::prelude::astar;

fn main() {
    let game_grid = parser::parse_input();

    let path = astar(
        &game_grid.start,
        |point| {
            game_grid
                .get_cell_neighbors(*point)
                .into_iter()
                .map(|point| (point, 1))
        },
        |point| point.get_distance_to(game_grid.end),
        |point| *point == game_grid.end,
    );

    println!("{:#?}", path.unwrap().1);
}

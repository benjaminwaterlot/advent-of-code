use std::collections::HashMap;

use pathfinding::directed::astar::astar;

use crate::parser::ToElevation;

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum Cell {
    START,
    PATH(isize),
    END,
}

#[derive(Debug, Eq, Hash, PartialEq, Copy, Clone)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Point {
    pub fn get_distance_to(&self, point: Point) -> isize {
        (self.x - point.x).abs() + (self.x - point.x).abs()
    }
}

#[derive(Debug)]
pub struct Grid {
    data: HashMap<Point, Cell>,
    pub starts: Vec<Point>,
    pub end: Point,
}

impl Grid {
    pub fn new(data: HashMap<Point, Cell>, starts: Vec<Point>, end: Point) -> Grid {
        Grid { data, starts, end }
    }

    pub fn get_cell_neighbors(&self, current: Point) -> Vec<Point> {
        let mut neighbors: Vec<Point> = vec![];

        for (x, y) in [
            (current.x, current.y - 1),
            (current.x, current.y + 1),
            (current.x - 1, current.y),
            (current.x + 1, current.y),
        ] {
            let point = Point { x, y };
            if let Some(cell) = self.data.get(&point) {
                let current_elevation = match self.data.get(&current).unwrap() {
                    Cell::START => 0,
                    Cell::PATH(ele) => *ele,
                    Cell::END => 0,
                };

                match cell {
                    Cell::END => {
                        if 'z'.to_elevation() <= current_elevation + 1 {
                            neighbors.push(point)
                        }
                    }
                    Cell::PATH(elevation) => {
                        if *elevation <= current_elevation + 1 {
                            neighbors.push(point)
                        }
                    }
                    Cell::START => (),
                }
            }
        }

        neighbors
    }

    pub fn get_shortest_path_from_starts_to_end(&self) -> Vec<isize> {
        self.starts
            .iter()
            .map(|start| -> Option<isize> {
                let path = astar(
                    start,
                    |point| {
                        self.get_cell_neighbors(*point)
                            .into_iter()
                            .map(|point| (point, 1))
                    },
                    |point| point.get_distance_to(self.end),
                    |point| *point == self.end,
                );

                match path {
                    Some((_, val)) => Some(val),
                    None => None,
                }
            })
            .filter(|p| *p != None)
            .map(|p| p.unwrap())
            .collect()
    }
}

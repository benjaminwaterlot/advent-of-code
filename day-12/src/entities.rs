use std::collections::HashMap;

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum Cell {
    START,
    MOUTAIN(isize),
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
    pub start: Point,
    pub end: Point,
}

impl Grid {
    pub fn new(data: HashMap<Point, Cell>, start: Point, end: Point) -> Grid {
        Grid { data, start, end }
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
                    Cell::MOUTAIN(ele) => *ele,
                    Cell::END => 0,
                };

                match cell {
                    Cell::END => {
                        if 'z' as isize - 'a' as isize <= current_elevation + 1 {
                            neighbors.push(point)
                        }
                    }
                    Cell::MOUTAIN(elevation) => {
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
}

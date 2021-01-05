enum Cell {
    Dead,
    Alive,
}

struct Coordinate(u32, u32);

pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

impl Universe {
    pub fn new((width, height): (u32, u32)) -> Self {
        let cells = (0..width * height)
            .map(|i| {
                if i % 2 == 0 || i % 7 == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();

        Self {
            width,
            height,
            cells
        }
    }

    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;

        let north = if row == 0 {
            self.height - 1
        } else {
            row - 1
        };

        let south = if row == self.height - 1 {
            0
        } else {
            row + 1
        };

        let west = if column == 0 {
            self.width - 1
        } else {
            column - 1
        };

        let east = if column == self.width - 1 {
            0
        } else {
            column + 1
        };

        let nw = self.get_index(north, west);
        count += self.cells[nw] as u8;

        let n = self.get_index(north, column);
        count += self.cells[n] as u8;

        let ne = self.get_index(north, east);
        count += self.cells[ne] as u8;

        let w = self.get_index(row, west);
        count += self.cells[w] as u8;

        let e = self.get_index(row, east);
        count += self.cells[e] as u8;

        let sw = self.get_index(south, west);
        count += self.cells[sw] as u8;

        let s = self.get_index(south, column);
        count += self.cells[s] as u8;

        let se = self.get_index(south, east);
        count += self.cells[se] as u8;

        count
    }

    fn neighbors(&self, coords: Coordinate) -> [Coordinate; 8] {
        let right = Coordinate(coords.x + 1, coords.y);
        let bottom = Coordinate(coords.x, coords.y + 1);
        let bottom_right = Coordinate(coords.x + 1, coords.y + 1);
        let left = Coordinate(coords.x - 1, coords.y);
        let top = Coordinate(coords.x, coords.y - 1);
        let top_left = Coordinate(coords.x - 1, coords.y - 1);
        let top_right = Coordinate(coords.x + 1, coords.y - 1);
        let bottom_left = Coordinate(coords.x - 1, coords.y + 1);
        [
            top_left,
            top,
            top_right,
            left,
            right,
            bottom_left,
            bottom,
            bottom_right,
        ]
    }

    fn index_from_coord(&self, coord: Coordinate) -> u32 {
        coord.0 + self.width * coord.1
    }

    fn coord_from_index(&self, index: u32) -> Coordinate {
        Coordinate(index % self.width, index / self.width)
    }
}


fn main() {
    let universe = Universe::new((64, 64));
    universe.tick();
}

mod tests {
    use crate::{Universe, Coordinate};

    #[test]
    fn check_coord_from_index() {
        let universe = Universe::new((64, 64));
        assert_eq!(universe.coord_from_index(0).x, 0);
        assert_eq!(universe.coord_from_index(0).y, 0);
    }

    #[test]
    fn check_index_from_coord() {
        let universe = Universe::new((64, 64));
        assert_eq!(universe.index_from_coord(Coordinate { x: 0, y: 0 }), 0);
        assert_eq!(universe.index_from_coord(Coordinate { x: 1, y: 0 }), 1);
        assert_eq!(universe.index_from_coord(Coordinate { x: 0, y: 1 }), 64);
    }
}

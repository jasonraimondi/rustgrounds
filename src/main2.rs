enum Cell {
    Dead,
    Alive,
}

#[derive(Clone, Copy)]
struct Coordinate {
    x: u32,
    y: u32,
}

pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

impl Universe {
    pub fn new((width, height): (u32, u32)) -> Universe {
        let cells = (0..width * height)
            .map(|i| {
                if i % 2 == 0 || i % 7 == 0 {
                    Cell::Alive
                } else {
                    Cell::Dead
                }
            })
            .collect();

        Universe {
            width,
            height,
            cells,
        }
    }

    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let coord = Coordinate { x: row, y: col };
                let idx = self.get_index(coord);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(coord);

                let next_cell = match (cell, live_neighbors) {
                    // Rule 1: Any live cell with fewer than two live neighbours
                    // dies, as if caused by underpopulation.
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    // Rule 2: Any live cell with two or three live neighbours
                    // lives on to the next generation.
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    // Rule 3: Any live cell with more than three live
                    // neighbours dies, as if by overpopulation.
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    // Rule 4: Any dead cell with exactly three live neighbours
                    // becomes a live cell, as if by reproduction.
                    (Cell::Dead, 3) => Cell::Alive,
                    // All other cells remain in the same state.
                    (otherwise, _) => otherwise,
                };

                next[idx] = next_cell;
            }
        }

        self.cells = next;
    }

    fn live_neighbor_count(&self, coord: Coordinate) -> u32 {
        let mut count: u32 = 0;
        for delta_row in [self.height - 1, 0, 1].iter().cloned() {
            for delta_col in [self.width - 1, 0, 1].iter().cloned() {
                if delta_row == 0 && delta_col == 0 {
                    continue;
                }
                let idx = self.get_index(Coordinate {
                    x: (row + delta_row) % self.height,
                    y: (column + delta_col) % self.width,
                });
                println!("idx {}", idx)
                // count += self.cells[idx] as u8;
            }
        }
        count
    }

    fn get_index(&self, coord: Coordinate) -> u32 {
        coord.x + self.width * coord.y
    }

    // fn coord_from_index(&self, index: u32) -> Coordinate {
    //     Coordinate {
    //         x: index % self.width,
    //         y: index / self.width,
    //     }
    // }
}


fn main() {
    let mut universe = Universe::new((64, 64));
    universe.tick();
    universe.tick();
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
        assert_eq!(universe.get_index(Coordinate { x: 0, y: 0 }), 0);
        assert_eq!(universe.get_index(Coordinate { x: 1, y: 0 }), 1);
        assert_eq!(universe.get_index(Coordinate { x: 0, y: 1 }), 64);
    }
}

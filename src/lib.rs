#[macro_use] mod utils;
mod timer;
extern crate js_sys;
use wasm_bindgen::prelude::*;
use std::fmt;
use crate::timer::Timer;
use dubble::DoubleBuffered;
// use wasm_bindgen::__rt::core::fmt::{Display, Formatter, Error};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Cell {
    Dead = 0,
    Alive = 1
}

impl Cell {
    pub fn toggle(&mut self) {
        *self = match *self {
            Cell::Dead => Cell::Alive,
            Cell::Alive => Cell::Dead
        }
    }
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: DoubleBuffered<Vec<Cell>>
}

impl Universe {
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }
    /*
    * Heavy usage. with modulo. change the implementation with rather efficient.
    */
    // fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
    //     let mut count = 0;
    //     for delta_row in [self.height -1, 0, 1].iter().cloned() {
    //         for delta_col in [self.width -1, 0, 1].iter().cloned() {
    //             if delta_row == 0 && delta_col == 0 {
    //                 continue;
    //             }
    //
    //             let neighbor_row = (row + delta_row) % self.height;
    //             let neighbor_col = (column + delta_col) % self.width;
    //             let idx = self.get_index(neighbor_row, neighbor_col);
    //             count += self.cells[idx] as u8;
    //         }
    //     }
    //     count
    // }
    fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
        let mut count = 0;
        let north = if row == 0 {
            self.height -1
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
        // log!("Alright! {}", nw);
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
}

//Public method exported to Javascript.
#[wasm_bindgen]
impl Universe {
    pub fn tick(&mut self) {
        let _timer = Timer::new("Universe::tick");
        {
            let _timer = Timer::new("new generation");
            for row in 0..self.height {
                for col in 0..self.width {
                    let idx = self.get_index(row, col);
                    let cell = self.cells.get(idx);
                    let live_neighbors = self.live_neighbor_count(row,col);

                    // next.set(idx, match (cell, live_neighbors) {
                    //     (true, x) if x < 2 => false,
                    //     (true, 2) | (true, 3) => true,
                    //     (true, x) if x > 3 => false,
                    //     (false, 3) => true,
                    //     (otherwise, _) => otherwise
                    // });
                    let next_cell = match (cell, live_neighbors) {
                        // Rule 1: Any live cell with fewer than two live neighbours
                        // dies, as if caused by underpopulation.
                        (Some(c), x) if *c == Cell::Alive &&  x < 2 => Cell::Dead,
                        // Rule 2: Any live cell with two or three live neighbours
                        // lives on to the next generation.
                        (Some(c), 2) | (Some(c), 3) if *c == Cell::Alive => Cell::Alive,
                        // Rule 3: Any live cell with more than three live
                        // neighbours dies, as if by overpopulation.
                        (Some(c), x) if *c == Cell::Alive && x > 3 => Cell::Dead,
                        // Rule 4: Any dead cell with exactly three live neighbours
                        // becomes a live cell, as if by reproduction.
                        (Some(c), 3) if *c == Cell::Dead => Cell::Alive,
                        // All other cells remai
                        // n in the same state.
                        (_otherwise, _) => Cell::Dead,
                    };
                    self.cells[idx] = next_cell;
                }
            }
        }
        // log!("  Deii  it becomes {:?}", next);
        let _timer = Timer::new("free old cells");
        self.cells.update();
    }
}

// To make stuffs human Readable. let's impl Display.

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead { '◻' } else {'◼'};
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n");
        }
        Ok(())
    }
}
#[wasm_bindgen]
impl Universe {
    pub fn new() -> Universe {
        utils::set_panic_hook();
        let width = 128;
        let height = 128;

        // let size = (width * height) as usize;
        // let mut cells = FixedBitSet::with_capacity(size);
        //
        // for i in 0..size {
        //     cells.set(i, js_sys::Math::random() < 0.5);
        // }
        let mut cells = DoubleBuffered::construct_with(Vec::<Cell>::new);
        for _c in 0..(width * height) {
            if js_sys::Math::random() < 0.5 {
                cells.push(Cell::Alive);
            } else {
                cells.push(Cell::Dead);
            }
        }
        // log!("Did you panick heres? {}" , cells.len());
        cells.update();
        // log!("Did you panick heres? {}" , cells.len());
        Universe {
            width, height, cells
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    pub fn render(&self) -> String {
        self.to_string() //THis is possible because impl the Display trait with fmt. and hence to_string()!
    }

    /// Set the width of the universe.
    ///
    /// Resets all cells to the dead state.
    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        self.cells.clear();
        for _c in 0..(width * self.height) {
            self.cells.push(Cell::Dead);
        }
    }

    /// Set the height of the universe.
    ///
    /// Resets all cells to the dead state.
    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        self.cells.clear();
        for _c in 0..(self.width * height) {
            self.cells.push(Cell::Dead);
        }
        // (0..self.width * height).map(|_i| self.cells.push(Cell::Dead));
        self.cells.update();
    }

    pub fn toggle_cell(&mut self, row: u32, column: u32) {
        let idx = self.get_index(row, column);
        if let Some(cell) = self.cells.get_mut(idx) {
            cell.toggle();
        }
        self.cells.update();
    }

    pub fn reset_cells(&mut self) {
        self.cells.clear();
        for _ in 0..self.cells.len() {
            self.cells.push(Cell::Dead);
        }
        // (0..self.width * self.height).map(
        //     |_i| self.cells.push(Cell::Dead)
        // );
        self.cells.update();
    }
}

impl Universe {
    pub fn get_cells(&self) -> &[Cell] {
        self.cells.as_ref()
    }
    /// Set cells to be alive in a universe by passing the row and column
        /// of each cell as an array.
    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        for (row, col) in cells.iter().cloned() {
            let idx = self.get_index(row, col);
            if let Some(cell) = self.cells.get_mut(idx) {
                *cell = Cell::Alive.to_owned();
            }
            self.cells.update();
        }
    }
}
//Rust-generated WebAssembly functions cannot return borrowed references.

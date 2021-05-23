mod utils;

extern crate web_sys;
extern crate js_sys;

use wasm_bindgen::prelude::*;
use std::fmt;

/*
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}
*/

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// Each cell is represented as a single byte
#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

impl Cell {
    fn toggle(&mut self) {
        *self = match *self {
            Cell::Dead => Cell::Alive,
            Cell::Alive => Cell::Dead,
        }
    }
}

/// Universe struct, exported to JavaScript
#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

/// Public methods, exported to JavaScript.
#[wasm_bindgen]
impl Universe {
    /// Get the 2D grid index given row and col
    fn get_index(&self, row: u32, col: u32) -> usize {
        (row * self.width + col) as usize
    }

    /// Get number of live neighbors for a given Cell position
    fn live_neighbor_count(&self, row: u32, col: u32) -> u8 {
        let mut count = 0;
        for drow in [self.height - 1, 0, 1].iter().cloned() {
            for dcol in [self.width - 1, 0, 1].iter().cloned() {
                if drow == 0 && dcol == 0 {
                    continue;
                }

                let nrow = (row + drow) % self.height;
                let ncol = (col + dcol) % self.width;
                let idx = self.get_index(nrow, ncol);
                count += self.cells[idx] as u8;
            }
        }
        count
    }

    /// Represents the logic of an epoch in the Universe
    pub fn tick(&mut self) {
        let mut next = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let idx = self.get_index(row, col);
                let cell = self.cells[idx];
                let live_neighbors = self.live_neighbor_count(row, col);

                /* 
                log!(
                    "cell[{}, {}] is initially {:?} and has {} live neighbors",
                    row,
                    col,
                    cell,
                    live_neighbors
                );
                */

                let next_cell = match (cell, live_neighbors) {
                    (Cell::Alive, x) if x < 2 => Cell::Dead,
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    (Cell::Alive, x) if x > 3 => Cell::Dead,
                    (Cell::Dead, 3) => Cell::Alive,
                    (otherwise, _) => otherwise,
                };

                // log!("      it becomes {:?}", next_cell);

                next[idx] = next_cell;
            }
        }

        self.cells = next;
    }

    /// Create a new Universe with initial live cells
    pub fn new() -> Universe {
        utils::set_panic_hook();
        let width = 40;
        let height = 40;
        let size = (width * height) as usize;
        let cells = (0..size).map(|_i| Cell::Dead).collect();

        Universe {
            width,
            height,
            cells,
        }
    }

    pub fn random(&mut self) {
        let size = (self.width * self.height) as usize;
        self.cells = (0..size).map(|_i| {
            if js_sys::Math::random() < 0.5 {
                return Cell::Alive;
            } else {
                return Cell::Dead;
            }
        }).collect()
    }
    
    /// Purge cells in Universe
    pub fn purge(&mut self) {
        let size = (self.width * self.height) as usize;
        self.cells = (0..size).map(|_i| Cell::Dead).collect();
    }
    
    /// Reset cells in Universe
    pub fn reset(&mut self) {
        let size = (self.width * self.height) as usize;
        self.cells = (0..size).map(|_i| Cell::Dead).collect();
    }

    /// Helper for displaying Universe
    pub fn render(&self) -> String {
        self.to_string()
    }

    /// Width of the Universe
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Modify the width of the Universe and reset the grid
    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        self.cells = (0..width * self.height).map(|_i| Cell::Dead).collect();
    }

    /// Height of the Universe
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Modify the height of the Universe and reset the grid
    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        self.cells = (0..self.width * height).map(|_i| Cell::Dead).collect();
    }

    /// Retrieve all cells in Universe
    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    /// Change the state of a cell to its opposite state
    pub fn toggle_cell(&mut self, row: u32, col: u32) {
        let idx = self.get_index(row, col);
        self.cells[idx].toggle();
    }
}

impl Universe {
    /// Get the dead and alive values of the whole universe
    pub fn get_cells(&self) -> &[Cell] {
        &self.cells
    }

    /// Set cells to be alive in a universe by passing the row and col
    /// of each cell as an array
    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        for (row, col) in cells.iter().cloned() {
            let idx = self.get_index(row, col);
            self.cells[idx] = Cell::Alive;
        }
    }
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead { '◻' } else { '◼' };
                write!(f, "{}", symbol)?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

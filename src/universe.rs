use crate::{cell::Cell, timer::Timer, utils};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Universe {
  width: u32,
  height: u32,
  cells: Vec<Cell>,
}

#[wasm_bindgen]
impl Universe {
  // constructor for a new Universe
  pub fn new() -> Universe {
    utils::set_panic_hook();

    let width = 64;
    let height = 64;

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

  // get the width of the universe
  pub fn width(&self) -> u32 {
    self.width
  }

  // get the height of the universe
  pub fn height(&self) -> u32 {
    self.height
  }

  // get the pointer to the universe's cells array
  pub fn cells(&self) -> *const Cell {
    self.cells.as_ptr()
  }

  pub fn set_width(&mut self, width: u32) {
    self.width = width;
    self.cells = (0..width * self.height).map(|_i| Cell::Dead).collect();
  }

  pub fn set_height(&mut self, height: u32) {
    self.height = height;
    self.cells = (0..self.width * height).map(|_i| Cell::Dead).collect();
  }

  pub fn toggle_cell(&mut self, row: u32, column: u32) {
    let idx = self.get_index(row, column);
    self.cells[idx].toggle();
  }

  pub fn tick(&mut self) {
    let _timer = Timer::new("Universe::tick");
    let mut next = {
      let _timer = Timer::new("allocate next cells");
      self.cells.clone()
    };

    {
      let _timer = Timer::new("new generation");
      for row in 0..self.height {
        for col in 0..self.width {
          let idx = self.get_index(row, col);
          let cell = self.cells[idx];
          let live_neighbors = self.live_neighbor_count(row, col);
          let next_cell = match (cell, live_neighbors) {
            (Cell::Alive, x) if x < 2 => Cell::Dead,
            (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
            (Cell::Alive, x) if x > 3 => Cell::Dead,
            (Cell::Dead, 3) => Cell::Alive,
            (otherwise, _) => otherwise,
          };
          next[idx] = next_cell;
        }
      }
    }

    let _timer = Timer::new("free old cells");
    self.cells = next;
  }
}

impl Universe {
  fn get_index(&self, row: u32, column: u32) -> usize {
    (row * self.width + column) as usize
  }

  fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
    let mut count = 0;

    let north = if row == 0 { self.height - 1 } else { row - 1 };

    let south = if row == self.height - 1 { 0 } else { row + 1 };

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

  pub fn get_cells(&self) -> &[Cell] {
    &self.cells
  }

  pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
    for (row, col) in cells.iter().cloned() {
      let idx = self.get_index(row, col);
      self.cells[idx] = Cell::Alive;
    }
  }
}

use crate::cell::Cell;
use std::collections::HashSet;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub struct Universe {
  width: u32,
  height: u32,
  cells: Vec<Cell>,
  alive: HashSet<(u32, u32)>,
}

#[wasm_bindgen]
impl Universe {
  // constructor for a new Universe
  pub fn new(size: u32) -> Universe {
    let width = size;
    let height = size;

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
      alive: HashSet::new(),
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

  pub fn toggle_cell(&mut self, row: u32, column: u32) {
    let idx = self.get_index(row, column);
    self.cells[idx].toggle();
  }
}

impl Default for Universe {
  fn default() -> Self {
    Universe::new(64)
  }
}

impl Universe {
  pub fn alive_cells(&self) -> &HashSet<(u32, u32)> {
    &self.alive
  }

  pub fn cell(&self, idx: usize) -> Cell {
    self.cells[idx]
  }

  pub fn get_index(&self, row: u32, column: u32) -> usize {
    (row * self.width + column) as usize
  }

  fn live_neighbor_count(&self, row: u32, column: u32) -> u8 {
    let mut count = 0;

    for delta_row in [self.height - 1, 0, 1].iter().cloned() {
      for delta_col in [self.width - 1, 0, 1].iter().cloned() {
        if delta_row == 0 && delta_col == 0 {
          continue;
        }

        let neighbor_row = (row + delta_row) % self.height;
        let neighbor_col = (column + delta_col) % self.width;
        let idx = self.get_index(neighbor_row, neighbor_col);

        count += self.cells[idx] as u8;
      }
    }

    count
  }

  pub fn tick(&mut self) {
    let mut next = self.cells.clone();

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

        if next_cell != self.cells[idx] {
          match next_cell {
            Cell::Alive => self.alive.insert((row, col)),
            Cell::Dead => self.alive.remove(&(row, col)),
          };
        }

        next[idx] = next_cell;
      }
    }

    self.cells = next;
  }

  pub fn tick_many(&mut self, steps: u32) {
    for _ in 0..steps {
      self.tick();
    }
  }
}

use crate::{shaders::*, universe::Universe};
use std::collections::HashSet;
use wasm_bindgen::prelude::*;
use web_sys::HtmlCanvasElement;

pub mod cell;
pub mod shaders;
pub mod universe;

const CELL_SIZE: u32 = 6;

fn compute_draw_cells_webgl(changes: &HashSet<(u32, u32)>) -> Vec<f32> {
  let mut vertices = Vec::new();
  let fcs = CELL_SIZE as f32;

  for &(row, col) in changes {
    let scaled = |idx: u32| (idx as f32) * (fcs + 1.) + 1.;

    let v0 = vec![
      scaled(col),
      scaled(row),
      scaled(col) + fcs,
      scaled(row),
      scaled(col) + fcs,
      scaled(row) + fcs,
      scaled(col),
      scaled(row) + fcs,
    ];

    v0.into_iter().for_each(|v| vertices.push(v));
  }

  vertices
}

#[wasm_bindgen(js_name = "setupWebgl")]
pub fn setup_webgl() -> Result<(), JsValue> {
  setup_shaders()?;

  Ok(())
}

#[wasm_bindgen(js_name = "animationWebgl")]
pub fn animation_webgl(universe: &mut Universe, ticks: u32) -> Result<(), JsValue> {
  universe.tick_many(ticks);

  let vertices = compute_draw_cells_webgl(universe.alive_cells());

  render_pipeline(&vertices)
}

#[wasm_bindgen(js_name = "getCellSize")]
pub fn get_cell_size() -> Result<u32, JsValue> {
  Ok(CELL_SIZE)
}

#[wasm_bindgen(js_name = "setupCanvas")]
pub fn setup_canvas(universe: &Universe) -> Result<(), JsValue> {
  let canvas: HtmlCanvasElement =
    get_canvas().ok_or_else(|| JsValue::from_str("Failed getting canvas"))?;

  canvas.set_width((CELL_SIZE + 1) * universe.width() + 1);
  canvas.set_height((CELL_SIZE + 1) * universe.height() + 1);

  Ok(())
}

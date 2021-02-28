use glsmrs::{GlState, Program};
use wasm_bindgen::prelude::*;

mod shaders;

#[wasm_bindgen(js_name = "animationWebgl")]
pub fn animation_webgl(
  program: &mut Program,
  compute_program: &Program,
  state: &mut GlState,
) -> Result<(), JsValue> {
  shaders::render_pipeline(program, compute_program, state)
}

#[wasm_bindgen(js_name = "setupComputeProgram")]
pub fn setup_compute_program() -> Result<Program, JsValue> {
  shaders::setup_compute_program()
}

#[wasm_bindgen(js_name = "setupInitProgram")]
pub fn setup_init_program() -> Result<Program, JsValue> {
  shaders::setup_display_program()
}

#[wasm_bindgen(js_name = "setupWebgl")]
pub fn setup_webgl() -> Result<GlState, JsValue> {
  shaders::setup_shaders()
}

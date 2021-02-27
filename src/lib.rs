use crate::{cell::Cell, universe::Universe};
use std::collections::HashSet;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{
  window, CanvasRenderingContext2d, HtmlCanvasElement, WebGlProgram, WebGlRenderingContext,
  WebGlShader,
};

pub mod cell;
pub mod timer;
pub mod universe;

const CELL_SIZE: u32 = 6;

fn get_canvas() -> Option<HtmlCanvasElement> {
  let document = window()?.document()?;
  let canvas = document.get_element_by_id("game-of-life-canvas")?;

  canvas.dyn_into::<HtmlCanvasElement>().ok()
}

fn get_ctx<T: JsCast>(ctx_name: &str) -> Result<T, JsValue> {
  let ctx = get_canvas()
    .ok_or_else(|| JsValue::from_str("Failed to get canvas"))?
    .get_context(ctx_name)?
    .ok_or_else(|| JsValue::from_str("Failed getting ctx"))?;

  ctx.dyn_into::<T>().map_err(JsValue::from)
}

fn draw_grid(ctx: &CanvasRenderingContext2d, universe: &Universe) -> Result<(), JsValue> {
  ctx.begin_path();
  ctx.set_stroke_style(&JsValue::from_str("gray"));

  let float_width = universe.width() as f64;
  let float_height = universe.height() as f64;
  let float_cell_size = CELL_SIZE as f64;

  for i in 0..universe.width() + 1 {
    let fi = i as f64;
    ctx.move_to(fi * (float_cell_size + 1.) + 1., 0.);
    ctx.line_to(
      fi * (float_cell_size + 1.) + 1.,
      (float_cell_size + 1.) * float_height + 1.,
    );
  }

  for j in 0..universe.height() + 1 {
    let fj = j as f64;
    ctx.move_to(0., fj * (float_cell_size + 1.) + 1.);
    ctx.line_to(
      (float_cell_size + 1.) * float_width + 1.,
      fj * (float_cell_size + 1.) + 1.,
    );
  }

  ctx.stroke();

  Ok(())
}

fn draw_cells(
  ctx: &CanvasRenderingContext2d,
  universe: &Universe,
  changes: HashSet<(u32, u32)>,
) -> Result<(), JsValue> {
  let float_cell_size = CELL_SIZE as f64;

  ctx.begin_path();

  for (row, col) in changes {
    let idx = universe.get_index(row, col);

    let stroke_style = match universe.cell(idx) {
      Cell::Dead => "white",
      Cell::Alive => "black",
    };

    ctx.set_fill_style(&JsValue::from(stroke_style));
    ctx.fill_rect(
      (col as f64) * (float_cell_size + 1.) + 1.,
      (row as f64) * (float_cell_size + 1.) + 1.,
      float_cell_size,
      float_cell_size,
    );
  }

  ctx.stroke();

  Ok(())
}

fn compile_shader(
  ctx: &WebGlRenderingContext,
  shader_type: u32,
  source: &str,
) -> Result<WebGlShader, String> {
  let shader = ctx
    .create_shader(shader_type)
    .ok_or_else(|| String::from("Unable to create shader object"))?;
  ctx.shader_source(&shader, source);
  ctx.compile_shader(&shader);

  if ctx
    .get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS)
    .as_bool()
    .unwrap_or(false)
  {
    Ok(shader)
  } else {
    Err(
      ctx
        .get_shader_info_log(&shader)
        .unwrap_or_else(|| String::from("Unknown error creating shader")),
    )
  }
}

fn link_program(
  ctx: &WebGlRenderingContext,
  vert_shader: &WebGlShader,
  frag_shader: &WebGlShader,
) -> Result<WebGlProgram, String> {
  let program = ctx
    .create_program()
    .ok_or_else(|| String::from("Unable to create shader object"))?;

  ctx.attach_shader(&program, vert_shader);
  ctx.attach_shader(&program, frag_shader);
  ctx.link_program(&program);

  if ctx
    .get_program_parameter(&program, WebGlRenderingContext::LINK_STATUS)
    .as_bool()
    .unwrap_or(false)
  {
    Ok(program)
  } else {
    Err(
      ctx
        .get_program_info_log(&program)
        .unwrap_or_else(|| String::from("Unknown error creating program object")),
    )
  }
}

#[wasm_bindgen(js_name = "animationLoop")]
pub fn animation_loop(universe: &mut Universe, ticks: u32) -> Result<(), JsValue> {
  let ctx = get_ctx("2d")?;
  let changes = universe.tick_many(ticks);

  draw_cells(&ctx, &universe, changes)?;
  draw_grid(&ctx, &universe)?;

  Ok(())
}

fn setup_shaders() -> Result<WebGlRenderingContext, JsValue> {
  let ctx = get_ctx("webgl")?;

  let vert_shader = compile_shader(
    &ctx,
    WebGlRenderingContext::VERTEX_SHADER,
    r#"
      precision highp float;
      uniform float uCol;
      attribute vec4 position;
      void main() {
        gl_Position = position;
      }
    "#,
  )?;

  let frag_shader = compile_shader(
    &ctx,
    WebGlRenderingContext::FRAGMENT_SHADER,
    r#"
      precision highp float;
      uniform float uCol;
      void main() {
        gl_FragColor = vec4(uCol * 1.0, 1.0, 1.0, 1.0);
      }
    "#,
  )?;
  let program = link_program(&ctx, &vert_shader, &frag_shader)?;

  ctx.use_program(Some(&program));

  let u_color = ctx
    .get_uniform_location(&program, "uCol")
    .ok_or_else(|| JsValue::from(format!("Failed to get uniform uCol: {}", ctx.get_error())))?;

  ctx.uniform1f(Some(&u_color), 0.5);

  Ok(ctx)
}

#[wasm_bindgen(js_name = "animationWebgl")]
pub fn animation_webgl() -> Result<(), JsValue> {
  let ctx = setup_shaders()?;
  let vertices: [f32; 9] = [-0.7, -0.7, 0.0, 0.7, -0.7, 0.0, 0.0, 0.7, 0.0];

  let buffer = ctx.create_buffer().ok_or("failed to create buffer")?;
  ctx.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));

  unsafe {
    let vert_array = js_sys::Float32Array::view(&vertices);

    ctx.buffer_data_with_array_buffer_view(
      WebGlRenderingContext::ARRAY_BUFFER,
      &vert_array,
      WebGlRenderingContext::STATIC_DRAW,
    );
  }

  ctx.vertex_attrib_pointer_with_i32(0, 3, WebGlRenderingContext::FLOAT, false, 0, 0);
  ctx.enable_vertex_attrib_array(0);
  ctx.clear_color(0.0, 0.0, 0.0, 1.0);
  ctx.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);
  ctx.draw_arrays(
    WebGlRenderingContext::TRIANGLES,
    0,
    (vertices.len() / 3) as i32,
  );

  Ok(())
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

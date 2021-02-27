use crate::universe::Universe;
use std::collections::HashSet;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{window, HtmlCanvasElement, WebGlProgram, WebGlRenderingContext, WebGlShader};

pub mod cell;
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

fn setup_shaders() -> Result<WebGlRenderingContext, JsValue> {
  let canvas = get_canvas().ok_or_else(|| JsValue::from_str("Failed to get canvas"))?;
  let context = get_ctx("webgl")?;

  let vert_shader = compile_shader(
    &context,
    WebGlRenderingContext::VERTEX_SHADER,
    r#"
        precision highp float;
        attribute vec2 position;
        uniform vec2 canvasSize;
        void main() {
            vec2 zeroOne = position / canvasSize;
            vec2 clipSpace = zeroOne * 2.0 - 1.0;
            gl_Position = vec4(clipSpace * vec2(1, -1), 0, 1);
        }
    "#,
  )?;
  let frag_shader = compile_shader(
    &context,
    WebGlRenderingContext::FRAGMENT_SHADER,
    r#"
        precision highp float;
        void main() {
            gl_FragColor = vec4(1.0, 1.0, 1.0, 1.0);
        }
    "#,
  )?;
  let program = link_program(&context, &vert_shader, &frag_shader)?;
  context.use_program(Some(&program));

  let canvas_size = context
    .get_uniform_location(&program, "canvasSize")
    .ok_or_else(|| {
      JsValue::from(format!(
        "Failed to get uniform uCol: {}",
        context.get_error()
      ))
    })?;

  context.uniform2f(
    Some(&canvas_size),
    canvas.width() as f32,
    canvas.height() as f32,
  );

  Ok(context)
}

fn compute_draw_cells_webgl(
  universe: &universe::Universe,
  changes: &HashSet<(u32, u32)>,
) -> Vec<f32> {
  let mut vertices = Vec::new();
  let fcs = CELL_SIZE as f32;

  for (row, col) in changes {
    let idx = universe.get_index(*row, *col);

    let scaled = |idx: u32| (idx as f32) * (fcs + 1.) + 1.;

    let v0 = vec![
      scaled(*col),
      scaled(*row),
      scaled(*col) + fcs,
      scaled(*row),
      scaled(*col) + fcs,
      scaled(*row) + fcs,
      scaled(*col),
      scaled(*row) + fcs,
    ];

    v0.into_iter().for_each(|v| vertices.push(v));
  }

  vertices
}

#[wasm_bindgen(js_name = "animationWebgl")]
pub fn animation_webgl(universe: &mut Universe) -> Result<(), JsValue> {
  let ctx = setup_shaders()?;

  universe.tick_many(1);

  let vertices = compute_draw_cells_webgl(&universe, universe.alive_cells());
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

  ctx.vertex_attrib_pointer_with_i32(0, 2, WebGlRenderingContext::FLOAT, false, 0, 0);
  ctx.enable_vertex_attrib_array(0);
  ctx.clear_color(0.0, 0.0, 0.0, 1.0);
  ctx.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);

  for c in (0..vertices.len()).step_by(8) {
    ctx.draw_arrays(
      WebGlRenderingContext::TRIANGLE_FAN,
      c as i32,
      (8 / 2) as i32,
    );
  }

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

use glsmrs as gl;
use js_sys;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::WebGlRenderingContext as WebGl;

pub fn get_canvas() -> Option<web_sys::HtmlCanvasElement> {
  let document = web_sys::window()?.document()?;
  let canvas = document.get_element_by_id("game-of-life-canvas")?;

  canvas.dyn_into::<web_sys::HtmlCanvasElement>().ok()
}

fn get_ctx<T: JsCast>(ctx_name: &str) -> Result<T, JsValue> {
  let ctx = get_canvas()
    .ok_or_else(|| JsValue::from_str("Failed to get canvas"))?
    .get_context(ctx_name)?
    .ok_or_else(|| JsValue::from_str("Failed getting ctx"))?;

  ctx.dyn_into::<T>().map_err(JsValue::from)
}

pub fn setup_shaders(w: u32, h: u32) -> Result<gl::GlState, JsValue> {
  let canvas = get_canvas().ok_or_else(|| JsValue::from_str("Failed to get canvas"))?;
  let context: WebGl = get_ctx("webgl")?;

  let (vertices, uvs, indices) = make_quad();

  let mut state = gl::GlState::new(
    &context,
    gl::Viewport {
      w: canvas.width(),
      h: canvas.height(),
    },
  );

  let packf32 = |v: &[f32]| {
    v.iter()
      .flat_map(|el| el.to_ne_bytes().to_vec())
      .collect::<Vec<u8>>()
  };
  let packu16 = |v: &[u16]| {
    v.iter()
      .flat_map(|el| el.to_ne_bytes().to_vec())
      .collect::<Vec<u8>>()
  };
  let packu32 = |v: &[u32]| {
    v.iter()
      .flat_map(|el| el.to_ne_bytes().to_vec())
      .collect::<Vec<u8>>()
  };

  let tex_state = (0..w * h)
    .map(|idx: u32| {
      if js_sys::Math::random() > 0.9 || idx % 7 == 0 {
        1
      } else {
        0
      }
    })
    .collect::<Vec<u32>>();

  state
    .vertex_buffer("position", packf32(&vertices).as_slice())?
    .vertex_buffer("uv", packf32(&uvs).as_slice())?
    .element_buffer(packu16(&indices).as_slice())?
    .texture("display", Some(packu32(&tex_state).as_slice()), w, h)?
    .texture("state", None, w, h)?;

  Ok(state)
}

pub fn setup_display_monochrome_program() -> Result<gl::Program, JsValue> {
  let context: WebGl = get_ctx("webgl")?;

  gl::Program::new(
    &context,
    include_str!("../shaders/dummy.vert"),
    include_str!("../shaders/display_monochrome.frag"),
    vec![gl::UniformDescription::new(
      "state",
      gl::UniformType::Sampler2D,
    )],
    vec![
      gl::AttributeDescription::new("position", gl::AttributeType::Vector2),
      gl::AttributeDescription::new("uv", gl::AttributeType::Vector2),
    ],
  )
  .map_err(|e| JsValue::from(e))
}

pub fn setup_display_program() -> Result<gl::Program, JsValue> {
  let context: WebGl = get_ctx("webgl")?;

  gl::Program::new(
    &context,
    include_str!("../shaders/dummy.vert"),
    include_str!("../shaders/display.frag"),
    vec![gl::UniformDescription::new(
      "state",
      gl::UniformType::Sampler2D,
    )],
    vec![
      gl::AttributeDescription::new("position", gl::AttributeType::Vector2),
      gl::AttributeDescription::new("uv", gl::AttributeType::Vector2),
    ],
  )
  .map_err(JsValue::from)
}

pub fn setup_copy_program() -> Result<gl::Program, JsValue> {
  let context: WebGl = get_ctx("webgl")?;

  gl::Program::new(
    &context,
    include_str!("../shaders/dummy.vert"),
    include_str!("../shaders/copy.frag"),
    vec![gl::UniformDescription::new(
      "state",
      gl::UniformType::Sampler2D,
    )],
    vec![
      gl::AttributeDescription::new("position", gl::AttributeType::Vector2),
      gl::AttributeDescription::new("uv", gl::AttributeType::Vector2),
    ],
  )
  .map_err(JsValue::from)
}

pub fn setup_compute_program() -> Result<gl::Program, JsValue> {
  let context: WebGl = get_ctx("webgl")?;

  gl::Program::new(
    &context,
    include_str!("../shaders/dummy.vert"),
    include_str!("../shaders/compute.frag"),
    vec![
      gl::UniformDescription::new("state", gl::UniformType::Sampler2D),
      gl::UniformDescription::new("canvasSize", gl::UniformType::Vector2),
    ],
    vec![
      gl::AttributeDescription::new("position", gl::AttributeType::Vector2),
      gl::AttributeDescription::new("uv", gl::AttributeType::Vector2),
    ],
  )
  .map_err(JsValue::from)
}

pub fn make_quad() -> ([f32; 8], [f32; 8], [u16; 6]) {
  let vertices: [f32; 8] = [-1.0, -1.0, 1.0, -1.0, 1.0, 1.0, -1.0, 1.0];
  let uvs: [f32; 8] = [0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0];
  let indices: [u16; 6] = [0, 1, 2, 2, 3, 0];

  (vertices, uvs, indices)
}

pub fn render_pipeline(
  display_program: &gl::Program,
  compute_program: &gl::Program,
  copy_program: &gl::Program,
  w: u32,
  h: u32,
  state: &mut gl::GlState,
) -> Result<(), JsValue> {
  let context: WebGl = get_ctx("webgl")?;

  context.clear_color(0.0, 0.0, 0.0, 1.0);
  context.clear(WebGl::COLOR_BUFFER_BIT);

  let uniforms = vec![
    ("canvasSize", gl::UniformData::Vector2([w as f32, h as f32])),
    ("state", gl::UniformData::Texture("display")),
  ]
  .into_iter()
  .collect::<HashMap<_, _>>();

  let copy_uniforms = vec![("state", gl::UniformData::Texture("state"))]
    .into_iter()
    .collect::<HashMap<_, _>>();

  state
    .run_mut(&compute_program, &uniforms, "state")?
    .run_mut(&copy_program, &copy_uniforms, "display")?
    .run(&display_program, &uniforms)?;

  Ok(())
}

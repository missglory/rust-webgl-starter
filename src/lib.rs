extern crate js_sys;
extern crate wasm_bindgen;
extern crate web_sys;
use js_sys::{Float32Array, WebAssembly};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::WebGlRenderingContext;

#[allow(dead_code)]
mod utils;
use utils::{compile_shader, link_program, set_panic_hook};

#[allow(non_snake_case)]
#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    set_panic_hook();
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    // canvas.set_attribute("width", &web_sys::window().unwrap().inner_width().unwrap().as_string().unwrap()[..]);
    // canvas.set_attribute("height", "1080");
    // canvas.
    
    // canvas.set_attribute("height", &web_sys::window().unwrap().inner_height().unwrap().as_string().unwrap()[..]);
    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;

    let gl = canvas
        .get_context("webgl")?
        .unwrap()
        .dyn_into::<WebGlRenderingContext>()?;

    let vertices: [f32; 6] = [
        -0.5, 0.5,
        0.0, 0.5,
        -0.25, 0.25
    ];
    let vertices_array = {
        let memory_buffer = wasm_bindgen::memory()
            .dyn_into::<WebAssembly::Memory>()?
            .buffer();
        let location: u32 = vertices.as_ptr() as u32 / 4;
        Float32Array::new(&memory_buffer).subarray(location, location + vertices.len() as u32)
    };

    // Create an empty buffer object to store the vertex buffer
    let vertex_buffer = gl.create_buffer().ok_or("failed to create buffer")?;

    //Bind appropriate array buffer to it
    gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&vertex_buffer));

    // Pass the vertex data to the buffer
    gl.buffer_data_with_array_buffer_view(
        WebGlRenderingContext::ARRAY_BUFFER,
        &vertices_array,
        WebGlRenderingContext::STATIC_DRAW,
    );

    gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, None);

    let vertCode = r#"attribute vec2 coordinates;
void main(void) {
    gl_Position = vec4(coordinates, 0.0, 1.0);
    gl_PointSize = 5.0;
}
"#;
    let vertShader = compile_shader(&gl, WebGlRenderingContext::VERTEX_SHADER, vertCode)?;

    let fragCode = r#"void main(void) {
    gl_FragColor = vec4(1.0, 1.0, 1.0, 0.1);
}"#;
    let fragShader = compile_shader(&gl, WebGlRenderingContext::FRAGMENT_SHADER, fragCode)?;
    let shaderProgram = link_program(&gl, &vertShader, &fragShader)?;

    gl.use_program(Some(&shaderProgram));
    gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&vertex_buffer));
    let coord = gl.get_attrib_location(&shaderProgram, "coordinates") as u32;
    gl.vertex_attrib_pointer_with_i32(coord, 3, WebGlRenderingContext::FLOAT, false, 0, 0);
    gl.enable_vertex_attrib_array(coord);
    gl.clear_color(0., 0., 0., 1.);
    // gl.clear_color(1., 1., 1., 1.);
    gl.disable(WebGlRenderingContext::DEPTH_TEST);
    gl.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);
    gl.viewport(0, 0, canvas.width() as i32, canvas.height() as i32);
    gl.draw_arrays(WebGlRenderingContext::POINTS, 0, 3);

    Ok(())
}

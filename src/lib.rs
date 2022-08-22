use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlShader};
use console_error_panic_hook::hook;
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;
use glam::Mat4;

// macro_rules! console_log {
//     ($($t:tt)*) => (web_sys::console::log_1(&format_args!($($t)*).to_string().into()))
// }

macro_rules! console_log {
    ($($t:tt)*) => (web_sys::console::log_1(&format!($($t)*).into()))
}

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    std::panic::set_hook(Box::new(hook));

    console_log!("starting webgl");

    let window = web_sys::window().expect("no global `window` exists");
    let document = window.document().unwrap();
    let body = document.body().expect("document should have a body");

    let canvas_width = 700;
    let canvas_height = 500;
    let canvas_aspect_ratio = canvas_width as f32 / canvas_height as f32;

    let canvas = document.create_element("canvas")?;
    canvas.set_attribute("width", &canvas_width.to_string())?;
    canvas.set_attribute("height", &canvas_height.to_string())?;
    body.append_child(&canvas)?;

    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;

    let context = canvas
        .get_context("webgl2")?
        .unwrap()
        .dyn_into::<WebGl2RenderingContext>()?;

    let vert_shader = compile_shader(
        &context,
        WebGl2RenderingContext::VERTEX_SHADER,
        include_str!("shader.vert"),
    )?;

    let frag_shader = compile_shader(
        &context,
        WebGl2RenderingContext::FRAGMENT_SHADER,
        include_str!("shader.frag"),
    )?;
    let program = link_program(&context, &vert_shader, &frag_shader)?;
    context.use_program(Some(&program));

    // let mut vertices: [f32; 9] = [-0.7, -0.7, 0.0, 0.7, -0.7, 0.0, 0.0, 0.7, 0.0];
    // let mut vertices: Vec<i32> = vec![-0.7, -0.7, 0.0, 0.7, -0.7, 0.0, 0.0, 0.7, 0.0];
    // let mut vertices: Vec<i32> = vec![0, 0, 1, -1, 2, -1, 2, 0, 2, 1];
    let mut vertices: Vec<i32> = Vec::new();

    let mut alive_cells: HashSet<(i32, i32)> = HashSet::new();
    let mut alive_cells_next: HashSet<(i32, i32)> = HashSet::new();

    // glider
    alive_cells.insert((0, 0));
    alive_cells.insert((1, -1));
    alive_cells.insert((2, -1));
    alive_cells.insert((2, 0));
    alive_cells.insert((2, 1));

    // square
    alive_cells.insert((-3, 5));
    alive_cells.insert((-2, 5));
    alive_cells.insert((-2, 4));
    alive_cells.insert((-3, 4));

    // stable thing
    alive_cells.insert((7, 7));
    alive_cells.insert((8, 7));
    alive_cells.insert((9, 7));
    alive_cells.insert((8, 8));

    fn get_num_neighbours(cell: &(i32, i32), alive_cells: &HashSet<(i32, i32)>) -> i32 {
        let mut num_neighbours = 0;
        for x in -1..=1 {
            for y in -1..=1 {
                if !(x == 0 && y == 0) && alive_cells.contains(&(cell.0 + x, cell.1 + y)) {
                    num_neighbours += 1;
                }
            }
        }
        return num_neighbours;
    }

    fn game_of_life_step(alive_cells: &HashSet<(i32, i32)>, alive_cells_next: &mut HashSet<(i32, i32)>) {
        alive_cells_next.clear();
        for cell in alive_cells {
            let num_neighbours = get_num_neighbours(cell, &alive_cells);
            for x in -1..=1 {
                for y in -1..=1 {
                    if !(x == 0 && y == 0) {
                        let check_cell = (cell.0 + x, cell.1 + y);
                        let is_dead = !alive_cells.contains(&check_cell);
                        if is_dead && get_num_neighbours(&check_cell, &alive_cells) == 3 {
                            alive_cells_next.insert(check_cell);
                        }
                    }
                }
            }
            if num_neighbours == 2 || num_neighbours == 3 {
                alive_cells_next.insert(cell.clone());
            }
        }
    }

    // for i in 0..3 {
    //     game_of_life_step(&alive_cells, &mut alive_cells_next);
    //     std::mem::swap(&mut alive_cells, &mut alive_cells_next);
    // }
    //
    // vertices.clear();
    // for cell in &alive_cells {
    //     vertices.push(cell.0);
    //     vertices.push(cell.1);
    // }

    // for x in 0..100 {
    //     for y in 0..100 {
    //         vertices.push(x);
    //         vertices.push(y);
    //     }
    // }

    // for i in (0..vertices.len()).step_by(2) {
    //     web_sys::console::log_1(&format!("{}, {}", vertices[i], vertices[i + 1]).into());
    // }

    let position_attribute_location = context.get_attrib_location(&program, "position");
    let buffer = context.create_buffer().ok_or("Failed to create buffer")?;
    context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));

    // Note that `Float32Array::view` is somewhat dangerous (hence the
    // `unsafe`!). This is creating a raw view into our module's
    // `WebAssembly.Memory` buffer, but if we allocate more pages for ourself
    // (aka do a memory allocation in Rust) it'll cause the buffer to change,
    // causing the `Float32Array` to be invalid.
    //
    // As a result, after `Float32Array::view` we have to be very careful not to
    // do any memory allocations before it's dropped.
    // unsafe {
    //     // let positions_array_buf_view = js_sys::Float32Array::view(&vertices);
    //     let positions_array_buf_view = js_sys::Int32Array::view(&vertices);
    //
    //     context.buffer_data_with_array_buffer_view(
    //         WebGl2RenderingContext::ARRAY_BUFFER,
    //         &positions_array_buf_view,
    //         WebGl2RenderingContext::STREAM_DRAW,
    //     );
    // }

    let buffer_size = 1024 * 1024 * 100; // 100Mib
    context.buffer_data_with_i32(
        WebGl2RenderingContext::ARRAY_BUFFER,
        buffer_size,
        WebGl2RenderingContext::STREAM_DRAW,
    );

    let vao = context
        .create_vertex_array()
        .ok_or("Could not create vertex array object")?;
    context.bind_vertex_array(Some(&vao));

    context.vertex_attrib_pointer_with_i32(0, 2, WebGl2RenderingContext::INT, false, 0, 0);
    context.enable_vertex_attrib_array(position_attribute_location as u32);

    context.bind_vertex_array(Some(&vao));

    let u_canvas_height_loc = context.get_uniform_location(&program, "u_canvas_height");
    context.uniform1i(u_canvas_height_loc.as_ref(), canvas_height);

    let orthographic_size = 100.0;
    let projection = Mat4::orthographic_rh_gl(-canvas_aspect_ratio * orthographic_size, canvas_aspect_ratio * orthographic_size, -1.0 * orthographic_size, 1.0 * orthographic_size, -1.0, 1.0);

    let u_orthographic_size_loc = context.get_uniform_location(&program, "u_orthographic_size");
    context.uniform1f(u_orthographic_size_loc.as_ref(), orthographic_size);

    let u_projection_loc = context.get_uniform_location(&program, "u_projection");
    context.uniform_matrix4fv_with_f32_array(u_projection_loc.as_ref(), false, projection.as_ref());

    let animation_loop_closure = Rc::new(RefCell::new(None::<Closure::<dyn FnMut()>>));
    let animation_loop_closure_outer = animation_loop_closure.clone();

    let window = Rc::new(window);
    let window_outer = Rc::clone(&window);

    *animation_loop_closure_outer.borrow_mut() = Some(Closure::<dyn FnMut()>::new(move || {
        game_of_life_step(&alive_cells, &mut alive_cells_next);
        std::mem::swap(&mut alive_cells, &mut alive_cells_next);

        vertices.clear();
        for cell in &alive_cells {
            if (vertices.len() + 2) * 4 > buffer_size as usize {
                console_log!("buffer not large enough. skipping vertices");
                break;
            }
            vertices.push(cell.0);
            vertices.push(cell.1);
        }

        unsafe {
            let positions_array_buf_view = js_sys::Int32Array::view(&vertices);

            context.buffer_sub_data_with_i32_and_array_buffer_view_and_src_offset_and_length(
                WebGl2RenderingContext::ARRAY_BUFFER,
                0,
                &positions_array_buf_view,
                0,
                vertices.len() as u32
            );
        }

        let vert_count = (vertices.len() / 2) as i32;
        draw(&context, vert_count);

        window.request_animation_frame(animation_loop_closure.borrow().as_ref().unwrap().as_ref().unchecked_ref()).expect("request_animation_frame failed");
    }));
    window_outer.request_animation_frame(animation_loop_closure_outer.borrow().as_ref().unwrap().as_ref().unchecked_ref()).expect("request_animation_frame failed");

    Ok(())
}

fn draw(context: &WebGl2RenderingContext, vert_count: i32) {
    context.clear_color(0.0, 0.0, 0.0, 1.0);
    context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

    context.draw_arrays(WebGl2RenderingContext::POINTS, 0, vert_count);
}

pub fn compile_shader(
    context: &WebGl2RenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let shader = context
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create shader object"))?;
    context.shader_source(&shader, source);
    context.compile_shader(&shader);

    if context
        .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(shader)
    } else {
        Err(context
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Unknown error creating shader")))
    }
}

pub fn link_program(
    context: &WebGl2RenderingContext,
    vert_shader: &WebGlShader,
    frag_shader: &WebGlShader,
) -> Result<WebGlProgram, String> {
    let program = context
        .create_program()
        .ok_or_else(|| String::from("Unable to create shader object"))?;

    context.attach_shader(&program, vert_shader);
    context.attach_shader(&program, frag_shader);
    context.link_program(&program);

    if context
        .get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false)
    {
        Ok(program)
    } else {
        Err(context
            .get_program_info_log(&program)
            .unwrap_or_else(|| String::from("Unknown error creating program object")))
    }
}

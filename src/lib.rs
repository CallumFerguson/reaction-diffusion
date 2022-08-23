use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlShader};
use console_error_panic_hook::hook;
use std::cell::RefCell;
use std::collections::HashSet;
use std::ops::{Mul, Sub};
use std::rc::Rc;
use glam::{Mat4, Vec3, Vec4};
use rand::Rng;

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

    let mut vertices: Vec<i32> = Vec::new();

    let mut alive_cells: HashSet<(i32, i32)> = HashSet::new();
    let mut alive_cells_next: HashSet<(i32, i32)> = HashSet::new();

    // // glider
    // alive_cells.insert((0, 0));
    // alive_cells.insert((1, -1));
    // alive_cells.insert((2, -1));
    // alive_cells.insert((2, 0));
    // alive_cells.insert((2, 1));
    //
    // // square
    // alive_cells.insert((-3, 5));
    // alive_cells.insert((-2, 5));
    // alive_cells.insert((-2, 4));
    // alive_cells.insert((-3, 4));
    //
    // // stable thing
    // alive_cells.insert((7, 7));
    // alive_cells.insert((8, 7));
    // alive_cells.insert((9, 7));
    // alive_cells.insert((8, 8));

    let start_square_size = 100;

    let mut rng = rand::thread_rng();
    for x in 0..start_square_size {
        for y in 0..start_square_size {
            if rng.gen::<bool>() {
                alive_cells.insert((x, y));
            }
        }
    }

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
                        if is_dead && !alive_cells_next.contains(&check_cell) && get_num_neighbours(&check_cell, &alive_cells) == 3 {
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

    let position_attribute_location = context.get_attrib_location(&program, "position");
    let buffer = context.create_buffer().ok_or("Failed to create buffer")?;
    context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&buffer));

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

    let mut orthographic_size: f32 = start_square_size as f32;
    let projection = Mat4::orthographic_rh_gl(-canvas_aspect_ratio * orthographic_size, canvas_aspect_ratio * orthographic_size, -1.0 * orthographic_size, 1.0 * orthographic_size, -1.0, 1.0);
    let projection = Rc::new(RefCell::new(projection));

    let u_projection_loc = context.get_uniform_location(&program, "u_projection");
    context.uniform_matrix4fv_with_f32_array(u_projection_loc.as_ref(), false, projection.borrow().as_ref());

    let u_orthographic_size_loc = context.get_uniform_location(&program, "u_orthographic_size");
    context.uniform1f(u_orthographic_size_loc.as_ref(), orthographic_size);

    let camera_pos = Vec3::new(start_square_size as f32 / 2.0 - 0.5, start_square_size as f32 / 2.0 - 0.5, 0.0);
    let view = Mat4::from_translation(camera_pos).inverse();
    let camera_pos = Rc::new(RefCell::new(camera_pos));
    let view = Rc::new(RefCell::new(view));

    let screen_to_clip = Mat4::orthographic_rh_gl(0.0, canvas_width as f32, canvas_height as f32, 0.0, -1.0, 1.0);
    let clip_to_screen = screen_to_clip.clone().inverse();

    let u_view_loc = context.get_uniform_location(&program, "u_view");
    context.uniform_matrix4fv_with_f32_array(u_view_loc.as_ref(), false, view.borrow().as_ref());

    let context = Rc::new(context);
    let context_inner = Rc::clone(&context);
    let projection_outer = Rc::clone(&projection);
    let view_outer = Rc::clone(&view);
    let camera_pos_outer = Rc::clone(&camera_pos);
    let u_view_loc_outer = u_view_loc.clone();
    let event_closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::WheelEvent| {
        let world_to_clip = *projection.borrow() * *view.borrow();
        let clip_to_world = world_to_clip.clone().inverse();

        let mouse_world_before = clip_to_world * screen_to_clip * Vec4::new(event.offset_x() as f32, event.offset_y() as f32, 0.0, 1.0);

        orthographic_size += event.delta_y() as f32 / 500.0 * orthographic_size;
        orthographic_size = orthographic_size.clamp(1.0, 10000.0);

        *projection.borrow_mut() = Mat4::orthographic_rh_gl(-canvas_aspect_ratio * orthographic_size, canvas_aspect_ratio * orthographic_size, -1.0 * orthographic_size, 1.0 * orthographic_size, -1.0, 1.0);

        context_inner.uniform_matrix4fv_with_f32_array(u_projection_loc.as_ref(), false, projection.borrow().as_ref());
        context_inner.uniform1f(u_orthographic_size_loc.as_ref(), orthographic_size);

        let world_to_clip = *projection.borrow() * *view.borrow();
        let clip_to_world = world_to_clip.clone().inverse();
        let mouse_world_after = clip_to_world * screen_to_clip * Vec4::new(event.offset_x() as f32, event.offset_y() as f32, 0.0, 1.0);

        let change = mouse_world_after - mouse_world_before;

        camera_pos.borrow_mut().x -= change.x;
        camera_pos.borrow_mut().y -= change.y;
        *view.borrow_mut() = Mat4::from_translation(*camera_pos.borrow()).inverse();

        context_inner.uniform_matrix4fv_with_f32_array(u_view_loc.as_ref(), false, view.borrow().as_ref());
    });
    canvas.add_event_listener_with_callback("wheel", event_closure.as_ref().unchecked_ref())?;
    event_closure.forget();
    let projection = projection_outer;
    let view = view_outer;
    let camera_pos = camera_pos_outer;
    let u_view_loc = u_view_loc_outer;

    let context_inner = Rc::clone(&context);
    let event_closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::MouseEvent| {
        let primary = event.buttons() & (1u16 << 0) > 0;
        let secondary = event.buttons() & (1u16 << 1) > 0;
        let wheel = event.buttons() & (1u16 << 2) > 0;

        if primary {
            // console_log!("[{}, {}]", event.movement_x(), -event.movement_y());
            // console_log!("[{}, {}]", event.offset_x(), event.offset_y());

            let world_to_clip = *projection.borrow() * *view.borrow();
            let clip_to_world = world_to_clip.clone().inverse();

            let zero_zero_world = clip_to_world * screen_to_clip * Vec4::new(0.0, 0.0, 0.0, 1.0);
            let change_from_zero_zero_world = clip_to_world * screen_to_clip * Vec4::new(event.movement_x() as f32, event.movement_y() as f32, 0.0, 1.0);
            // console_log!("[{}, {}] [{}, {}]", event.movement_x(), event.movement_y(), world_change.x, world_change.y);

            camera_pos.borrow_mut().x -= change_from_zero_zero_world.x - zero_zero_world.x;
            camera_pos.borrow_mut().y -= change_from_zero_zero_world.y - zero_zero_world.y;
            *view.borrow_mut() = Mat4::from_translation(*camera_pos.borrow()).inverse();

            context_inner.uniform_matrix4fv_with_f32_array(u_view_loc.as_ref(), false, view.borrow().as_ref());
        }
    });
    canvas.add_event_listener_with_callback("mousemove", event_closure.as_ref().unchecked_ref())?;
    event_closure.forget();

    // console_log!("{}", camera_pos.x);

    let animation_loop_closure = Rc::new(RefCell::new(None::<Closure::<dyn FnMut(_)>>));
    let animation_loop_closure_outer = animation_loop_closure.clone();

    let window = Rc::new(window);
    let window_outer = Rc::clone(&window);

    let mut start_time = -1.0;
    let mut last_unscaled_time = 0.0;

    *animation_loop_closure_outer.borrow_mut() = Some(Closure::<dyn FnMut(_)>::new(move |now: f64| {
        let now = now * 0.001;
        if start_time < 0.0 {
            start_time = now;
        }
        let unscaled_time = now - start_time;
        let delta_time = unscaled_time - last_unscaled_time;
        last_unscaled_time = unscaled_time;
        // console_log!("{}", 1.0 / delta_time);

        game_of_life_step(&alive_cells, &mut alive_cells_next);
        std::mem::swap(&mut alive_cells, &mut alive_cells_next);

        vertices.clear();
        for cell in &alive_cells {
            if (vertices.len() + 2) * 4 > buffer_size as usize {
                console_log!("buffer not large enough. skipping remaining cells");
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
                vertices.len() as u32,
            );
        }

        let vert_count = (vertices.len() / 2) as i32;
        draw(&context, vert_count);

        window.request_animation_frame(animation_loop_closure.borrow().as_ref().unwrap().as_ref().unchecked_ref()).expect("request_animation_frame failed");
    }));
    let window = window_outer;
    window.request_animation_frame(animation_loop_closure_outer.borrow().as_ref().unwrap().as_ref().unchecked_ref()).expect("request_animation_frame failed");

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

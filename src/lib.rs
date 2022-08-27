use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlShader};
use console_error_panic_hook::hook;
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;
use glam::{Mat4, Vec4};
use crate::engine::viewport::Viewport;
use crate::engine::component::Component;
use crate::engine::game_object::GameObject;
use crate::game_of_life::GameOfLife;

#[macro_use]
mod utils;
mod engine;
mod game_of_life;

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    std::panic::set_hook(Box::new(hook));

    console_log!("starting webgl");

    let window = Rc::new(web_sys::window().expect("no global `window` exists"));

    let viewport = Viewport::new();

    let canvas = viewport.borrow().canvas();
    let context = viewport.borrow().context();

    let program = create_shader_program(&context, include_str!("shader.vert"), include_str!("shader.frag"));
    context.use_program(Some(&program));

    let mut vertices: Vec<i32> = Vec::new();

    let mut alive_cells: HashSet<(i32, i32)> = HashSet::new();
    let mut alive_cells_next: HashSet<(i32, i32)> = HashSet::new();

    let start_cells = "........................O...........
......................O.O...........
............OO......OO............OO
...........O...O....OO............OO
OO........O.....O...OO..............
OO........O...O.OO....O.O...........
..........O.....O.......O...........
...........O...O....................
............OO......................";

    let mut x = 0;
    let mut y = 0;
    for char in start_cells.chars() {
        if char == 'O' {
            alive_cells.insert((x, y));
        }
        x += 1;
        if char == '\n' {
            x = 0;
            y -= 1;
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

    let u_orthographic_size_loc = context.get_uniform_location(&program, "u_orthographic_size");
    viewport.borrow_mut().set_orthographic_size_change(Some(Box::new(move |viewport: &Viewport| {
        viewport.context().uniform1f(u_orthographic_size_loc.as_ref(), viewport.orthographic_size());
    })));

    let u_canvas_height_loc = context.get_uniform_location(&program, "u_canvas_height");
    viewport.borrow_mut().set_height_change(Some(Box::new(move |viewport: &Viewport| {
        viewport.context().uniform1i(u_canvas_height_loc.as_ref(), viewport.height());
    })));

    let u_view_loc = context.get_uniform_location(&program, "u_view");
    viewport.borrow_mut().set_view_change(Some(Box::new(move |viewport: &Viewport| {
        viewport.context().uniform_matrix4fv_with_f32_array(u_view_loc.as_ref(), false, viewport.view().as_ref());
    })));

    let u_projection_loc = context.get_uniform_location(&program, "u_projection");
    viewport.borrow_mut().set_projection_change(Some(Box::new(move |viewport: &Viewport| {
        viewport.context().uniform_matrix4fv_with_f32_array(u_projection_loc.as_ref(), false, viewport.projection().as_ref());
    })));

    let viewport = Rc::clone(&viewport);
    let viewport_outer = Rc::clone(&viewport);
    let event_closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::WheelEvent| {
        let world_to_clip = viewport.borrow().projection() * viewport.borrow().view();
        let clip_to_world = world_to_clip.clone().inverse();

        let screen_to_clip = Mat4::orthographic_rh_gl(0.0, viewport.borrow().width() as f32, viewport.borrow().height() as f32, 0.0, -1.0, 1.0);
        let mouse_world_before = clip_to_world * screen_to_clip * Vec4::new(event.offset_x() as f32, event.offset_y() as f32, 0.0, 1.0);

        let mut orthographic_size = viewport.borrow().orthographic_size();
        orthographic_size += event.delta_y() as f32 / 500.0 * orthographic_size;
        orthographic_size = orthographic_size.clamp(7.5, 7500.0);
        viewport.borrow_mut().set_orthographic_size(orthographic_size);

        let world_to_clip = viewport.borrow().projection() * viewport.borrow().view();
        let clip_to_world = world_to_clip.clone().inverse();
        let mouse_world_after = clip_to_world * screen_to_clip * Vec4::new(event.offset_x() as f32, event.offset_y() as f32, 0.0, 1.0);

        let change = mouse_world_after - mouse_world_before;

        let mut camera_pos = *viewport.borrow().camera_pos();
        camera_pos.x -= change.x;
        camera_pos.y -= change.y;
        viewport.borrow_mut().set_camera_pos(camera_pos);
    });
    canvas.add_event_listener_with_callback("wheel", event_closure.as_ref().unchecked_ref())?;
    event_closure.forget();
    let viewport = viewport_outer;

    let viewport_outer = Rc::clone(&viewport);
    let event_closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::MouseEvent| {
        let primary = event.buttons() & (1u16 << 0) > 0;
        // let secondary = event.buttons() & (1u16 << 1) > 0;
        // let wheel = event.buttons() & (1u16 << 2) > 0;

        if primary {
            let world_to_clip = viewport.borrow().projection() * viewport.borrow().view();
            let clip_to_world = world_to_clip.clone().inverse();

            let screen_to_clip = Mat4::orthographic_rh_gl(0.0, viewport.borrow().width() as f32, viewport.borrow().height() as f32, 0.0, -1.0, 1.0);
            let zero_zero_world = clip_to_world * screen_to_clip * Vec4::new(0.0, 0.0, 0.0, 1.0);
            let change_from_zero_zero_world = clip_to_world * screen_to_clip * Vec4::new(event.movement_x() as f32, event.movement_y() as f32, 0.0, 1.0);

            let mut camera_pos = *viewport.borrow().camera_pos();
            camera_pos.x -= change_from_zero_zero_world.x - zero_zero_world.x;
            camera_pos.y -= change_from_zero_zero_world.y - zero_zero_world.y;
            viewport.borrow_mut().set_camera_pos(camera_pos);
        }
    });
    canvas.add_event_listener_with_callback("mousemove", event_closure.as_ref().unchecked_ref())?;
    event_closure.forget();
    let viewport = viewport_outer;

    let animation_loop_closure = Rc::new(RefCell::new(None::<Closure::<dyn FnMut(_)>>));
    let animation_loop_closure_outer = animation_loop_closure.clone();

    let window = Rc::new(window);
    let window_outer = Rc::clone(&window);

    let mut start_time = -1.0;
    let mut last_unscaled_time = 0.0;

    let mut game_objects = Vec::<GameObject>::new();

    let mut game_of_life = GameObject::new();
    game_of_life.add_component(Box::new(GameOfLife::new()));

    game_objects.push(game_of_life);

    *animation_loop_closure_outer.borrow_mut() = Some(Closure::<dyn FnMut(_)>::new(move |now: f64| {
        let now = now * 0.001;
        if start_time < 0.0 {
            start_time = now;
        }
        let unscaled_time = now - start_time;
        let _delta_time = unscaled_time - last_unscaled_time;
        last_unscaled_time = unscaled_time;
        // console_log!("{}", 1.0 / delta_time);

        viewport.borrow().update_uniforms_in_shader();

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
        context.clear_color(0.0, 0.0, 0.0, 1.0);
        context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
        context.draw_arrays(WebGl2RenderingContext::POINTS, 0, vert_count);

        for game_object in &game_objects {
            for component in game_object.components_iter() {
                component.on_update();
            }
        }

        for game_object in &game_objects {
            for component in game_object.components_iter() {
                component.on_render_object();
            }
        }

        window.request_animation_frame(animation_loop_closure.borrow().as_ref().unwrap().as_ref().unchecked_ref()).expect("request_animation_frame failed");
    }));
    let window = window_outer;
    window.request_animation_frame(animation_loop_closure_outer.borrow().as_ref().unwrap().as_ref().unchecked_ref()).expect("request_animation_frame failed");

    Ok(())
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

fn create_shader_program(context: &WebGl2RenderingContext, vertex_shader_string: &str, fragment_shader_string: &str) -> WebGlProgram {
    let vert_shader = compile_shader(
        context,
        WebGl2RenderingContext::VERTEX_SHADER,
        vertex_shader_string,
    ).unwrap();

    let frag_shader = compile_shader(
        context,
        WebGl2RenderingContext::FRAGMENT_SHADER,
        fragment_shader_string,
    ).unwrap();

    return link_program(&context, &vert_shader, &frag_shader).unwrap();
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

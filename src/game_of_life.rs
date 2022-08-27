use std::collections::HashSet;
use std::rc::Rc;
use glam::{Mat4, Vec4};
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::Closure;
use web_sys::{WebGl2RenderingContext, WebGlBuffer, WebGlProgram, WebGlVertexArrayObject};
use crate::{Component, EngineState, Viewport};
use crate::utils::create_shader_program;

const BUFFER_SIZE: i32 = 1024 * 1024 * 100; // 100Mib

pub struct GameOfLife {
    id: i32,
    vertices: Vec<i32>,
    alive_cells: HashSet<(i32, i32)>,
    alive_cells_next: HashSet<(i32, i32)>,
    vao: Rc<Option<WebGlVertexArrayObject>>,
    program: Rc<Option<WebGlProgram>>,
    buffer: Option<WebGlBuffer>
}

impl GameOfLife {
    pub fn new(id: i32) -> Self {
        return Self {
            id,
            vertices: Vec::new(),
            alive_cells: HashSet::new(),
            alive_cells_next: HashSet::new(),
            vao: Rc::new(None),
            program: Rc::new(None),
            buffer: None
        };
    }
}

impl Component for GameOfLife {
    fn on_add_to_game_object(&mut self, engine_state: &EngineState) {
        let viewport = engine_state.viewport();

        let canvas = viewport.borrow().canvas();
        let context = viewport.borrow().context();

        self.vao = Rc::new(Some(context
            .create_vertex_array()
            .ok_or("Could not create vertex array object").unwrap()));
        context.bind_vertex_array(self.vao.as_ref().as_ref());

        self.program = Rc::new(Some(create_shader_program(&context, include_str!("shader.vert"), include_str!("shader.frag"))));
        context.use_program(self.program.as_ref().as_ref());

        let mut start_cells = "........................O...........
......................O.O...........
............OO......OO............OO
...........O...O....OO............OO
OO........O.....O...OO..............
OO........O...O.OO....O.O...........
..........O.....O.......O...........
...........O...O....................
............OO......................";

        let u_color_loc = context.get_uniform_location(self.program.as_ref().as_ref().unwrap(), "u_color");

        if self.id == 0 {
            start_cells = "..O
O.O
.OO";
            context.uniform3f(u_color_loc.as_ref(), 1.0, 0.0, 0.0);
        }

        if self.id == 1 {
            start_cells = "..............OOO";
            context.uniform3f(u_color_loc.as_ref(), 0.0, 1.0, 0.0);
        }

        let mut x = 0;
        let mut y = 0;
        for char in start_cells.chars() {
            if char == 'O' {
                self.alive_cells.insert((x, y));
            }
            x += 1;
            if char == '\n' {
                x = 0;
                y -= 1;
            }
        }

        let position_attribute_location = context.get_attrib_location(self.program.as_ref().as_ref().unwrap(), "position");
        self.buffer = Some(context.create_buffer().ok_or("Failed to create buffer").unwrap());
        context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, self.buffer.as_ref());

        context.buffer_data_with_i32(
            WebGl2RenderingContext::ARRAY_BUFFER,
            BUFFER_SIZE,
            WebGl2RenderingContext::STREAM_DRAW,
        );

        context.vertex_attrib_pointer_with_i32(0, 2, WebGl2RenderingContext::INT, false, 0, 0);
        context.enable_vertex_attrib_array(position_attribute_location as u32);

        let u_orthographic_size_loc = context.get_uniform_location(self.program.as_ref().as_ref().unwrap(), "u_orthographic_size");
        context.uniform1f(u_orthographic_size_loc.as_ref(), viewport.borrow().orthographic_size());
        let vao = Rc::clone(&self.vao);
        let program = Rc::clone(&self.program);
        viewport.borrow_mut().set_orthographic_size_change(Some(Box::new(move |viewport: &Viewport| {
            viewport.context().bind_vertex_array(vao.as_ref().as_ref());
            viewport.context().use_program(program.as_ref().as_ref());
            viewport.context().uniform1f(u_orthographic_size_loc.as_ref(), viewport.orthographic_size());
        })));

        let u_canvas_height_loc = context.get_uniform_location(self.program.as_ref().as_ref().unwrap(), "u_canvas_height");
        context.uniform1i(u_canvas_height_loc.as_ref(), viewport.borrow().height());
        let vao = Rc::clone(&self.vao);
        let program = Rc::clone(&self.program);
        viewport.borrow_mut().set_height_change(Some(Box::new(move |viewport: &Viewport| {
            viewport.context().bind_vertex_array(vao.as_ref().as_ref());
            viewport.context().use_program(program.as_ref().as_ref());
            viewport.context().uniform1i(u_canvas_height_loc.as_ref(), viewport.height());
        })));

        let u_view_loc = context.get_uniform_location(self.program.as_ref().as_ref().unwrap(), "u_view");
        context.uniform_matrix4fv_with_f32_array(u_view_loc.as_ref(), false, viewport.borrow().view().as_ref());
        let vao = Rc::clone(&self.vao);
        let program = Rc::clone(&self.program);
        viewport.borrow_mut().set_view_change(Some(Box::new(move |viewport: &Viewport| {
            viewport.context().bind_vertex_array(vao.as_ref().as_ref());
            viewport.context().use_program(program.as_ref().as_ref());
            viewport.context().uniform_matrix4fv_with_f32_array(u_view_loc.as_ref(), false, viewport.view().as_ref());
        })));

        let u_projection_loc = context.get_uniform_location(self.program.as_ref().as_ref().unwrap(), "u_projection");
        context.uniform_matrix4fv_with_f32_array(u_projection_loc.as_ref(), false, viewport.borrow().projection().as_ref());
        let vao = Rc::clone(&self.vao);
        let program = Rc::clone(&self.program);
        viewport.borrow_mut().set_projection_change(Some(Box::new(move |viewport: &Viewport| {
            viewport.context().bind_vertex_array(vao.as_ref().as_ref());
            viewport.context().use_program(program.as_ref().as_ref());
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
        canvas.add_event_listener_with_callback("wheel", event_closure.as_ref().unchecked_ref()).unwrap();
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
        canvas.add_event_listener_with_callback("mousemove", event_closure.as_ref().unchecked_ref()).unwrap();
        event_closure.forget();
    }

    fn on_update(&mut self, engine_state: &EngineState) {
        let viewport = engine_state.viewport();
        let viewport = viewport.borrow();
        let context = viewport.context();

        context.bind_vertex_array(self.vao.as_ref().as_ref());
        context.use_program(self.program.as_ref().as_ref());

        engine_state.viewport().borrow().update_uniforms_in_shader();

        context.bind_vertex_array(self.vao.as_ref().as_ref());
        context.use_program(self.program.as_ref().as_ref());

        game_of_life_step(&self.alive_cells, &mut self.alive_cells_next);
        std::mem::swap(&mut self.alive_cells, &mut self.alive_cells_next);

        self.vertices.clear();
        for cell in &self.alive_cells {
            if (self.vertices.len() + 2) * 4 > BUFFER_SIZE as usize {
                console_log!("buffer not large enough. skipping remaining cells");
                break;
            }
            self.vertices.push(cell.0);
            self.vertices.push(cell.1);
        }

        context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, self.buffer.as_ref());

        unsafe {
            let positions_array_buf_view = js_sys::Int32Array::view(&self.vertices);

            context.buffer_sub_data_with_i32_and_array_buffer_view_and_src_offset_and_length(
                WebGl2RenderingContext::ARRAY_BUFFER,
                0,
                &positions_array_buf_view,
                0,
                self.vertices.len() as u32,
            );
        }
    }

    fn on_render_object(&mut self, engine_state: &EngineState) {
        let context = engine_state.viewport().borrow().context();

        context.bind_vertex_array(self.vao.as_ref().as_ref());
        context.use_program(self.program.as_ref().as_ref());

        let vert_count = (self.vertices.len() / 2) as i32;
        context.draw_arrays(WebGl2RenderingContext::POINTS, 0, vert_count);
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

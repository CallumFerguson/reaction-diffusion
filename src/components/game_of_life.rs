use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;
use web_sys::{WebGl2RenderingContext, WebGlBuffer, WebGlProgram, WebGlVertexArrayObject};
use crate::{Component, Viewport};

const BUFFER_SIZE: i32 = 1024 * 1024 * 100; // 100Mib

pub struct GameOfLife {
    start_cells: &'static str,
    start_cells_offset: (i32, i32),
    vertices: Vec<i32>,
    alive_cells: HashSet<(i32, i32)>,
    alive_cells_next: HashSet<(i32, i32)>,
    vao: Rc<Option<WebGlVertexArrayObject>>,
    program: Rc<WebGlProgram>,
    buffer: Option<WebGlBuffer>,
    viewport: Rc<RefCell<Viewport>>
}

impl GameOfLife {
    pub fn new(viewport: Rc<RefCell<Viewport>>, program: Rc<WebGlProgram>, start_cells: &'static str, start_cells_offset: (i32, i32)) -> Self {
        return Self {
            start_cells,
            start_cells_offset,
            vertices: Vec::new(),
            alive_cells: HashSet::new(),
            alive_cells_next: HashSet::new(),
            vao: Rc::new(None),
            program,
            buffer: None,
            viewport,
        };
    }
}

impl Component for GameOfLife {
    fn on_add_to_game_object(&mut self) {
        let viewport = &self.viewport;
        let context = viewport.borrow().context();

        self.vao = Rc::new(Some(context
            .create_vertex_array()
            .ok_or("Could not create vertex array object").unwrap()));
        context.bind_vertex_array(self.vao.as_ref().as_ref());

        context.use_program(Some(&self.program));

        let mut x = 0;
        let mut y = 0;
        for char in self.start_cells.chars() {
            if char == 'O' {
                self.alive_cells.insert((x + self.start_cells_offset.0, y + self.start_cells_offset.1));
            }
            if char == 'O' || char == '.' {
                x += 1;
            }
            if char == '\n' {
                x = 0;
                y -= 1;
            }
        }

        let position_attribute_location = context.get_attrib_location(&self.program, "a_position");
        self.buffer = Some(context.create_buffer().ok_or("Failed to create buffer").unwrap());
        context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, self.buffer.as_ref());

        context.buffer_data_with_i32(
            WebGl2RenderingContext::ARRAY_BUFFER,
            BUFFER_SIZE,
            WebGl2RenderingContext::STREAM_DRAW,
        );

        context.vertex_attrib_pointer_with_i32(position_attribute_location as u32, 2, WebGl2RenderingContext::INT, false, 0, 0);
        context.enable_vertex_attrib_array(position_attribute_location as u32);
    }

    fn on_update(&mut self) {
        let viewport = &self.viewport;
        let viewport = viewport.borrow();
        let context = viewport.context();

        context.bind_vertex_array(self.vao.as_ref().as_ref());
        context.use_program(Some(&self.program));

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

    fn on_render(&mut self) {
        let context = self.viewport.borrow().context();

        context.bind_vertex_array(self.vao.as_ref().as_ref());
        context.use_program(Some(&self.program));

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

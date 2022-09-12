use std::any::Any;
use std::rc::Rc;
use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlVertexArrayObject};
use crate::{Component, GameObject};
use crate::engine::app::App;

pub struct Square {
    vao: Option<WebGlVertexArrayObject>,
    program: Rc<WebGlProgram>,
    indices_count: i32
}

impl Square {
    pub fn new(program: Rc<WebGlProgram>) -> Self {
        return Self {
            vao: None,
            program,
            indices_count: 0
        }
    }
}

impl Component for Square {
    fn on_add_to_game_object(&mut self, game_object: &mut GameObject, app: &App) {
        let gl = app.gl();

        self.vao = gl.create_vertex_array();
        gl.bind_vertex_array(self.vao.as_ref());

        let position_attribute_location = gl.get_attrib_location(&self.program, "a_position");
        let buffer = gl.create_buffer();
        gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, buffer.as_ref());

        let vertices = [-0.5, 0.5, 0.0, 0.5, 0.5, 0.0, 0.5, -0.5, 0.0, -0.5, -0.5, 0.0];

        unsafe {
            let positions_array_buf_view = js_sys::Float32Array::view(&vertices);

            gl.buffer_data_with_array_buffer_view(
                WebGl2RenderingContext::ARRAY_BUFFER,
                &positions_array_buf_view,
                WebGl2RenderingContext::STATIC_DRAW,
            );
        }

        gl.vertex_attrib_pointer_with_i32(position_attribute_location as u32, 3, WebGl2RenderingContext::FLOAT, false, 0, 0);
        gl.enable_vertex_attrib_array(position_attribute_location as u32);

        let buffer = gl.create_buffer();
        gl.bind_buffer(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, buffer.as_ref());

        let indices = [0, 1, 2, 0, 2, 3];
        self.indices_count = indices.len() as i32;

        unsafe {
            let indices_array_buf_view = js_sys::Uint16Array::view(&indices);

            gl.buffer_data_with_array_buffer_view(
                WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
                &indices_array_buf_view,
                WebGl2RenderingContext::STATIC_DRAW,
            );
        }
    }

    fn on_render(&mut self, game_object: &mut GameObject, app: &App) {
        let gl = app.gl();

        gl.bind_vertex_array(self.vao.as_ref());
        gl.use_program(Some(&self.program));

        gl.draw_elements_with_i32(WebGl2RenderingContext::TRIANGLES, self.indices_count, WebGl2RenderingContext::UNSIGNED_SHORT, 0);
    }
}
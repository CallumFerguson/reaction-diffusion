use std::cell::RefCell;
use std::rc::Rc;
use web_sys::{WebGl2RenderingContext, WebGlBuffer, WebGlProgram, WebGlVertexArrayObject};
use crate::{Component, Viewport};

pub struct ReactionDiffusion {
    vao: Option<WebGlVertexArrayObject>,
    program: Rc<WebGlProgram>,
    viewport: Rc<RefCell<Viewport>>,
    indices_count: i32
}

impl ReactionDiffusion {
    pub fn new(viewport: Rc<RefCell<Viewport>>, program: Rc<WebGlProgram>) -> Self {
        return Self {
            vao: None,
            program,
            viewport,
            indices_count: 0
        }
    }
}

impl Component for ReactionDiffusion {
    fn on_add_to_game_object(&mut self) {
        let context = self.viewport.borrow().context();

        self.vao = context.create_vertex_array();
        context.bind_vertex_array(self.vao.as_ref());

        let position_attribute_location = context.get_attrib_location(&self.program, "position");
        let buffer = context.create_buffer();
        context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, buffer.as_ref());

        let vertices = [-0.5, 0.5, 0.0, 0.5, 0.5, 0.0, 0.5, -0.5, 0.0, -0.5, -0.5, 0.0];

        unsafe {
            let positions_array_buf_view = js_sys::Float32Array::view(&vertices);

            context.buffer_data_with_array_buffer_view(
                WebGl2RenderingContext::ARRAY_BUFFER,
                &positions_array_buf_view,
                WebGl2RenderingContext::STATIC_DRAW,
            );
        }

        context.vertex_attrib_pointer_with_i32(0, 3, WebGl2RenderingContext::FLOAT, false, 0, 0);
        context.enable_vertex_attrib_array(position_attribute_location as u32);

        let buffer = context.create_buffer();
        context.bind_buffer(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, buffer.as_ref());

        let indices = [0, 1, 2, 0, 2, 3];
        self.indices_count = indices.len() as i32;

        unsafe {
            let indices_array_buf_view = js_sys::Uint16Array::view(&indices);

            context.buffer_data_with_array_buffer_view(
                WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
                &indices_array_buf_view,
                WebGl2RenderingContext::STATIC_DRAW,
            );
        }
    }

    fn on_render(&mut self) {
        let context = self.viewport.borrow().context();

        context.bind_vertex_array(self.vao.as_ref());
        context.use_program(Some(&self.program));

        context.draw_elements_with_i32(WebGl2RenderingContext::TRIANGLES, self.indices_count, WebGl2RenderingContext::UNSIGNED_SHORT, 0);
    }
}
use std::cell::RefCell;
use std::rc::Rc;
use web_sys::{WebGl2RenderingContext, WebGlBuffer, WebGlProgram, WebGlVertexArrayObject};
use crate::{Component, Viewport};

pub struct ReactionDiffusion {
    vao: Option<WebGlVertexArrayObject>,
    program: Rc<WebGlProgram>,
    viewport: Rc<RefCell<Viewport>>,
    indices_count: i32,
}

impl ReactionDiffusion {
    pub fn new(viewport: Rc<RefCell<Viewport>>, program: Rc<WebGlProgram>) -> Self {
        return Self {
            vao: None,
            program,
            viewport,
            indices_count: 0,
        };
    }
}

impl Component for ReactionDiffusion {
    fn on_add_to_game_object(&mut self) {
        let context = self.viewport.borrow().context();

        self.vao = context.create_vertex_array();
        context.bind_vertex_array(self.vao.as_ref());

        let position_attribute_location = context.get_attrib_location(&self.program, "a_position");
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

        context.vertex_attrib_pointer_with_i32(position_attribute_location as u32, 3, WebGl2RenderingContext::FLOAT, false, 0, 0);
        context.enable_vertex_attrib_array(position_attribute_location as u32);

        let uv_attribute_location = context.get_attrib_location(&self.program, "a_uv");
        let buffer = context.create_buffer();
        context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, buffer.as_ref());

        let uv = [0.0, 1.0, 1.0, 1.0, 1.0, 0.0, 0.0];

        unsafe {
            let uv_array_buf_view = js_sys::Float32Array::view(&uv);

            context.buffer_data_with_array_buffer_view(
                WebGl2RenderingContext::ARRAY_BUFFER,
                &uv_array_buf_view,
                WebGl2RenderingContext::STATIC_DRAW,
            );
        }

        context.vertex_attrib_pointer_with_i32(uv_attribute_location as u32, 2, WebGl2RenderingContext::FLOAT, false, 0, 0);
        context.enable_vertex_attrib_array(uv_attribute_location as u32);

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

        let texture = context.create_texture();
        context.bind_texture(WebGl2RenderingContext::TEXTURE_2D, texture.as_ref());

        let texture_data: [u8; 4] = [125, 125, 125, 255];

        unsafe {
            let view = js_sys::Uint8Array::view(&texture_data);

            context.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_array_buffer_view_and_src_offset(
                WebGl2RenderingContext::TEXTURE_2D,
                0,
                WebGl2RenderingContext::RGBA as i32,
                1,
                1,
                0,
                WebGl2RenderingContext::RGBA,
                WebGl2RenderingContext::UNSIGNED_BYTE,
                &view,
                0
            ).unwrap();
        }
    }

    fn on_render(&mut self) {
        let context = self.viewport.borrow().context();

        context.bind_vertex_array(self.vao.as_ref());
        context.use_program(Some(&self.program));

        context.draw_elements_with_i32(WebGl2RenderingContext::TRIANGLES, self.indices_count, WebGl2RenderingContext::UNSIGNED_SHORT, 0);
    }
}
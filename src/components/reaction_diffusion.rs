use std::cell::RefCell;
use std::rc::Rc;
use glam::{Mat4, Quat, Vec3};
use rand::Rng;
use web_sys::{WebGl2RenderingContext, WebGlBuffer, WebGlProgram, WebGlVertexArrayObject};
use crate::{Component, Viewport};

const CELLS_WIDTH: i32 = 512;
const CELLS_HEIGHT: i32 = 512;

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

        // let texture_data: [u16; 2] = [u16::MAX / 2, 50000];
        let mut cells = Vec::<u16>::with_capacity((CELLS_WIDTH * CELLS_HEIGHT * 2) as usize);

        let mut rng = rand::thread_rng();
        for _ in 0..(CELLS_WIDTH * CELLS_HEIGHT) {
            cells.push(rng.gen::<u16>());
            cells.push(rng.gen::<u16>());
        }

        // context.pixel_storei(WebGl2RenderingContext::UNPACK_ALIGNMENT, 1);

        unsafe {
            let view = js_sys::Uint16Array::view(&cells);

            context.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_array_buffer_view_and_src_offset(
                WebGl2RenderingContext::TEXTURE_2D,
                0,
                WebGl2RenderingContext::RG16UI as i32,
                CELLS_WIDTH,
                CELLS_HEIGHT,
                0,
                WebGl2RenderingContext::RG_INTEGER,
                WebGl2RenderingContext::UNSIGNED_SHORT,
                &view,
                0,
            ).unwrap();
        }

        context.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_MIN_FILTER, WebGl2RenderingContext::NEAREST as i32);
        context.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_MAG_FILTER, WebGl2RenderingContext::NEAREST as i32);
        context.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_WRAP_S, WebGl2RenderingContext::CLAMP_TO_EDGE as i32);
        context.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_WRAP_T, WebGl2RenderingContext::CLAMP_TO_EDGE as i32);

        let model = Mat4::from_scale_rotation_translation(Vec3::new(CELLS_WIDTH as f32, CELLS_HEIGHT as f32, 1.0), Quat::IDENTITY, Vec3::new(0.0, 0.0, 0.0));

        let u_model_loc = context.get_uniform_location(self.program.as_ref(), "u_model");
        context.uniform_matrix4fv_with_f32_array(u_model_loc.as_ref(), false, model.as_ref());
    }

    fn on_render(&mut self) {
        let context = self.viewport.borrow().context();

        context.bind_vertex_array(self.vao.as_ref());
        context.use_program(Some(&self.program));

        context.draw_elements_with_i32(WebGl2RenderingContext::TRIANGLES, self.indices_count, WebGl2RenderingContext::UNSIGNED_SHORT, 0);
    }
}

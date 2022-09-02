use std::cell::RefCell;
use std::rc::Rc;
use glam::{Mat4, Quat, Vec3};
use rand::Rng;
use web_sys::{WebGl2RenderingContext, WebGlBuffer, WebGlFramebuffer, WebGlProgram, WebGlTexture, WebGlVertexArrayObject};
use crate::{Component, Viewport};

const CELLS_WIDTH: i32 = 512;
const CELLS_HEIGHT: i32 = 512;

const D_A: f32 = 1.0;
const D_B: f32 = 0.5;
const F: f32 = 0.055;
const K: f32 = 0.062;
const DELTA_T: f32 = 1.0;

pub struct ReactionDiffusion {
    vao: Option<WebGlVertexArrayObject>,
    render_texture_vao: Option<WebGlVertexArrayObject>,
    unlit_texture_bicubic: Rc<WebGlProgram>,
    reaction_diffusion: Rc<WebGlProgram>,
    reaction_diffusion_render: Rc<WebGlProgram>,
    viewport: Rc<RefCell<Viewport>>,
    indices_count: i32,
    cells: [u16; (CELLS_WIDTH * CELLS_HEIGHT * 2) as usize],
    cells_next: [u16; (CELLS_WIDTH * CELLS_HEIGHT * 2) as usize],
    fbo: Option<Box<WebGlFramebuffer>>,
    input_texture: Option<Box<WebGlTexture>>,
    output_texture: Option<Box<WebGlTexture>>,
    render_texture: Option<Box<WebGlTexture>>,
}

impl ReactionDiffusion {
    pub fn new(viewport: Rc<RefCell<Viewport>>, unlit_texture_bicubic: Rc<WebGlProgram>, reaction_diffusion: Rc<WebGlProgram>, reaction_diffusion_render: Rc<WebGlProgram>) -> Self {
        let gl = viewport.borrow().gl();
        return Self {
            vao: None,
            render_texture_vao: None,
            unlit_texture_bicubic,
            reaction_diffusion,
            reaction_diffusion_render,
            viewport,
            indices_count: 0,
            cells: [0; (CELLS_WIDTH * CELLS_HEIGHT * 2) as usize],
            cells_next: [0; (CELLS_WIDTH * CELLS_HEIGHT * 2) as usize],
            fbo: None,
            input_texture: None,
            output_texture: None,
            render_texture: None,
        };
    }
}

impl Component for ReactionDiffusion {
    fn on_add_to_game_object(&mut self) {
        let viewport = self.viewport.borrow();
        let gl = viewport.gl();

        init_cells(&mut self.cells);

        let vertices = [-0.5, 0.5, 0.0, 0.5, 0.5, 0.0, 0.5, -0.5, 0.0, -0.5, -0.5, 0.0];
        self.vao = Some(init_quad(&gl, &self.unlit_texture_bicubic, &vertices));
        self.indices_count = 6;

        let vertices = [-1.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0, -1.0, 0.0, -1.0, -1.0, 0.0];
        self.render_texture_vao = Some(init_quad(&gl, &self.unlit_texture_bicubic, &vertices));

        gl.use_program(Some(&self.unlit_texture_bicubic));
        let model = Mat4::from_scale_rotation_translation(Vec3::new(CELLS_WIDTH as f32, CELLS_HEIGHT as f32, 1.0), Quat::IDENTITY, Vec3::new(0.0, 0.0, 0.0));
        let u_model_loc = gl.get_uniform_location(self.unlit_texture_bicubic.as_ref(), "u_model");
        gl.uniform_matrix4fv_with_f32_array(u_model_loc.as_ref(), false, model.as_ref());

        gl.use_program(Some(&self.reaction_diffusion));
        let u_kernel_loc = gl.get_uniform_location(self.reaction_diffusion.as_ref(), "u_kernel");
        let kernel: [f32; 9] = [
            0.05, 0.2, 0.05,
            0.2, -1.0, 0.2,
            0.05, 0.2, 0.05
        ];
        gl.uniform1fv_with_f32_array(u_kernel_loc.as_ref(), &kernel);

        let u_texture_width_loc = gl.get_uniform_location(self.reaction_diffusion.as_ref(), "u_texture_width");
        gl.uniform1i(u_texture_width_loc.as_ref(), CELLS_WIDTH);

        let u_texture_height_loc = gl.get_uniform_location(self.reaction_diffusion.as_ref(), "u_texture_height");
        gl.uniform1i(u_texture_height_loc.as_ref(), CELLS_HEIGHT);

        self.input_texture = Some(Box::new(create_and_bind_texture(&gl, WebGl2RenderingContext::NEAREST, WebGl2RenderingContext::REPEAT).unwrap()));
        unsafe {
            let view = js_sys::Uint16Array::view(&self.cells);

            gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_array_buffer_view_and_src_offset(
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

        self.output_texture = Some(Box::new(create_and_bind_texture(&gl, WebGl2RenderingContext::NEAREST, WebGl2RenderingContext::REPEAT).unwrap()));
        gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_array_buffer_view(
            WebGl2RenderingContext::TEXTURE_2D,
            0,
            WebGl2RenderingContext::RG16UI as i32,
            CELLS_WIDTH,
            CELLS_HEIGHT,
            0,
            WebGl2RenderingContext::RG_INTEGER,
            WebGl2RenderingContext::UNSIGNED_SHORT,
            None,
        ).unwrap();

        self.render_texture = Some(Box::new(create_and_bind_texture(&gl, WebGl2RenderingContext::LINEAR, WebGl2RenderingContext::REPEAT).unwrap()));
        gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_array_buffer_view(
            WebGl2RenderingContext::TEXTURE_2D,
            0,
            WebGl2RenderingContext::RGBA as i32,
            CELLS_WIDTH,
            CELLS_HEIGHT,
            0,
            WebGl2RenderingContext::RGBA,
            WebGl2RenderingContext::UNSIGNED_BYTE,
            None,
        ).unwrap();

        self.fbo = Some(Box::new(gl.create_framebuffer().unwrap()));
    }

    fn on_update(&mut self) {
        let viewport = self.viewport.borrow();
        let gl = viewport.gl();

        let iterations = 15;
        gl.bind_vertex_array(self.render_texture_vao.as_ref());
        gl.bind_framebuffer(WebGl2RenderingContext::FRAMEBUFFER, Some(self.fbo.as_ref().unwrap().as_ref()));
        gl.viewport(0, 0, CELLS_WIDTH, CELLS_HEIGHT);
        gl.use_program(Some(&self.reaction_diffusion));
        for _ in 0..iterations {
            gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(self.input_texture.as_ref().unwrap().as_ref()));

            gl.framebuffer_texture_2d(WebGl2RenderingContext::FRAMEBUFFER, WebGl2RenderingContext::COLOR_ATTACHMENT0, WebGl2RenderingContext::TEXTURE_2D, Some(self.output_texture.as_ref().unwrap().as_ref()), 0);

            gl.draw_elements_with_i32(WebGl2RenderingContext::TRIANGLES, self.indices_count, WebGl2RenderingContext::UNSIGNED_SHORT, 0);

            std::mem::swap(&mut self.input_texture, &mut self.output_texture);
        }

        // rerender special texture into a regular RGBA UNSIGNED_BYTE texture
        gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(self.input_texture.as_ref().unwrap().as_ref()));

        gl.framebuffer_texture_2d(WebGl2RenderingContext::FRAMEBUFFER, WebGl2RenderingContext::COLOR_ATTACHMENT0, WebGl2RenderingContext::TEXTURE_2D, Some(self.render_texture.as_ref().unwrap().as_ref()), 0);

        gl.use_program(Some(&self.reaction_diffusion_render));
        gl.draw_elements_with_i32(WebGl2RenderingContext::TRIANGLES, self.indices_count, WebGl2RenderingContext::UNSIGNED_SHORT, 0);

        gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(self.render_texture.as_ref().unwrap().as_ref()));

        // reset back to rendering to canvas
        gl.bind_framebuffer(WebGl2RenderingContext::FRAMEBUFFER, None);
        viewport.set_gl_viewport_to_current_width_height();

        // let kernel = [
        //     [0.05, 0.2, 0.05],
        //     [0.2, -1.0, 0.2],
        //     [0.05, 0.2, 0.05]
        // ];
        //
        // for x in 0..CELLS_WIDTH {
        //     for y in 0..CELLS_HEIGHT {
        //         let i = cell_xy_to_index(x, y);
        //
        //         let a = u16float_to_float(self.cells[i]);
        //         let b = u16float_to_float(self.cells[i + 1]);
        //
        //         let mut nabla_squared_a = 0.0;
        //         let mut nabla_squared_b = 0.0;
        //
        //         for nx in -1..=1 {
        //             for ny in -1..=1 {
        //                 let i = cell_xy_to_index(x + nx, y + ny);
        //
        //                 let a = u16float_to_float(self.cells[i]);
        //                 nabla_squared_a += a * kernel[(2 - (ny + 1)) as usize][(nx + 1) as usize];
        //
        //                 let b = u16float_to_float(self.cells[i + 1]);
        //                 nabla_squared_b += b * kernel[(2 - (ny + 1)) as usize][(nx + 1) as usize];
        //             }
        //         }
        //
        //         let a_prime = a + (D_A * nabla_squared_a - a * b * b + F * (1.0 - a)) * DELTA_T;
        //         let b_prime = b + (D_B * nabla_squared_b + a * b * b - (K + F) * b) * DELTA_T;
        //
        //         self.cells_next[i] = float_to_u16float(a_prime);
        //         self.cells_next[i + 1] = float_to_u16float(b_prime);
        //     }
        // }
        //
        // std::mem::swap(&mut self.cells, &mut self.cells_next);
        //
        // unsafe {
        //     let view = js_sys::Uint16Array::view(&self.cells);
        //
        //     gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_array_buffer_view_and_src_offset(
        //         WebGl2RenderingContext::TEXTURE_2D,
        //         0,
        //         WebGl2RenderingContext::RG16UI as i32,
        //         CELLS_WIDTH,
        //         CELLS_HEIGHT,
        //         0,
        //         WebGl2RenderingContext::RG_INTEGER,
        //         WebGl2RenderingContext::UNSIGNED_SHORT,
        //         &view,
        //         0,
        //     ).unwrap();
        // }
    }

    fn on_render(&mut self) {
        let gl = self.viewport.borrow().gl();

        gl.bind_vertex_array(self.vao.as_ref());
        gl.use_program(Some(&self.unlit_texture_bicubic));

        gl.draw_elements_with_i32(WebGl2RenderingContext::TRIANGLES, self.indices_count, WebGl2RenderingContext::UNSIGNED_SHORT, 0);
    }
}

fn u16float_to_float(value: u16) -> f32 {
    return value as f32 / u16::MAX as f32;
}

fn float_to_u16float(value: f32) -> u16 {
    return (value * u16::MAX as f32).round() as u16;
}

fn cell_xy_to_index(x: i32, y: i32) -> usize {
    let mut x = x;
    let mut y = y;
    if x < 0 {
        x += CELLS_WIDTH
    }
    if y < 0 {
        y += CELLS_HEIGHT;
    }
    return (((x % CELLS_WIDTH) + (y % CELLS_HEIGHT) * CELLS_WIDTH) * 2) as usize;
}

fn create_and_bind_texture(gl: &WebGl2RenderingContext, filter_mode: u32, wrap_mode: u32) -> Option<WebGlTexture> {
    let texture = gl.create_texture();
    gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, texture.as_ref());
    // gl.pixel_storei(WebGl2RenderingContext::UNPACK_ALIGNMENT, 1);

    gl.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_MIN_FILTER, filter_mode as i32);
    gl.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_MAG_FILTER, filter_mode as i32);
    gl.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_WRAP_S, wrap_mode as i32);
    gl.tex_parameteri(WebGl2RenderingContext::TEXTURE_2D, WebGl2RenderingContext::TEXTURE_WRAP_T, wrap_mode as i32);

    return texture;
}

fn init_quad(gl: &WebGl2RenderingContext, program: &WebGlProgram, vertices: &[f32; 12]) -> WebGlVertexArrayObject {
    let vao = gl.create_vertex_array();
    gl.bind_vertex_array(vao.as_ref());

    let position_attribute_location = gl.get_attrib_location(&program, "a_position");
    let buffer = gl.create_buffer();
    gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, buffer.as_ref());

    unsafe {
        let positions_array_buf_view = js_sys::Float32Array::view(vertices);

        gl.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ARRAY_BUFFER,
            &positions_array_buf_view,
            WebGl2RenderingContext::STATIC_DRAW,
        );
    }

    gl.vertex_attrib_pointer_with_i32(position_attribute_location as u32, 3, WebGl2RenderingContext::FLOAT, false, 0, 0);
    gl.enable_vertex_attrib_array(position_attribute_location as u32);

    let uv_attribute_location = gl.get_attrib_location(&program, "a_uv");
    let buffer = gl.create_buffer();
    gl.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, buffer.as_ref());

    let uv = [0.0, 1.0, 1.0, 1.0, 1.0, 0.0, 0.0];

    unsafe {
        let uv_array_buf_view = js_sys::Float32Array::view(&uv);

        gl.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ARRAY_BUFFER,
            &uv_array_buf_view,
            WebGl2RenderingContext::STATIC_DRAW,
        );
    }

    gl.vertex_attrib_pointer_with_i32(uv_attribute_location as u32, 2, WebGl2RenderingContext::FLOAT, false, 0, 0);
    gl.enable_vertex_attrib_array(uv_attribute_location as u32);

    let buffer = gl.create_buffer();
    gl.bind_buffer(WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER, buffer.as_ref());

    let indices = [0, 1, 2, 0, 2, 3];

    unsafe {
        let indices_array_buf_view = js_sys::Uint16Array::view(&indices);

        gl.buffer_data_with_array_buffer_view(
            WebGl2RenderingContext::ELEMENT_ARRAY_BUFFER,
            &indices_array_buf_view,
            WebGl2RenderingContext::STATIC_DRAW,
        );
    }

    return vao.unwrap();
}

fn init_cells(cells: &mut [u16; (CELLS_WIDTH * CELLS_HEIGHT * 2) as usize]) {
    for i in (0..cells.len()).step_by(2) {
        cells[i] = float_to_u16float(1.0);
        cells[i + 1] = float_to_u16float(0.0);
    }

    for x in (CELLS_WIDTH / 2 - 10)..(CELLS_WIDTH / 2 + 10) {
        for y in (CELLS_HEIGHT / 2 - 10)..(CELLS_HEIGHT / 2 + 10) {
            let i = cell_xy_to_index(x, y);
            cells[i + 1] = float_to_u16float(1.0);
        }
    }
}

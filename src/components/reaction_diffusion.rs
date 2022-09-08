use std::rc::Rc;
use glam::{Mat4, Quat, Vec3};
use web_sys::{WebGl2RenderingContext, WebGlFramebuffer, WebGlProgram, WebGlTexture, WebGlVertexArrayObject};
use crate::{Component, create_shader_program};
use crate::engine::app::App;
use crate::engine::app::input::Button::Left;
use crate::utils::{distance, lerp};

const SIMULATION_SCALE: f32 = 1.5;

pub struct ReactionDiffusion {
    quad_vao: Option<WebGlVertexArrayObject>,
    render_texture_vao: Option<WebGlVertexArrayObject>,
    unlit_texture_bicubic: Rc<WebGlProgram>,
    reaction_diffusion: Rc<WebGlProgram>,
    reaction_diffusion_render: Rc<WebGlProgram>,
    basic_rg16ui: WebGlProgram,
    unlit_color_on_rg16ui: WebGlProgram,
    indices_count: i32,
    fbo: Option<Box<WebGlFramebuffer>>,
    input_texture: Option<Box<WebGlTexture>>,
    output_texture: Option<Box<WebGlTexture>>,
    render_texture: Option<Box<WebGlTexture>>,
    width: i32,
    height: i32,
    last_mouse_position: (i32, i32),
}

impl ReactionDiffusion {
    pub fn new(app: &App, unlit_texture_bicubic: Rc<WebGlProgram>, reaction_diffusion: Rc<WebGlProgram>, reaction_diffusion_render: Rc<WebGlProgram>) -> Self {
        let gl = app.gl();

        let width = (app.screen().width() as f32 / SIMULATION_SCALE).round() as i32;
        let height = (app.screen().height() as f32 / SIMULATION_SCALE).round() as i32;

        return Self {
            quad_vao: None,
            render_texture_vao: None,
            unlit_texture_bicubic,
            reaction_diffusion,
            reaction_diffusion_render,
            basic_rg16ui: create_shader_program(&gl, include_str!("../shaders/basic_RG16UI.vert"), include_str!("../shaders/basic_RG16UI.frag")),
            unlit_color_on_rg16ui: create_shader_program(&gl, include_str!("../shaders/unlit_color_on_RG16UI.vert"), include_str!("../shaders/unlit_color_on_RG16UI.frag")),
            indices_count: 0,
            fbo: None,
            input_texture: None,
            output_texture: None,
            render_texture: None,
            width,
            height,
            last_mouse_position: (-1, -1),
        };
    }
}

impl Component for ReactionDiffusion {
    fn on_add_to_game_object(&mut self, app: &App) {
        let gl = app.gl();

        let vertices = [-0.5, 0.5, 0.0, 0.5, 0.5, 0.0, 0.5, -0.5, 0.0, -0.5, -0.5, 0.0];
        self.quad_vao = Some(init_quad(&gl, &self.unlit_color_on_rg16ui, &vertices));

        let vertices = [-1.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0, -1.0, 0.0, -1.0, -1.0, 0.0];
        self.render_texture_vao = Some(init_quad(&gl, &self.unlit_texture_bicubic, &vertices));

        self.indices_count = 6;

        // gl.use_program(Some(&self.unlit_texture_bicubic));
        // let model = Mat4::from_scale_rotation_translation(Vec3::new(self.width as f32, self.height as f32, 1.0), Quat::IDENTITY, Vec3::new(0.0, 0.0, 0.0));
        // let u_model_loc = gl.get_uniform_location(self.unlit_texture_bicubic.as_ref(), "u_model");
        // gl.uniform_matrix4fv_with_f32_array(u_model_loc.as_ref(), false, model.as_ref());

        gl.use_program(Some(&self.reaction_diffusion));
        let u_kernel_loc = gl.get_uniform_location(self.reaction_diffusion.as_ref(), "u_kernel");
        let kernel: [f32; 9] = [
            0.05, 0.2, 0.05,
            0.2, -1.0, 0.2,
            0.05, 0.2, 0.05
        ];
        gl.uniform1fv_with_f32_array(u_kernel_loc.as_ref(), &kernel);

        self.input_texture = Some(Box::new(create_and_bind_texture(&gl, WebGl2RenderingContext::NEAREST, WebGl2RenderingContext::REPEAT).unwrap()));
        let mut cells: Vec<u16> = vec![0; (self.width * self.height * 2) as usize];
        init_cells(&mut cells, self.width, self.height);
        unsafe {
            let view = js_sys::Uint16Array::view(&cells);

            gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_array_buffer_view_and_src_offset(
                WebGl2RenderingContext::TEXTURE_2D,
                0,
                WebGl2RenderingContext::RG16UI as i32,
                self.width,
                self.height,
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
            self.width,
            self.height,
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
            self.width,
            self.height,
            0,
            WebGl2RenderingContext::RGBA,
            WebGl2RenderingContext::UNSIGNED_BYTE,
            None,
        ).unwrap();

        self.fbo = Some(Box::new(gl.create_framebuffer().unwrap()));
    }

    fn on_resize(&mut self, app: &App) {
        let gl = app.gl();

        let width = app.screen().width();
        let height = app.screen().height();
        self.width = (width as f32 / SIMULATION_SCALE).round() as i32;
        self.height = (height as f32 / SIMULATION_SCALE).round() as i32;

        gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(self.output_texture.as_ref().unwrap().as_ref()));
        gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_array_buffer_view(
            WebGl2RenderingContext::TEXTURE_2D,
            0,
            WebGl2RenderingContext::RG16UI as i32,
            self.width,
            self.height,
            0,
            WebGl2RenderingContext::RG_INTEGER,
            WebGl2RenderingContext::UNSIGNED_SHORT,
            None,
        ).unwrap();

        gl.bind_vertex_array(self.render_texture_vao.as_ref());
        gl.bind_framebuffer(WebGl2RenderingContext::FRAMEBUFFER, Some(self.fbo.as_ref().unwrap().as_ref()));
        gl.viewport(0, 0, self.width, self.height);
        gl.use_program(Some(&self.basic_rg16ui));

        gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(self.input_texture.as_ref().unwrap().as_ref()));

        gl.framebuffer_texture_2d(WebGl2RenderingContext::FRAMEBUFFER, WebGl2RenderingContext::COLOR_ATTACHMENT0, WebGl2RenderingContext::TEXTURE_2D, Some(self.output_texture.as_ref().unwrap().as_ref()), 0);

        gl.draw_elements_with_i32(WebGl2RenderingContext::TRIANGLES, self.indices_count, WebGl2RenderingContext::UNSIGNED_SHORT, 0);

        gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(self.input_texture.as_ref().unwrap().as_ref()));
        gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_array_buffer_view(
            WebGl2RenderingContext::TEXTURE_2D,
            0,
            WebGl2RenderingContext::RG16UI as i32,
            self.width,
            self.height,
            0,
            WebGl2RenderingContext::RG_INTEGER,
            WebGl2RenderingContext::UNSIGNED_SHORT,
            None,
        ).unwrap();

        std::mem::swap(&mut self.input_texture, &mut self.output_texture);

        gl.bind_texture(WebGl2RenderingContext::TEXTURE_2D, Some(self.render_texture.as_ref().unwrap().as_ref()));
        gl.tex_image_2d_with_i32_and_i32_and_i32_and_format_and_type_and_opt_array_buffer_view(
            WebGl2RenderingContext::TEXTURE_2D,
            0,
            WebGl2RenderingContext::RGBA as i32,
            self.width,
            self.height,
            0,
            WebGl2RenderingContext::RGBA,
            WebGl2RenderingContext::UNSIGNED_BYTE,
            None,
        ).unwrap();
    }

    fn on_update(&mut self, app: &App) {
        let gl = app.gl();

        if app.input().get_button_down(Left) || app.input().get_button(Left) && app.input().mouse_delta_position() != (0, 0) {
            gl.bind_vertex_array(self.quad_vao.as_ref());
            gl.bind_framebuffer(WebGl2RenderingContext::FRAMEBUFFER, Some(self.fbo.as_ref().unwrap().as_ref()));
            gl.viewport(0, 0, self.width, self.height);
            gl.use_program(Some(&self.unlit_color_on_rg16ui));

            let mouse_position = app.input().mouse_position();

            let mat = Mat4::IDENTITY;
            let loc = gl.get_uniform_location(&self.unlit_color_on_rg16ui, "u_view");
            gl.uniform_matrix4fv_with_f32_array(loc.as_ref(), false, mat.as_ref());

            let mat = Mat4::orthographic_rh_gl(0.0, app.screen().width() as f32, app.screen().height() as f32, 0.0, -1.0, 1.0);
            let loc = gl.get_uniform_location(&self.unlit_color_on_rg16ui, "u_projection");
            gl.uniform_matrix4fv_with_f32_array(loc.as_ref(), false, mat.as_ref());

            gl.framebuffer_texture_2d(WebGl2RenderingContext::FRAMEBUFFER, WebGl2RenderingContext::COLOR_ATTACHMENT0, WebGl2RenderingContext::TEXTURE_2D, Some(self.input_texture.as_ref().unwrap().as_ref()), 0);

            if self.last_mouse_position == (-1, -1) {
                self.last_mouse_position = mouse_position;
            }

            let distance = distance(self.last_mouse_position, mouse_position);
            let num_circles = distance.round().clamp(1.0, f32::MAX) as i32;

            for i in 0..=num_circles {
                let t = i as f32 / num_circles as f32;
                let x = lerp(self.last_mouse_position.0 as f32, mouse_position.0 as f32, t);
                let y = lerp(self.last_mouse_position.1 as f32, mouse_position.1 as f32, t);

                let mat = Mat4::from_scale_rotation_translation(Vec3::new(10.0, 10.0, 1.0), Quat::IDENTITY, Vec3::new(x, y, 0.0));
                let loc = gl.get_uniform_location(&self.unlit_color_on_rg16ui, "u_model");
                gl.uniform_matrix4fv_with_f32_array(loc.as_ref(), false, mat.as_ref());

                gl.draw_elements_with_i32(WebGl2RenderingContext::TRIANGLES, self.indices_count, WebGl2RenderingContext::UNSIGNED_SHORT, 0);
            }

            self.last_mouse_position = mouse_position;
        } else {
            self.last_mouse_position = (-1, -1);
        }

        // do the reaction diffusion with a shader for the computation
        let iterations = 15;
        gl.bind_vertex_array(self.render_texture_vao.as_ref());
        gl.bind_framebuffer(WebGl2RenderingContext::FRAMEBUFFER, Some(self.fbo.as_ref().unwrap().as_ref()));
        gl.viewport(0, 0, self.width, self.height);
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
        gl.viewport(0, 0, app.screen().width(), app.screen().height());
    }

    fn on_render(&mut self, app: &App) {
        let gl = app.gl();

        gl.bind_vertex_array(self.render_texture_vao.as_ref());
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

fn cell_xy_to_index(x: i32, y: i32, width: i32, height: i32) -> usize {
    let mut x = x;
    let mut y = y;
    if x < 0 {
        x += width
    }
    if y < 0 {
        y += height;
    }
    return (((x % width) + (y % height) * width) * 2) as usize;
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

fn init_cells(cells: &mut Vec<u16>, width: i32, height: i32) {
    for i in (0..cells.len()).step_by(2) {
        cells[i] = float_to_u16float(1.0);
        cells[i + 1] = float_to_u16float(0.0);
    }

    for x in (width / 2 - 10)..(width / 2 + 10) {
        for y in (height / 2 - 10)..(height / 2 + 10) {
            let i = cell_xy_to_index(x, y, width, height);
            cells[i + 1] = float_to_u16float(1.0);
        }
    }
}

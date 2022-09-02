use std::cell::RefCell;
use std::rc::Rc;
use glam::{Mat4, Vec4};
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::Closure;
use web_sys::WebGlProgram;
use crate::{Component, Viewport};

pub struct CameraPan {
    viewport: Rc<RefCell<Viewport>>,
    program: Rc<WebGlProgram>
}

impl CameraPan {
    pub fn new(viewport: Rc<RefCell<Viewport>>, program: Rc<WebGlProgram>) -> Self {
        return Self {
            viewport,
            program
        };
    }
}

impl Component for CameraPan {
    fn on_add_to_game_object(&mut self) {
        let viewport = &self.viewport;

        let canvas = viewport.borrow().canvas();
        let gl = viewport.borrow().gl();

        gl.use_program(Some(self.program.as_ref()));
        let u_orthographic_size_loc = gl.get_uniform_location(self.program.as_ref(), "u_orthographic_size");
        gl.uniform1f(u_orthographic_size_loc.as_ref(), viewport.borrow().orthographic_size());
        let program = Rc::clone(&self.program);
        viewport.borrow_mut().set_orthographic_size_change(Some(Box::new(move |viewport: &Viewport| {
            viewport.gl().use_program(Some(program.as_ref()));
            viewport.gl().uniform1f(u_orthographic_size_loc.as_ref(), viewport.orthographic_size());
        })));

        let u_canvas_height_loc = gl.get_uniform_location(self.program.as_ref(), "u_canvas_height");
        gl.uniform1i(u_canvas_height_loc.as_ref(), viewport.borrow().height());
        let program = Rc::clone(&self.program);
        viewport.borrow_mut().set_height_change(Some(Box::new(move |viewport: &Viewport| {
            viewport.gl().use_program(Some(program.as_ref()));
            viewport.gl().uniform1i(u_canvas_height_loc.as_ref(), viewport.height());
        })));

        let u_view_loc = gl.get_uniform_location(self.program.as_ref(), "u_view");
        gl.uniform_matrix4fv_with_f32_array(u_view_loc.as_ref(), false, viewport.borrow().view().as_ref());
        let program = Rc::clone(&self.program);
        viewport.borrow_mut().set_view_change(Some(Box::new(move |viewport: &Viewport| {
            viewport.gl().use_program(Some(program.as_ref()));
            viewport.gl().uniform_matrix4fv_with_f32_array(u_view_loc.as_ref(), false, viewport.view().as_ref());
        })));

        let u_projection_loc = gl.get_uniform_location(self.program.as_ref(), "u_projection");
        gl.uniform_matrix4fv_with_f32_array(u_projection_loc.as_ref(), false, viewport.borrow().projection().as_ref());
        let program = Rc::clone(&self.program);
        viewport.borrow_mut().set_projection_change(Some(Box::new(move |viewport: &Viewport| {
            viewport.gl().use_program(Some(program.as_ref()));
            viewport.gl().uniform_matrix4fv_with_f32_array(u_projection_loc.as_ref(), false, viewport.projection().as_ref());
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

        // let viewport_outer = Rc::clone(&viewport);
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

    fn on_pre_render(&mut self) {
        self.viewport.borrow().update_uniforms_in_shader();
    }
}

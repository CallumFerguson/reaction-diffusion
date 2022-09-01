use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::WebGl2RenderingContext;
use glam::{Mat4, Vec3};

struct Updater<T> {
    value: T,
    update: Vec<Option<Box<dyn Fn(&Viewport)>>>,
    needs_update: bool,
}

impl<T> Updater<T> {
    pub fn get_value(&self) -> &T { &self.value }

    pub fn set_value(&mut self, value: T) {
        self.value = value;
        self.needs_update = true;
    }

    pub fn set_update(&mut self, update: Option<Box<dyn Fn(&Viewport)>>) {
        self.update.push(update);
        self.needs_update = true;
    }

    pub fn update_if_not_none(&self, viewport: &Viewport) {
        for u in &self.update {
            match u {
                Some(value) => {
                    (value.as_ref())(viewport);
                }
                None => ()
            }
        }
    }
}

pub struct Viewport {
    width: i32,
    height_updater: RefCell<Updater<i32>>,

    canvas: Rc<web_sys::HtmlCanvasElement>,
    gl: Rc<WebGl2RenderingContext>,

    camera_pos: Vec3,

    view_updater: RefCell<Updater<Mat4>>,
    projection_updater: RefCell<Updater<Mat4>>,

    orthographic_size_updater: RefCell<Updater<f32>>,
}

impl Viewport {
    pub fn new() -> Rc<RefCell<Self>> {
        let window = Rc::new(web_sys::window().expect("no global `window` exists"));
        let document = window.document().unwrap();
        let body = document.body().expect("document should have a body");

        let width = window.inner_width().unwrap().as_f64().unwrap() as i32;
        let height = window.inner_height().unwrap().as_f64().unwrap() as i32;

        let canvas = document.create_element("canvas").unwrap();
        canvas.set_attribute("width", &width.to_string()).unwrap();
        canvas.set_attribute("height", &height.to_string()).unwrap();
        body.append_child(&canvas).unwrap();
        let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();

        let gl = canvas
            .get_context("webgl2").unwrap()
            .unwrap()
            .dyn_into::<WebGl2RenderingContext>().unwrap();

        let orthographic_size = 512.0 / 2.0 + 10.0; // 50.0
        // let camera_pos = Vec3::new(16.0, -5.0, 0.0);
        let camera_pos = Vec3::new(0.0, 0.0, 0.0);

        // let view = Mat4::from_translation(camera_pos).inverse();
        // let projection = Mat4::orthographic_rh_gl(-aspect_ratio * orthographic_size, aspect_ratio * orthographic_size, -1.0 * orthographic_size, 1.0 * orthographic_size, -1.0, 1.0);

        let mut viewport = Self {
            width,
            height_updater: RefCell::new(Updater { value: height, update: Vec::new(), needs_update: true }),
            canvas: Rc::new(canvas),
            gl: Rc::new(gl),
            camera_pos,
            view_updater: RefCell::new(Updater {value: Mat4::IDENTITY, update: Vec::new(), needs_update: true}),
            projection_updater: RefCell::new(Updater {value: Mat4::IDENTITY, update: Vec::new(), needs_update: true}),
            orthographic_size_updater: RefCell::new(Updater { value: orthographic_size, update: Vec::new(), needs_update: true }),
        };
        viewport.recalculate_view();
        viewport.recalculate_projection();
        let viewport = Rc::new(RefCell::new(viewport));

        let window_outer = Rc::clone(&window);
        let screen_outer = Rc::clone(&viewport);
        let event_closure = Closure::<dyn FnMut()>::new(move || {
            let width = window.inner_width().unwrap().as_f64().unwrap() as i32;
            let height = window.inner_height().unwrap().as_f64().unwrap() as i32;

            viewport.borrow_mut().width = width;
            viewport.borrow_mut().height_updater.borrow_mut().set_value(height);

            viewport.borrow().canvas.set_attribute("width", &width.to_string()).unwrap();
            viewport.borrow().canvas.set_attribute("height", &height.to_string()).unwrap();

            viewport.borrow().gl.viewport(0, 0, width, height);

            viewport.borrow_mut().recalculate_view();
            viewport.borrow_mut().recalculate_projection();
        });
        let viewport = screen_outer;
        let window = window_outer;
        window.add_event_listener_with_callback("resize", event_closure.as_ref().unchecked_ref()).unwrap();
        event_closure.forget();

        return viewport;
    }

    pub fn width(&self) -> i32 { self.width }
    pub fn height(&self) -> i32 { *self.height_updater.borrow().get_value() }
    pub fn aspect_ratio(&self) -> f32 { self.width as f32 / self.height() as f32 }
    pub fn canvas(&self) -> Rc<web_sys::HtmlCanvasElement> { Rc::clone(&self.canvas) }
    pub fn gl(&self) -> Rc<WebGl2RenderingContext> { Rc::clone(&self.gl) }
    pub fn camera_pos(&self) -> &Vec3 { &self.camera_pos }
    pub fn orthographic_size(&self) -> f32 { *self.orthographic_size_updater.borrow().get_value() }
    pub fn view(&self) -> Mat4 { *self.view_updater.borrow().get_value() }
    pub fn projection(&self) -> Mat4 { *self.projection_updater.borrow().get_value() }

    pub  fn set_gl_viewport_to_current_width_height(&self) {
        self.gl.viewport(0, 0, self.width(), self.height());
    }

    pub fn set_height_change(&mut self, height_change: Option<Box<dyn Fn(&Self)>>) {
        self.height_updater.borrow_mut().set_update(height_change);
    }

    pub fn set_orthographic_size_change(&mut self, orthographic_size_change: Option<Box<dyn Fn(&Self)>>) {
        self.orthographic_size_updater.borrow_mut().set_update(orthographic_size_change);
    }

    pub fn set_view_change(&mut self, view_change: Option<Box<dyn Fn(&Self)>>) {
        self.view_updater.borrow_mut().set_update(view_change);
    }

    pub fn set_projection_change(&mut self, projection_change: Option<Box<dyn Fn(&Self)>>) {
        self.projection_updater.borrow_mut().set_update(projection_change);
    }

    pub fn set_orthographic_size(&mut self, orthographic_size: f32) {
        self.orthographic_size_updater.borrow_mut().set_value(orthographic_size);
        self.recalculate_projection();
    }

    pub fn set_camera_pos(&mut self, camera_pos: Vec3) {
        self.camera_pos = camera_pos;
        self.recalculate_view();
    }

    pub fn update_uniforms_in_shader(&self) {
        handle_updater(&self.height_updater, self);
        handle_updater(&self.orthographic_size_updater, self);
        handle_updater(&self.view_updater, self);
        handle_updater(&self.projection_updater, self);
    }

    fn recalculate_view(&mut self) {
        let view = Mat4::from_translation(self.camera_pos).inverse();
        self.view_updater.borrow_mut().set_value(view);
    }

    fn recalculate_projection(&mut self) {
        let aspect_ratio = self.aspect_ratio();
        let orthographic_size = self.orthographic_size();
        let projection = Mat4::orthographic_rh_gl(-aspect_ratio * orthographic_size, aspect_ratio * orthographic_size, -1.0 * orthographic_size, 1.0 * orthographic_size, -1.0, 1.0);
        self.projection_updater.borrow_mut().set_value(projection);
    }
}

fn handle_updater<T>(updater: &RefCell<Updater<T>>, viewport: &Viewport) {
    if updater.borrow().needs_update {
        updater.borrow().update_if_not_none(viewport);
        updater.borrow_mut().needs_update = false;
    }
}
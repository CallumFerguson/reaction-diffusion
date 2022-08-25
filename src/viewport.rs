use std::cell::RefCell;
use std::ops::MulAssign;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::WebGl2RenderingContext;
use glam::{Mat4, Vec3, Vec4};
use js_sys::Atomics::or;

struct Updater<T> {
    value: T,
    update: Option<Box<dyn Fn(&Viewport)>>,
    needs_update: bool,
}

impl<T> Updater<T> {
    pub fn get_value(&self) -> &T { &self.value }

    pub fn set_value(&mut self, value: T) {
        self.value = value;
        self.needs_update = true;
    }

    pub fn set_update(&mut self, update: Option<Box<dyn Fn(&Viewport)>>) {
        self.update = update;
        self.needs_update = true;
    }

    pub fn update_if_not_none(&self, viewport: &Viewport) {
        match &self.update {
            Some(value) => {
                (value.as_ref())(viewport);
            }
            None => ()
        }
    }
}

pub struct Viewport {
    width: i32,
    height_updater: RefCell<Updater<i32>>,

    canvas: Rc<web_sys::HtmlCanvasElement>,
    context: Rc<WebGl2RenderingContext>,

    camera_pos: Vec3,

    view: Mat4,
    projection: Mat4,

    orthographic_size_updater: RefCell<Updater<f32>>,
}

impl Viewport {
    pub fn new() -> Rc<RefCell<Self>> {
        let window = Rc::new(web_sys::window().expect("no global `window` exists"));
        let document = window.document().unwrap();
        let body = document.body().expect("document should have a body");

        let width = window.inner_width().unwrap().as_f64().unwrap() as i32;
        let height = window.inner_height().unwrap().as_f64().unwrap() as i32;
        let aspect_ratio = width as f32 / height as f32;

        let canvas = document.create_element("canvas").unwrap();
        canvas.set_attribute("width", &width.to_string()).unwrap();
        canvas.set_attribute("height", &height.to_string()).unwrap();
        body.append_child(&canvas).unwrap();
        let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>().unwrap();

        let context = canvas
            .get_context("webgl2").unwrap()
            .unwrap()
            .dyn_into::<WebGl2RenderingContext>().unwrap();

        let orthographic_size = 50.0;
        let camera_pos = Vec3::new(16.0, -5.0, 0.0);

        let view = Mat4::from_translation(camera_pos).inverse();
        let projection = Mat4::orthographic_rh_gl(-aspect_ratio * orthographic_size, aspect_ratio * orthographic_size, -1.0 * orthographic_size, 1.0 * orthographic_size, -1.0, 1.0);

        let viewport = Self {
            width,
            height_updater: RefCell::new(Updater { value: height, update: None, needs_update: true }),
            canvas: Rc::new(canvas),
            context: Rc::new(context),
            camera_pos,
            view,
            projection,
            orthographic_size_updater: RefCell::new(Updater { value: orthographic_size, update: None, needs_update: true }),
        };
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

            viewport.borrow().context.viewport(0, 0, width, height);
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
    pub fn context(&self) -> Rc<WebGl2RenderingContext> { Rc::clone(&self.context) }
    pub fn camera_pos(&self) -> &Vec3 { &self.camera_pos }
    pub fn orthographic_size(&self) -> f32 { *self.orthographic_size_updater.borrow().get_value() }
    pub fn view(&self) -> &Mat4 { &self.view }
    pub fn projection(&self) -> &Mat4 { &self.projection }

    pub fn set_height_change(&mut self, height_change: Option<Box<dyn Fn(&Self)>>) {
        self.height_updater.borrow_mut().set_update(height_change);
    }

    pub fn set_orthographic_size_change(&mut self, orthographic_size_change: Option<Box<dyn Fn(&Self)>>) {
        self.orthographic_size_updater.borrow_mut().set_update(orthographic_size_change);
    }

    pub fn set_orthographic_size(&mut self, orthographic_size: f32) {
        self.orthographic_size_updater.borrow_mut().set_value(orthographic_size);
    }

    pub fn update_uniforms_in_shader(&self) {
        handle_updater(&self.height_updater, self);
        handle_updater(&self.orthographic_size_updater, self);
    }
}

fn handle_updater<T>(updater: &RefCell<Updater<T>>, viewport: &Viewport) {
    if updater.borrow().needs_update {
        updater.borrow().update_if_not_none(viewport);
        updater.borrow_mut().needs_update = false;
    }
}
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::WebGl2RenderingContext;

pub struct Screen {
    width: i32,
    height: i32,

    canvas: web_sys::HtmlCanvasElement
}

impl Screen {
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

        let screen = Self {
            width,
            height,
            canvas
        };
        let screen = Rc::new(RefCell::new(screen));

        let context = screen.borrow().canvas()
            .get_context("webgl2").unwrap()
            .unwrap()
            .dyn_into::<WebGl2RenderingContext>().unwrap();

        let window_outer = Rc::clone(&window);
        let screen_outer = Rc::clone(&screen);
        let event_closure = Closure::<dyn FnMut()>::new(move || {
            let width = window.inner_width().unwrap().as_f64().unwrap() as i32;
            let height = window.inner_height().unwrap().as_f64().unwrap() as i32;

            screen.borrow_mut().width = width;
            screen.borrow_mut().height = height;

            screen.borrow().canvas.set_attribute("width", &width.to_string()).unwrap();
            screen.borrow().canvas.set_attribute("height", &height.to_string()).unwrap();

            context.viewport(0, 0, width, height);
        });
        let screen = screen_outer;
        let window = window_outer;
        window.add_event_listener_with_callback("resize", event_closure.as_ref().unchecked_ref()).unwrap();
        event_closure.forget();

        return screen;
    }

    pub fn width(&self) -> i32 {
        return self.width;
    }

    pub fn height(&self) -> i32 {
        return self.height;
    }

    pub fn aspect_ratio(&self) -> f32 {
        return self.width as f32 / self.height as f32;
    }

    pub fn canvas(&self) -> &web_sys::HtmlCanvasElement {
        return &self.canvas;
    }
}

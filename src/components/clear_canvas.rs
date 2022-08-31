use std::cell::RefCell;
use std::rc::Rc;
use web_sys::WebGl2RenderingContext;
use crate::{Component, Viewport};

pub struct ClearCanvas {
    viewport: Rc<RefCell<Viewport>>,
}

impl ClearCanvas {
    pub fn new(viewport: Rc<RefCell<Viewport>>) -> Self {
        return Self {
            viewport
        };
    }
}

impl Component for ClearCanvas {
    fn on_pre_render(&mut self) {
        let gl = self.viewport.borrow().gl();
        gl.clear_color(0.0, 0.0, 0.0, 1.0);
        gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
    }
}

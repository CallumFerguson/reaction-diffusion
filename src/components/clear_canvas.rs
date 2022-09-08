use std::cell::RefCell;
use std::rc::Rc;
use web_sys::WebGl2RenderingContext;
use crate::{Component, Viewport};
use crate::engine::app::App;

pub struct ClearCanvas {}

impl ClearCanvas {
    pub fn new() -> Self {
        return Self {};
    }
}

impl Component for ClearCanvas {
    fn on_pre_render(&mut self, app: &App) {
        let gl = app.gl();
        gl.clear_color(0.0, 0.0, 0.0, 1.0);
        gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);
    }
}

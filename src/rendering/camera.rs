use std::borrow::Borrow;
use web_sys::WebGl2RenderingContext;
use crate::{Component, GameObject};
use crate::engine::app::App;

pub struct Camera {}

impl Camera {
    pub fn new() -> Self {
        return Self {};
    }
}

impl Component for Camera {
    fn on_render(&mut self, game_object: &mut GameObject, app: &App) {
        let gl = app.gl();
        gl.clear_color(0.0, 0.0, 0.0, 1.0);
        gl.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

        let game_objects = app.game_objects();
        let game_objects_len = game_objects.len();
        for i in 0..game_objects_len {
            let game_object = game_objects[i].try_borrow();
            if game_object.is_ok() {
                let components = game_object.unwrap().components();
                for component in components.borrow_mut().iter_mut() {
                    let game_object = &game_objects[i];
                    component.component().borrow_mut().draw(&mut game_object.borrow_mut(), &app);
                }
            }
        }

        // let components = game_object.components();
        // for component in components.borrow_mut().iter_mut() {
        //     // component.component().borrow_mut().draw(game_object, &app);
        // }
    }
}

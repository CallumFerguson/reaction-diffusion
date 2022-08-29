use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::Closure;
use crate::{GameObject};

pub struct App {
    game_objects: Vec<GameObject>
}

impl App {
    pub fn new() -> Rc<RefCell<App>> {
        let app = App {
            game_objects: Vec::new()
        };
        let app = Rc::new(RefCell::new(app));

        let window = Rc::new(web_sys::window().expect("no global `window` exists"));

        let animation_loop_closure = Rc::new(RefCell::new(None::<Closure<dyn FnMut(_)>>));
        let animation_loop_closure_outer = animation_loop_closure.clone();

        let window = Rc::new(window);
        let window_outer = Rc::clone(&window);

        let mut start_time = -1.0;
        let mut last_unscaled_time = 0.0;

        let app_outer = Rc::clone(&app);
        *animation_loop_closure_outer.borrow_mut() = Some(Closure::<dyn FnMut(_)>::new(move |now: f64| {
            let mut app = app.borrow_mut();

            let now = now * 0.001;
            if start_time < 0.0 {
                start_time = now;
            }
            let unscaled_time = now - start_time;
            let _delta_time = unscaled_time - last_unscaled_time;
            last_unscaled_time = unscaled_time;
            // console_log!("{}", 1.0 / delta_time);

            for game_object in &mut app.game_objects {
                for component in game_object.components_iter() {
                    component.on_update();
                }
            }

            for game_object in &mut app.game_objects {
                for component in game_object.components_iter() {
                    component.on_render_clear();
                }
            }

            for game_object in &mut app.game_objects {
                for component in game_object.components_iter() {
                    component.on_render_object();
                }
            }

            window.request_animation_frame(animation_loop_closure.borrow().as_ref().unwrap().as_ref().unchecked_ref()).expect("request_animation_frame failed");
        }));
        let window = window_outer;
        window.request_animation_frame(animation_loop_closure_outer.borrow().as_ref().unwrap().as_ref().unchecked_ref()).expect("request_animation_frame failed");
        let app = app_outer;

        return app;
    }
}

impl App {
    pub fn add_game_object(&mut self, game_object: GameObject) {
        self.game_objects.push(game_object);
    }
}
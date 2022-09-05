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

        let resized = Rc::new(RefCell::new(false));
        let screen_width = Rc::new(RefCell::new(0));
        let screen_height = Rc::new(RefCell::new(0));

        let window_outer = Rc::clone(&window);
        let resized_outer = Rc::clone(&resized);
        let screen_width_outer = Rc::clone(&screen_width);
        let screen_height_outer = Rc::clone(&screen_height);
        let event_closure = Closure::<dyn FnMut()>::new(move || {
            let width = window.inner_width().unwrap().as_f64().unwrap() as i32;
            let height = window.inner_height().unwrap().as_f64().unwrap() as i32;

            *resized.borrow_mut() = true;
            *screen_width.borrow_mut() = width;
            *screen_height.borrow_mut() = height;
        });
        let window = window_outer;
        let resized = resized_outer;
        let screen_width = screen_width_outer;
        let screen_height = screen_height_outer;
        window.add_event_listener_with_callback("resize", event_closure.as_ref().unchecked_ref()).unwrap();
        event_closure.forget();

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

            if *resized.borrow() {
                *resized.borrow_mut() = false;
                for game_object in &mut app.game_objects {
                    for component in game_object.components_iter() {
                        component.on_resize(*screen_width.borrow(), *screen_height.borrow());
                    }
                }
            }

            for game_object in &mut app.game_objects {
                for component in game_object.components_iter() {
                    component.on_pre_render();
                }
            }

            for game_object in &mut app.game_objects {
                for component in game_object.components_iter() {
                    component.on_render();
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
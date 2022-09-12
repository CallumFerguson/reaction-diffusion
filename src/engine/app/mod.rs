use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::Closure;
use web_sys::{Document, HtmlCanvasElement, HtmlElement, WebGl2RenderingContext};
use crate::{ClearCanvas, Component, GameObject, ReactionDiffusionUI};
use crate::engine::app::input::Input;
use crate::engine::app::screen::Screen;
use crate::engine::app::time::Time;

pub mod input;
pub mod screen;
pub mod time;

pub struct App {
    canvas: HtmlCanvasElement,
    gl: Option<WebGl2RenderingContext>,
    game_objects: RefCell<Vec<GameObject>>,
    game_objects_to_be_added: RefCell<Vec<GameObject>>,
    document: Document,
    body: HtmlElement,
    input: Input,
    screen: Screen,
    time: Time,
}

impl App {
    pub fn new() -> Rc<RefCell<App>> {
        let window = Rc::new(web_sys::window().expect("no global `window` exists"));
        let document = window.document().unwrap();
        let body = document.body().expect("document should have a body");

        let width = window.inner_width().unwrap().as_f64().unwrap() as i32;
        let height = window.inner_height().unwrap().as_f64().unwrap() as i32;

        let canvas = document.create_element("canvas").unwrap();
        canvas.set_id("main_canvas");
        canvas.set_attribute("width", &width.to_string()).unwrap();
        canvas.set_attribute("height", &height.to_string()).unwrap();
        body.append_child(&canvas).unwrap();
        let canvas: HtmlCanvasElement = canvas.dyn_into::<HtmlCanvasElement>().unwrap();

        let app = App {
            canvas,
            gl: None,
            game_objects: RefCell::new(Vec::new()),
            game_objects_to_be_added: RefCell::new(Vec::new()),
            document,
            body,
            input: Input::new(),
            screen: Screen::new((width, height)),
            time: Time::new(),
        };
        let app = Rc::new(RefCell::new(app));

        let window_outer = Rc::clone(&window);
        let app_outer = Rc::clone(&app);
        let event_closure = Closure::<dyn FnMut()>::new(move || {
            let mut app = app.borrow_mut();

            let width = window.inner_width().unwrap().as_f64().unwrap() as i32;
            let height = window.inner_height().unwrap().as_f64().unwrap() as i32;
            app.screen.set_size((width, height));

            let canvas = app.canvas();
            canvas.set_attribute("width", &width.to_string()).unwrap();
            canvas.set_attribute("height", &height.to_string()).unwrap();
        });
        let window = window_outer;
        let app = app_outer;
        window.add_event_listener_with_callback("resize", event_closure.as_ref().unchecked_ref()).unwrap();
        event_closure.forget();

        let app_outer = Rc::clone(&app);
        let event_closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::MouseEvent| {
            app.borrow_mut().input.set_buttons(event.buttons());
        });
        let app = app_outer;
        app.borrow().canvas().add_event_listener_with_callback("mousedown", event_closure.as_ref().unchecked_ref()).unwrap();
        app.borrow().canvas().add_event_listener_with_callback("mouseup", event_closure.as_ref().unchecked_ref()).unwrap();
        event_closure.forget();

        let app_outer = Rc::clone(&app);
        let event_closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::MouseEvent| {
            app.borrow_mut().input.set_buttons(event.buttons());
            app.borrow_mut().input.set_mouse_position((event.offset_x(), event.offset_y()));
        });
        let app = app_outer;
        app.borrow().canvas().add_event_listener_with_callback("mousemove", event_closure.as_ref().unchecked_ref()).unwrap();
        event_closure.forget();

        let animation_loop_closure = Rc::new(RefCell::new(None::<Closure<dyn FnMut(_)>>));
        let animation_loop_closure_outer = animation_loop_closure.clone();

        let window = Rc::new(window);
        let window_outer = Rc::clone(&window);

        let mut start_time = -1.0;
        let mut last_unscaled_time = 0.0;

        let app_outer = Rc::clone(&app);
        *animation_loop_closure_outer.borrow_mut() = Some(Closure::<dyn FnMut(_)>::new(move |now: f64| {
            {
                let now = now * 0.001;
                if start_time < 0.0 {
                    start_time = now;
                }
                let unscaled_time = now - start_time;
                let delta_time = unscaled_time - last_unscaled_time;
                last_unscaled_time = unscaled_time;

                {
                    let mut app_mut = app.borrow_mut();
                    app_mut.time.set_delta_time(delta_time as f32);
                    app_mut.time.set_unscaled_time(unscaled_time as f32);
                }

                let app = app.borrow();

                {
                    let mut game_objects_to_be_added = app.game_objects_to_be_added.borrow_mut();
                    let mut game_objects = app.game_objects.borrow_mut();
                    while game_objects_to_be_added.len() > 0 {
                        game_objects.push(game_objects_to_be_added.pop().unwrap());
                    }
                }

                let game_objects_len = app.game_objects.borrow().len();

                for i in 0..game_objects_len {
                    let components = app.game_objects.borrow_mut()[i].components();
                    ;
                    for component in components.borrow_mut().iter_mut() {
                        let game_object = &mut app.game_objects.borrow_mut()[i];
                        if !component.had_first_update() {
                            component.component().borrow_mut().on_first_update(game_object, &app);
                            component.set_had_first_update();
                        }
                    }
                }

                for i in 0..game_objects_len {
                    let components = app.game_objects.borrow_mut()[i].components();
                    ;
                    for component in components.borrow_mut().iter_mut() {
                        let game_object = &mut app.game_objects.borrow_mut()[i];
                        component.component().borrow_mut().on_update(game_object, &app);
                    }
                }

                for i in 0..game_objects_len {
                    let components = app.game_objects.borrow_mut()[i].components();
                    ;
                    for component in components.borrow_mut().iter_mut() {
                        let game_object = &mut app.game_objects.borrow_mut()[i];
                        component.component().borrow_mut().on_pre_render(game_object, &app);
                    }
                }

                for i in 0..game_objects_len {
                    let components = app.game_objects.borrow_mut()[i].components();
                    ;
                    for component in components.borrow_mut().iter_mut() {
                        let game_object = &mut app.game_objects.borrow_mut()[i];
                        component.component().borrow_mut().on_render(game_object, &app);
                    }
                }

                for i in 0..game_objects_len {
                    let components = app.game_objects.borrow_mut()[i].components();
                    ;
                    for component in components.borrow_mut().iter_mut() {
                        let game_object = &mut app.game_objects.borrow_mut()[i];
                        component.component().borrow_mut().on_late_update(game_object, &app);
                    }
                }
            }

            {
                let mut app = app.borrow_mut();

                let mouse_position = app.input.mouse_position();
                app.input.set_last_mouse_position(mouse_position);

                let buttons = app.input.buttons();
                app.input.set_last_buttons(buttons);
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
    pub fn add_game_object(&self, game_object: GameObject) {
        self.game_objects_to_be_added.borrow_mut().push(game_object);
    }

    pub fn input(&self) -> &Input { &self.input }
    pub fn screen(&self) -> &Screen { &self.screen }
    pub fn time(&self) -> &Time { &self.time }

    pub fn canvas(&self) -> &HtmlCanvasElement {
        return &self.canvas;
    }

    pub fn document(&self) -> &Document { return &self.document; }
    pub fn body(&self) -> &HtmlElement { return &self.body; }

    pub fn init_gl(&mut self) {
        if self.gl == None {
            self.gl = Some(self.canvas
                .get_context("webgl2").unwrap()
                .unwrap()
                .dyn_into::<WebGl2RenderingContext>().unwrap());
        }
    }

    pub fn gl(&self) -> &WebGl2RenderingContext {
        return self.gl.as_ref().unwrap();
    }
}

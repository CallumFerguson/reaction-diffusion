use std::any::Any;
use std::cell::RefCell;
use std::rc::Rc;
use crate::Component;
use crate::engine::app::App;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

pub struct ReactionDiffusionUI {
    callbacks: Vec<Closure<dyn FnMut()>>,
    clear_button: Rc<RefCell<bool>>,
    random_preset_button: Rc<RefCell<bool>>,
}

impl ReactionDiffusionUI {
    pub fn new() -> Self {
        return Self {
            callbacks: Vec::new(),
            // on_click_clear_functions: Rc::new(RefCell::new(Vec::new())),
            clear_button: Rc::new(RefCell::new(false)),
            random_preset_button: Rc::new(RefCell::new(false)),
        };
    }
}

impl ReactionDiffusionUI {
    // pub fn add_clear_click_callback(&self, callback: impl FnMut() + 'static) {
    //     self.on_click_clear_functions.borrow_mut().push(Box::new(callback));
    // }

    pub fn clear_button(&self) -> bool {
        return *self.clear_button.borrow();
    }

    pub fn random_preset_button(&self) -> bool {
        return *self.random_preset_button.borrow();
    }
}

impl Component for ReactionDiffusionUI {
    fn on_add_to_game_object(&mut self, app: &App) {
        let ui_div_parent = app.document().create_element("div").unwrap().dyn_into::<HtmlElement>().unwrap();
        let style = ui_div_parent.style();
        style.set_property("width", "100%").unwrap();
        style.set_property("height", "100%").unwrap();
        style.set_property("position", "absolute").unwrap();
        style.set_property("left", "0").unwrap();
        style.set_property("top", "0").unwrap();
        style.set_property("pointer-events", "none").unwrap();
        style.set_property("draggable", "false").unwrap();
        style.set_property("-webkit-user-select", "none").unwrap();
        style.set_property("user-select", "none").unwrap();
        app.body().append_child(&ui_div_parent).unwrap();

        let ui_div = app.document().create_element("div").unwrap().dyn_into::<HtmlElement>().unwrap();
        let style = ui_div.style();
        style.set_property("pointer-events", "auto").unwrap();
        style.set_property("display", "inline-block").unwrap();
        ui_div_parent.append_child(&ui_div).unwrap();

        let controls = app.document().create_element("div").unwrap().dyn_into::<HtmlElement>().unwrap();
        let style = controls.style();
        style.set_property("width", "125px").unwrap();
        style.set_property("background-color", "white").unwrap();
        style.set_property("padding", "5px").unwrap();
        style.set_property("border-radius", "10px").unwrap();
        style.set_property("margin", "10px").unwrap();
        ui_div.append_child(&controls).unwrap();

        let label = app.document().create_element("label").unwrap().dyn_into::<HtmlElement>().unwrap();
        label.set_attribute("for", "feed-input").unwrap();
        label.set_inner_text("Feed rate");
        controls.append_child(&label).unwrap();

        let input = app.document().create_element("input").unwrap().dyn_into::<HtmlElement>().unwrap();
        input.set_id("feed-input");
        input.set_attribute("type", "range").unwrap();
        input.set_attribute("min", "0").unwrap();
        input.set_attribute("max", "0.1").unwrap();
        input.set_attribute("step", "0.001").unwrap();
        input.set_attribute("value", "0.055").unwrap();
        input.style().set_property("width", "calc(100% - 5px)").unwrap();
        controls.append_child(&input).unwrap();

        let label = app.document().create_element("label").unwrap().dyn_into::<HtmlElement>().unwrap();
        label.set_attribute("for", "kill-input").unwrap();
        label.set_inner_text("Kill rate");
        controls.append_child(&label).unwrap();

        let input = app.document().create_element("input").unwrap().dyn_into::<HtmlElement>().unwrap();
        input.set_id("kill-input");
        input.set_attribute("type", "range").unwrap();
        input.set_attribute("min", "0").unwrap();
        input.set_attribute("max", "0.1").unwrap();
        input.set_attribute("step", "0.001").unwrap();
        input.set_attribute("value", "0.062").unwrap();
        input.style().set_property("width", "calc(100% - 5px)").unwrap();
        controls.append_child(&input).unwrap();

        let button = app.document().create_element("button").unwrap().dyn_into::<HtmlElement>().unwrap();
        button.set_attribute("type", "button").unwrap();
        button.set_inner_text("clear");
        button.style().set_property("display", "block").unwrap();
        button.style().set_property("margin", "5px").unwrap();
        controls.append_child(&button).unwrap();

        let clear_button = Rc::clone(&self.clear_button);
        let callback = Closure::<dyn FnMut()>::new(move || {
            *clear_button.borrow_mut() = true;
        });
        button.add_event_listener_with_callback("click", callback.as_ref().unchecked_ref()).unwrap();
        self.callbacks.push(callback);

        let button = app.document().create_element("button").unwrap().dyn_into::<HtmlElement>().unwrap();
        button.set_attribute("type", "button").unwrap();
        button.set_inner_text("random preset");
        button.style().set_property("display", "block").unwrap();
        button.style().set_property("margin", "5px").unwrap();
        controls.append_child(&button).unwrap();

        let random_preset_button = Rc::clone(&self.random_preset_button);
        let callback = Closure::<dyn FnMut()>::new(move || {
            *random_preset_button.borrow_mut() = true;
        });
        button.add_event_listener_with_callback("click", callback.as_ref().unchecked_ref()).unwrap();
        self.callbacks.push(callback);
    }

    fn on_late_update(&mut self, app: &App) {
        *self.clear_button.borrow_mut() = false;
        *self.random_preset_button.borrow_mut() = false;
    }
}

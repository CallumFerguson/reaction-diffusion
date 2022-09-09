use crate::Component;
use crate::engine::app::App;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::HtmlElement;

pub struct ReactionDiffusionUI {}

impl ReactionDiffusionUI {
    pub fn new() -> Self {
        return Self {};
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
        style.set_property("background-color", "white").unwrap();
        style.set_property("width", "200px").unwrap();
        style.set_property("height", "200px").unwrap();
        ui_div.append_child(&controls).unwrap();

        let input = app.document().create_element("input").unwrap().dyn_into::<HtmlElement>().unwrap();
        input.set_attribute("type", "range").unwrap();
        input.set_attribute("min", "1").unwrap();
        input.set_attribute("max", "100").unwrap();
        input.set_attribute("value", "50").unwrap();
        controls.append_child(&input).unwrap();
    }
}

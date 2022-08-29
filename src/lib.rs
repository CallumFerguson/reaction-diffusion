use std::mem;
use wasm_bindgen::prelude::*;
use console_error_panic_hook::hook;
use std::rc::Rc;
use crate::engine::component::Component;
use crate::engine::game_object::GameObject;
use crate::game_of_life::GameOfLife;
use crate::rendering::viewport::Viewport;

#[macro_use]
mod utils;
mod engine;
mod game_of_life;
mod rendering;

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    std::panic::set_hook(Box::new(hook));

    console_log!("starting webgl");

    let app = crate::engine::app::App::new();
    let mut app = app.borrow_mut();

    let viewport = Viewport::new();

    let mut game_of_life_object = GameObject::new();
    game_of_life_object.add_component(Box::new(GameOfLife::new(Rc::clone(&viewport), 0)));
    app.add_game_object(game_of_life_object);

    let mut game_of_life_object = GameObject::new();
    game_of_life_object.add_component(Box::new(GameOfLife::new(Rc::clone(&viewport), 1)));
    app.add_game_object(game_of_life_object);

    // mem::forget(app);

    Ok(())
}

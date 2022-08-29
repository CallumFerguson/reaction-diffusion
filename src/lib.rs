use std::mem;
use wasm_bindgen::prelude::*;
use console_error_panic_hook::hook;
use std::rc::Rc;
use crate::camera_pan::CameraPan;
use crate::engine::component::Component;
use crate::engine::game_object::GameObject;
use crate::game_of_life::GameOfLife;
use crate::rendering::viewport::Viewport;
use crate::utils::create_shader_program;

#[macro_use]
mod utils;
mod engine;
mod game_of_life;
mod rendering;
mod camera_pan;

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    std::panic::set_hook(Box::new(hook));

    console_log!("starting webgl");

    let app = crate::engine::app::App::new();
    let mut app = app.borrow_mut();

    let viewport = Viewport::new();
    let context = viewport.borrow().context();

    let program = Rc::new(create_shader_program(&context, include_str!("shader.vert"), include_str!("shader.frag")));
    context.use_program(Some(&program));

    let mut camera_pan = GameObject::new();
    camera_pan.add_component(Box::new(CameraPan::new(Rc::clone(&viewport), Rc::clone(&program))));
    app.add_game_object(camera_pan);

    let start_cells = "........................O...........
                       ......................O.O...........
                       ............OO......OO............OO
                       ...........O...O....OO............OO
                       OO........O.....O...OO..............
                       OO........O...O.OO....O.O...........
                       ..........O.....O.......O...........
                       ...........O...O....................
                       ............OO......................";
    let mut game_of_life_object = GameObject::new();
    game_of_life_object.add_component(Box::new(GameOfLife::new(Rc::clone(&viewport), Rc::clone(&program), start_cells, (0, 0))));
    app.add_game_object(game_of_life_object);

    // let mut game_of_life_object = GameObject::new();
    // game_of_life_object.add_component(Box::new(GameOfLife::new(Rc::clone(&viewport), Rc::clone(&program), start_cells, (50, 0))));
    // app.add_game_object(game_of_life_object);

    Ok(())
}

use wasm_bindgen::prelude::*;
use console_error_panic_hook::hook;
use std::rc::Rc;
use crate::components::camera_pan::CameraPan;
use crate::components::clear_canvas::ClearCanvas;
use crate::engine::component::Component;
use crate::engine::game_object::GameObject;
use crate::components::game_of_life::GameOfLife;
use crate::rendering::viewport::Viewport;
use crate::utils::create_shader_program;
use crate::components::square::Square;
use crate::components::reaction_diffusion::ReactionDiffusion;

#[macro_use]
mod utils;
mod engine;
mod rendering;
mod components;

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    std::panic::set_hook(Box::new(hook));

    console_log!("starting webgl");

    let app = crate::engine::app::App::new();
    let mut app = app.borrow_mut();

    let viewport = Viewport::new();
    let context = viewport.borrow().context();

    let program = Rc::new(create_shader_program(&context, include_str!("shaders/shader.vert"), include_str!("shaders/shader.frag")));
    context.use_program(Some(&program));

    let mut game_manager = GameObject::new();
    game_manager.add_component(Box::new(CameraPan::new(Rc::clone(&viewport), Rc::clone(&program))));
    game_manager.add_component(Box::new(ClearCanvas::new(Rc::clone(&viewport))));
    app.add_game_object(game_manager);

    // let start_cells = "........................O...........
    //                    ......................O.O...........
    //                    ............OO......OO............OO
    //                    ...........O...O....OO............OO
    //                    OO........O.....O...OO..............
    //                    OO........O...O.OO....O.O...........
    //                    ..........O.....O.......O...........
    //                    ...........O...O....................
    //                    ............OO......................";
    // let mut game_of_life_object = GameObject::new();
    // game_of_life_object.add_component(Box::new(GameOfLife::new(Rc::clone(&viewport), Rc::clone(&program), start_cells, (0, 0))));
    // app.add_game_object(game_of_life_object);

    // let mut square_object = GameObject::new();
    // square_object.add_component(Box::new(Square::new(Rc::clone(&viewport), Rc::clone(&program))));
    // app.add_game_object(square_object);

    let mut reaction_diffusion_object = GameObject::new();
    reaction_diffusion_object.add_component(Box::new(ReactionDiffusion::new(Rc::clone(&viewport), Rc::clone(&program))));
    app.add_game_object(reaction_diffusion_object);

    Ok(())
}

use wasm_bindgen::prelude::*;
use console_error_panic_hook::hook;
use std::rc::Rc;
use crate::components::clear_canvas::ClearCanvas;
use crate::engine::component::Component;
use crate::engine::game_object::GameObject;
use crate::rendering::viewport::Viewport;
use crate::utils::create_shader_program;
use crate::components::reaction_diffusion::ReactionDiffusion;

#[macro_use]
mod utils;
mod engine;
mod rendering;
mod components;

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    std::panic::set_hook(Box::new(hook));

    console_log!("starting app");

    let app = crate::engine::app::App::new();
    app.borrow_mut().init_gl();
    let app = app.borrow();

    let gl = app.gl();

    let game_manager = GameObject::new();

    game_manager.add_component(Box::new(ClearCanvas::new()), &app);

    let basic_bicubic = Rc::new(create_shader_program(&gl, include_str!("shaders/basic_bicubic.vert"), include_str!("shaders/basic_bicubic.frag")));
    let reaction_diffusion = Rc::new(create_shader_program(&gl, include_str!("shaders/reaction_diffusion.vert"), include_str!("shaders/reaction_diffusion.frag")));
    let reaction_diffusion_render = Rc::new(create_shader_program(&gl, include_str!("shaders/reaction_diffusion_render.vert"), include_str!("shaders/reaction_diffusion_render.frag")));
    game_manager.add_component(Box::new(ReactionDiffusion::new(&app, Rc::clone(&basic_bicubic), Rc::clone(&reaction_diffusion), Rc::clone(&reaction_diffusion_render))), &app);

    app.add_game_object(game_manager);

    Ok(())
}

use wasm_bindgen::prelude::*;
use console_error_panic_hook::hook;
use crate::components::clear_canvas::ClearCanvas;
use crate::engine::component::Component;
use crate::engine::game_object::GameObject;
use crate::utils::create_shader_program;
use crate::components::reaction_diffusion::ReactionDiffusion;
use crate::components::reaction_diffusion_ui::ReactionDiffusionUI;

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

    let mut game_manager = GameObject::new();

    game_manager.add_component(ClearCanvas::new(), &app);

    game_manager.add_component(ReactionDiffusionUI::new(), &app);

    let rdui = game_manager.get_component::<ReactionDiffusionUI>().unwrap();
    game_manager.add_component(ReactionDiffusion::new(&app, rdui), &app);

    app.add_game_object(game_manager);

    Ok(())
}

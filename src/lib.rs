use wasm_bindgen::prelude::*;
use console_error_panic_hook::hook;
use crate::engine::component::Component;
use crate::engine::game_object::GameObject;
use crate::utils::create_shader_program;
use crate::components::reaction_diffusion::ReactionDiffusion;
use crate::components::reaction_diffusion_ui::ReactionDiffusionUI;
use crate::components::fps_tracker::FPSTracker;
use crate::rendering::camera::Camera;

#[macro_use]
mod utils;
mod engine;
mod rendering;
mod components;

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    std::panic::set_hook(Box::new(hook));

    console_log!("starting app");

    let app_ref = crate::engine::app::App::new();
    app_ref.borrow_mut().init_gl();
    let app = app_ref.borrow();

    let mut camera = GameObject::new();
    camera.add_component(Camera::new(), &app);
    app.add_game_object(camera);

    let mut game_manager = GameObject::new();
    // game_manager.add_component(FPSTracker::new(), &app);
    game_manager.add_component(ReactionDiffusionUI::new(), &app);
    game_manager.add_component(ReactionDiffusion::new(&app), &app);
    app.add_game_object(game_manager);

    Ok(())
}

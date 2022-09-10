use wasm_bindgen::prelude::*;
use std::rc::Rc;
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

    let app_ref = crate::engine::app::App::new();
    app_ref.borrow_mut().init_gl();
    let app = app_ref.borrow();

    let gl = app.gl();

    let mut game_manager = GameObject::new();

    game_manager.add_component(ClearCanvas::new(), &app);

    game_manager.add_component(ReactionDiffusionUI::new(), &app);

    let rdui = game_manager.get_component::<ReactionDiffusionUI>().unwrap();
    let rdui_clone = Rc::clone(&rdui);
    game_manager.add_component(ReactionDiffusion::new(&app, rdui), &app);

    let rd = game_manager.get_component::<ReactionDiffusion>().unwrap();
    let app_inner = Rc::clone(&app_ref);
    rdui_clone.borrow().add_clear_click_callback(move || {
        rd.borrow().clear(app_inner.borrow().gl());
    });

    app.add_game_object(game_manager);

    Ok(())
}

use wasm_bindgen::prelude::*;
use console_error_panic_hook::hook;
use std::rc::Rc;
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

    let game_manager = GameObject::new();

    game_manager.add_component(Box::new(ClearCanvas::new()), &app);

    let reaction_diffusion_ui = Box::new(ReactionDiffusionUI::new());
    game_manager.add_component(reaction_diffusion_ui, &app);

    game_manager.add_component(Box::new(ReactionDiffusion::new(&app)), &app);

    let component = game_manager.get_component::<ReactionDiffusionUI>().unwrap();
    let mut component = component.borrow_mut();
    let component = component.as_any();
    component.downcast_ref::<ReactionDiffusionUI>().unwrap().add_clear_click_callback(Box::new(|| {
        console_log!("reee 2");
    }));

    app.add_game_object(game_manager);

    Ok(())
}

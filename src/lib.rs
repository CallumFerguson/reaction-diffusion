use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{WebGl2RenderingContext, WebGlProgram, WebGlShader};
use console_error_panic_hook::hook;
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;
use glam::{Mat4, Vec4};
use crate::engine::viewport::Viewport;
use crate::engine::component::Component;
use crate::engine::engine_state::EngineState;
use crate::engine::game_object::GameObject;
use crate::game_of_life::GameOfLife;
use crate::utils::create_shader_program;

#[macro_use]
mod utils;
mod engine;
mod game_of_life;

#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    std::panic::set_hook(Box::new(hook));

    console_log!("starting webgl");

    let window = Rc::new(web_sys::window().expect("no global `window` exists"));

    let engine_state = EngineState::new(Viewport::new());

    let animation_loop_closure = Rc::new(RefCell::new(None::<Closure::<dyn FnMut(_)>>));
    let animation_loop_closure_outer = animation_loop_closure.clone();

    let window = Rc::new(window);
    let window_outer = Rc::clone(&window);

    let mut start_time = -1.0;
    let mut last_unscaled_time = 0.0;

    let mut game_objects = Vec::<GameObject>::new();

    let mut game_of_life_object = GameObject::new();
    game_of_life_object.add_component(&engine_state, Box::new(GameOfLife::new(0)));
    game_objects.push(game_of_life_object);

    let mut game_of_life_object = GameObject::new();
    game_of_life_object.add_component(&engine_state, Box::new(GameOfLife::new(1)));
    game_objects.push(game_of_life_object);

    *animation_loop_closure_outer.borrow_mut() = Some(Closure::<dyn FnMut(_)>::new(move |now: f64| {
        let now = now * 0.001;
        if start_time < 0.0 {
            start_time = now;
        }
        let unscaled_time = now - start_time;
        let _delta_time = unscaled_time - last_unscaled_time;
        last_unscaled_time = unscaled_time;
        // console_log!("{}", 1.0 / delta_time);

        for game_object in &mut game_objects {
            for component in game_object.components_iter() {
                component.on_update(&engine_state);
            }
        }

        let context = engine_state.viewport().borrow().context();
        context.clear_color(0.0, 0.0, 0.0, 1.0);
        context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

        for game_object in &mut game_objects {
            for component in game_object.components_iter() {
                component.on_render_object(&engine_state);
            }
        }

        window.request_animation_frame(animation_loop_closure.borrow().as_ref().unwrap().as_ref().unchecked_ref()).expect("request_animation_frame failed");
    }));
    let window = window_outer;
    window.request_animation_frame(animation_loop_closure_outer.borrow().as_ref().unwrap().as_ref().unchecked_ref()).expect("request_animation_frame failed");

    Ok(())
}

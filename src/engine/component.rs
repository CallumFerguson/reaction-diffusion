use crate::engine::engine_state::EngineState;

pub trait Component {
    fn on_add_to_game_object(&mut self, engine_state: &EngineState) {}
    fn on_update(&mut self, engine_state: &EngineState) {}
    fn on_render_object(&mut self, engine_state: &EngineState) {}
}

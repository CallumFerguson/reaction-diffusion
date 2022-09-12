use std::any::Any;
use crate::engine::app::App;
use crate::GameObject;

pub trait Component {
    fn on_add_to_game_object(&mut self, game_object: &mut GameObject, app: &App) {}
    fn on_first_update(&mut self, game_object: &mut GameObject, app: &App) {}
    fn on_update(&mut self, game_object: &mut GameObject, app: &App) {}
    fn on_pre_render(&mut self, game_object: &mut GameObject, app: &App) {}
    fn on_render(&mut self, game_object: &mut GameObject, app: &App) {}
    fn on_late_update(&mut self, game_object: &mut GameObject, app: &App) {}
}

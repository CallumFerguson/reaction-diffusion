use std::any::Any;
use crate::engine::app::App;

pub trait Component {
    fn as_any(&mut self) -> &mut dyn Any;
    fn on_add_to_game_object(&mut self, app: &App) {}
    fn on_resize(&mut self, app: &App) {}
    fn on_update(&mut self, app: &App) {}
    fn on_pre_render(&mut self, app: &App) {}
    fn on_render(&mut self, app: &App) {}
}

pub trait Component {
    fn on_add_to_game_object(&mut self) {}
    fn on_update(&mut self) {}
    fn on_render_clear(&mut self) {}
    fn on_render_object(&mut self) {}
}

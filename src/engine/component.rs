pub trait Component {
    fn on_add_to_game_object(&self) {}
    fn on_update(&self) {}
    fn on_render_object(&self) {}
}

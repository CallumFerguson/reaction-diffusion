pub trait Component {
    fn on_add_to_game_object(&mut self) {}
    fn on_update(&mut self) {}
    fn on_resize(&mut self, width: i32, height: i32) {}
    fn on_pre_render(&mut self) {}
    fn on_render(&mut self) {}
}

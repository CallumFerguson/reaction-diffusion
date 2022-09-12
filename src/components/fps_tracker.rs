use std::collections::VecDeque;
use crate::{Component, GameObject};
use crate::engine::app::App;

pub struct FPSTracker {
    frames: VecDeque<(f32, f32)>,
}

impl FPSTracker {
    pub fn new() -> Self {
        return Self {
            frames: VecDeque::new(),
        };
    }
}

impl Component for FPSTracker {
    fn on_update(&mut self, game_object: &mut GameObject, app: &App) {
        self.frames.push_back((app.time().unscaled_time(), app.time().delta_time()));
        while self.frames.len() > 0 && app.time().unscaled_time() > self.frames[0].0 + 1.0 {
            self.frames.pop_front();
        }
        let avg_delta_time = self.frames.iter().map(|e| e.1).sum::<f32>() / self.frames.len() as f32;
        // let min_delta_time = self.frames.iter().map(|e| e.1).reduce(f32::max).unwrap();
        // let max_delta_time = self.frames.iter().map(|e| e.1).reduce(f32::min).unwrap();
        // console_log!("fps (avg={}, min={}, max={})", (1.0 / avg_delta_time).round() as i32, (1.0 / min_delta_time).round() as i32, (1.0 / max_delta_time).round() as i32);
        console_log!("{}", (1.0 / avg_delta_time).round() as i32);
    }
}

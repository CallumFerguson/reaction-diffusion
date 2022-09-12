pub struct Time {
    delta_time: f32,
    unscaled_time: f32,
}

impl Time {
    pub fn new() -> Self {
        return Self {
            delta_time: 0.0,
            unscaled_time: 0.0,
        };
    }
}

impl Time {
    pub fn delta_time(&self) -> f32 { self.delta_time }
    pub fn set_delta_time(&mut self, delta_time: f32) { self.delta_time = delta_time; }

    pub fn unscaled_time(&self) -> f32 { self.unscaled_time }
    pub fn set_unscaled_time(&mut self, unscaled_time: f32) { self.unscaled_time = unscaled_time; }
}

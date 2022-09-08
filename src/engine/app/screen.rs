pub struct Screen {
    size: (i32, i32),
}

impl Screen {
    pub fn new(size: (i32, i32)) -> Self {
        return Self {
            size,
        };
    }
}

impl Screen {
    pub fn size(&self) -> (i32, i32) {
        return self.size;
    }

    pub fn width(&self) -> i32 {
        return self.size.0;
    }

    pub fn height(&self) -> i32 {
        return self.size.1;
    }

    pub fn set_size(&mut self, size: (i32, i32)) {
        self.size = size;
    }

    pub fn aspect_ratio(&self) -> f32 {
        return self.width() as f32 / self.height() as f32;
    }
}

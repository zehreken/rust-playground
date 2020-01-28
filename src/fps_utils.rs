pub struct FpsCounter {
    time_as_ms: u128,
    frames: i32,
}

impl FpsCounter {
    pub fn new() -> FpsCounter {
        FpsCounter {
            time_as_ms: 0,
            frames: 0,
        }
    }

    pub fn tick(&mut self, delta_ms: u128) -> i32 {
        self.time_as_ms += delta_ms;
        self.frames += 1;
        if self.time_as_ms >= 1000 {
            self.time_as_ms -= 1000;
            let temp = self.frames;
            self.frames = 0;
            return temp;
        }

        0
    }
}

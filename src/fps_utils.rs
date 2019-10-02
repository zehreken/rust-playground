pub mod fps_utils {
    pub struct FpsCounter {
        time_as_second: u128,
        frames: i32,
    }

    impl FpsCounter {
        pub fn new() -> FpsCounter {
            FpsCounter {
                time_as_second: 0,
                frames: 0,
            }
        }
    }
}

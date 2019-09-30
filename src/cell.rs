pub mod cell {
    use std::fmt;
    #[derive(Debug, Copy, Clone)]
    pub struct Point {
        pub x: i32,
        pub y: i32,
    }
    #[derive(Debug, Copy, Clone)]
    pub struct Cell {
        pub position: Point,
        pub neighbours: [Point; 8],
        pub current_state: i32,
        pub future_state: i32,
        pub on_count: i32,
    }

    impl fmt::Display for Point {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "x: {}, y: {}", self.x, self.y)
        }
    }
    impl fmt::Display for Cell {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "position {}\n", self.position)?;
            write!(f, "neighbours: [\n")?;
            for i in 0..8 {
                write!(f, "{}: {}\n", i, self.neighbours[i])?;
            }
            write!(f, "]\n")?;
            write!(f, "current_state: {}\n", self.current_state)?;
            write!(f, "future_state: {}\n", self.future_state)?;
            write!(f, "on_count: {}\n", self.on_count)
        }
    }
    fn setPosition(cell: Cell, row: i32, column: i32) {}
}

pub fn run() {
    let integer_point = Point { x: 4, y: 5 };
    let float_point = Point { x: 2.3, y: 3.4 };

    let float_integer_point = MixedPoint { x: 2.1, y: 5 };
}

fn largest<T>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T { // Returns a reference to x
        &self.x
    }
}

impl Point<f32> { // This is only for type f32
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct MixedPoint<T, U> {
    x: T,
    y: U,
}

use List::{Cons, Nil};

pub fn run() {
    let b = Box::new(5);
    println!("b = {}", b);

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}

enum List {
    Cons(i32, Box<List>), // By using a box , we've broken the infinite, recursive chain
    Nil,
}

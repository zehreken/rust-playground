pub fn run() {
    let foo = Foo::new();
    Foo::method_one();
    foo.method_three();
    foo.method_two();

    let bar = Bar::new();
    Bar::method_one();
    bar.method_three();
    bar.method_two();
}

pub trait MyTrait {
    fn method_one();
    fn method_two(self); // This method moves ownership
    fn method_three(&self);
}

// Foo
pub struct Foo {
    foo_int: i32,
}

impl Foo {
    fn new() -> Self {
        Foo { foo_int: 0 }
    }
}

impl MyTrait for Foo {
    fn method_one() {
        println!("Non-associated method of Foo");
    }

    fn method_two(self) {
        println!("[Moved]Associated method of Foo {}", self.foo_int);
    }

    fn method_three(&self) {
        println!("Associated method of Foo {}", self.foo_int);
    }
}

// Bar
pub struct Bar {
    bar_string: String,
}

impl Bar {
    fn new() -> Self {
        Bar {
            bar_string: "nonsense".to_string(),
        }
    }
}
impl MyTrait for Bar {
    fn method_one() {
        println!("Non-associated method of Bar");
    }

    fn method_two(self) {
        println!("[Moved]Associated method of Bar {}", self.bar_string);
    }

    fn method_three(&self) {
        println!("Associated method of Bar {}", self.bar_string);
    }
}

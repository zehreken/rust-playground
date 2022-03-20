use std::thread;

pub fn run() {
    let mut boom: Vec<Box<dyn MyTrait + Send>> = vec![];

    for i in 0..10 {
        let foo = Foo::new(i);
        let bar = Bar::new(format!("bar {}", i));
        boom.push(Box::new(foo));
        boom.push(Box::new(bar));
    }

    process(&boom);

    let h = thread::spawn(move || {
        for t in boom.iter() {
            t.method();
        }
    });
    h.join();
}

fn process(objects: &[Box<dyn MyTrait + Send>]) {
    for obj in objects {
        obj.method();
    }
}

pub trait MyTrait: Send {
    fn method(&self);
}

// Foo
pub struct Foo {
    foo_int: i32,
}

impl Foo {
    fn new(v: i32) -> Self {
        Foo { foo_int: v }
    }
}

impl MyTrait for Foo {
    fn method(&self) {
        println!("Associated method of Foo {}", self.foo_int);
    }
}

// Bar
pub struct Bar {
    bar_string: String,
}

impl Bar {
    fn new(v: String) -> Self {
        Bar { bar_string: v }
    }
}
impl MyTrait for Bar {
    fn method(&self) {
        println!("Associated method of Bar {}", self.bar_string);
    }
}

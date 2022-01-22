struct Foo {
    v: Vec<i32>,
}

impl Foo {
    fn new(val: i32, size: usize) -> Self {
        Self { v: vec![val; size] }
    }

    fn print(&self) {
        println!("v: {:?}", self.v);
    }
}

struct FooWithReference<'a> {
    v: &'a Vec<i32>,
}

impl<'a> FooWithReference<'a> {
    fn new(v: &'a Vec<i32>) -> Self {
        Self { v }
    }

    fn print(&self) {
        println!("v: {:?}", self.v);
    }
}
pub fn run() {
    let foo = Foo::new(1, 10);
    foo.print();

    let v = vec![2; 10];
    let foo_ref = FooWithReference::new(&v);
    foo_ref.print();
}

#[test]
fn test_foo_size() {
    let foo = Foo::new(1, 5);
    assert_eq!(foo.v.len(), 5);
}

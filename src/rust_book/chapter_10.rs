pub fn run() {
    let integer_point = Point { x: 4, y: 5 };
    let float_point = Point { x: 2.3, y: 3.4 };

    let float_integer_point = MixedPoint { x: 2.1, y: 5 };

    let numbers = vec![1, 2, 3, 4, 5, 6];
    println!("largest: {}", largest(&numbers));
    println!("largest_: {}", largest(&numbers));

    // Lifetimes
    let string1 = String::from("abcd");
    let string2;
    let result;
    {
        string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("the longer string is {}", result);

    // &i32 // a reference
    // &'a i32 // a reference with an explicit lifetime
    // &'a mut i32 // a mutable reference with an explicit lifetime
}

// We use slices because we don't want longest to take ownership of its parameters
// The first 'a is the generic lifetime, the rest means both x and y and the return value has the same lifetime 'a
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn largest_<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list.iter() {
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
    fn x(&self) -> &T {
        // Returns a reference to x
        &self.x
    }
}

impl Point<f32> {
    // This is only for type f32
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct MixedPoint<T, U> {
    x: T,
    y: U,
}

pub trait Summary {
    fn summarize(&self) -> String;
}

pub trait WithDefault {
    fn default_method(&self) -> String {
        // This is a default method
        String::from("(Read more...)")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

impl WithDefault for Tweet {} // Tweet uses the default implementation

pub fn notify_(item: impl Summary) {
    // This parameter accepts any type that implements Summary(the trait)
    println!("Breaking news! {}", item.summarize());
}

pub fn notify<T: Summary>(item: T) {
    // More verbose form of the above
    println!("Breaking news! {}", item.summarize());
}

pub fn double_bound_<T: Summary + WithDefault>(item: T) {} // T must implement both Summary and WithDefault

pub fn double_bound<T>(item: T) -> String
where
    T: Summary + WithDefault,
{
    String::from("")
}

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

    let novel = String::from("Call me Ihsmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExceprt {
        part: first_sentence,
    };
}

// Generic type parameters, trait bounds, and lifetimes all in one function!
use std::fmt::Display;
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display,
{
    println!("Announcement: {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// Structs doesn't hold owned type but a reference
// This annotation means an instance of ImportantExcerpt
// cant' outlive the reference it holds in its part field
struct ImportantExceprt<'a> {
    part: &'a str,
}

impl<'a> ImportantExceprt<'a> {
    // We're not required to annotate the lifetime of the reference to self because of the first elision rule
    fn level(&self) -> i32 {
        3
    }

    // There are two input lifetimes, Rust applies the first lifetime elision rule and gives
    // both &self and announcement their own lifetimes. Then, because one of the parameters is &self,
    // the return type gets the lifetime of &self, and all lifetimes have been accounted for.
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
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

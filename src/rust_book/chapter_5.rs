pub fn run() {
    let user1 = User {
        username: String::from("User1"),
        email: String::from("user1@email.com"),
        sign_in_count: 1,
        is_active: true,
    };

    // Create new user without using Struct Update Syntax
    let user2 = User {
        username: String::from("User2"),
        email: String::from("user2@email.com"),
        sign_in_count: user1.sign_in_count,
        is_active: user1.is_active,
    };

    // Create new user using Struct Update Syntax
    let user3 = User {
        username: String::from("User3"),
        email: String::from("user3@email.com"),
        ..user1
    };

    let rect = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect)
    );
    println!("The area of the rectangle is {} square pixels.", rect.area());

    println!("Rect: {:#?}", rect);
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    is_active: bool,
}

// These are called tuple structs
struct Color(i32, i32, i32);
struct Point(f32, f32, f32);

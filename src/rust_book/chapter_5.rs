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

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        area(&rect1)
    );
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!("Rect: {:#?}", rect1);

    println!("rect1 can hold rect2: {}", rect1.can_hold(&rect2));
    println!("rect1 can hold rect3: {}", rect1.can_hold(&rect3));

    let square = Rectangle::square(30);
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }

    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        (self.width > other.width && self.height > other.height)
            || (self.height > other.width && self.width > other.width)
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

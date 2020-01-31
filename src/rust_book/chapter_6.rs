pub fn run() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let home = IpAddrWAV::V4(String::from("127.0.0.1")); // Similar to above

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
    let loopback = IpAddrWAV::V6(String::from("::1")); // Similar to above

    let m = Message::Write(String::from("hello"));
    m.call();

    let some_number = Some(5);
    let some_string = Some("a string");

    let number = 5;
    let absent_number: Option<i32> = None;
    // let sum = number + absent_number; // This is a compile error

    value_in_cents(Coin::Quarter(UsState::Alabama));

    let u8_value = 0u8;
    match u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (), // Match any value and do nothing because () is unit value
    }

    // The 2 statements below are identical, second one is shorter and more common
    let some_u8_value = Some(0u8);
    match some_u8_value {
        Some(u) => Some(u + 1),
        _ => None,
    };

    if let Some(u) = some_u8_value {
        Some(u + 1);
    }
}

enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

// IP address with associated strings, similar to defining in a struct
enum IpAddrWAV {
    V4(String),
    V6(String),
}

// Another variant which can not be achieved with a struct
enum IpAddr2 {
    V4(u8, u8, u8, u8),
    V6(String),
}

// Another enum example
enum Message {
    Quit,                       // Has no associated data
    Move { x: u32, y: u32 },    // Includes an anonymous struct
    Write(String),              // Includes a single String
    ChangeColor(i32, i32, i32), // Includes three i32 values
}

impl Message {
    fn call(&self) {}
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            match state {
                UsState::Alabama => println!("Alabama"),
                UsState::Alaska => println!("Alaska"),
            }
            25
        }
    }
}

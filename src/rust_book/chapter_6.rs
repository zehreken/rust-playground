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

pub fn run() {
    // Very basic example for scope
    {
        let s = 3;
    }
    // println!("{}", s); // Throws error
    let mut s = String::from("hello");
    s.push_str(", world");
    println!("{}", s);

    let i1 = 5;
    let i2 = i1; // No move happens, just copy because i1 and i2 are fixed size variables
    println!("{}", i1);

    let s1 = String::from("move me");
    let s2 = s1; // Move happens
                 // println!("{}", s1); // Gives error 'value used here after move'

    let s3 = String::from("copy me");
    let s4 = s3.clone();
    println!("{}, {}", s3, s4);

    println!("lenght: {}, {}", calculate_length(&s3), s3);

    // Mutable references
    let mut s = String::from("hello");
    let r0 = &s;
    let r1 = &s;
    println!("{}, {}", r0, r1); // Since r0, r1 are used here, no problem borrowing s as mutable in the next line
    let r2 = &mut s;
    println!("{}", r2);

    let s = String::from("hello"); // s comes into scope

    takes_ownership(s); // s's value moves into the function...
                        // ... and so is no longer valid here

    let x = 5; // x comes into scope

    makes_copy(x); // x would move into the function,
                   // but i32 is Copy, so it’s okay to still
                   // use x afterward
} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn gives_ownership() -> String {
    // gives_ownership will move its
    // return value into the function
    // that calls it

    let some_string = String::from("hello"); // some_string comes into scope

    some_string // some_string is returned and
                // moves out to the calling
                // function
}

// takes_and_gives_back will take a String and return one
fn takes_and_gives_back(a_string: String) -> String {
    // a_string comes into
    // scope

    a_string // a_string is returned and moves out to the calling function
}

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{}", some_integer);
}

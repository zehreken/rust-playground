pub fn start_ownership() {
    let l = "literal"; // A string literal is a slice, hence a &str, immutable
    let s = String::from("hello world");

    let hello = &s[0..5]; // String slice
    let world = &s[6..11]; // It is reference to a part of a String
                           // Also &[0..n] is equal to &[..n], the zero can be dropped
                           // and &[n..len] is equal to &[n..], the last index can be dropped
                           // and &[0..len] is equal to &[..], if you get the entire slice
    println!("{}, {}", hello, world);

    let word = first_word(&s); // Here s is borrowed
    println!("{}", word);

    takes_ownership(s);

    let s = gives_ownership(); // s can be redefined because previous one is dropped at this point
    takes_ownership(s);

    let mut s = gives_ownership();
    s = takes_and_gives_ownership(s);

    println!("{}", calculate_length(&s));

    change_ref(&mut s);
    println!("{}", s);

    let x = 5;
    makes_copy(x);
    makes_copy(x);
}

// fn first_word(s: &String) -> &str { A better way of doing this is
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String {
    let s = String::from("hello");

    return s;
}

fn takes_and_gives_ownership(some_string: String) -> String {
    println!("{}", some_string);

    return some_string;
}

fn calculate_length(s: &String) -> usize {
    return s.len();
}

fn change_ref(s: &mut String) {
    s.push_str("_new");
}

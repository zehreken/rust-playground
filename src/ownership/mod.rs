pub fn start_ownership() {
    let s = String::from("hello");

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

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn gives_ownership() -> String
{
    let s = String::from("hello");

    return s;
}

fn takes_and_gives_ownership(some_string:  String) -> String {
    println!("{}", some_string);

    return some_string;
}

fn calculate_length(s: &String) -> usize {
    return s.len();
}

fn change_ref(s: &mut String) {
    s.push_str("_new");
}

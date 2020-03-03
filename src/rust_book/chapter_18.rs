pub fn run() {
    run_if_let();

    while_let();

    for_loop();

    match_literals();

    match_named();
}

fn run_if_let() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
}

fn while_let() {
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}

fn for_loop() {
    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        // (index, value) tuple is produced by enumerate()
        println!("{} is at index {}", value, index);
    }
}

fn match_literals() {
    let x = 1;

    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    match x {
        1 | 2 => println!("one or two"), // Multiple pattern
        3 => println!("three"),
        _ => println!("anything"),
    }
}

fn match_named() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {:?}", y), // this is a new 'y'
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {:?}", x, y);
}

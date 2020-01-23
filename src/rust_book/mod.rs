use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub fn start_rust_book() {
    chapter_4();
    // chapter_3();
    // chapter_2();
}

fn chapter_4() {
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

    let s = String::from("hello"); // s comes into scope

    takes_ownership(s); // s's value moves into the function...
                        // ... and so is no longer valid here

    let x = 5; // x comes into scope

    makes_copy(x); // x would move into the function,
                   // but i32 is Copy, so it’s okay to still
                   // use x afterward
} // Here, x goes out of scope, then s. But because s's value was moved, nothing
  // special happens.

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
// CHAPTER 4

fn chapter_3() {
    println!("CHAPTER 3");
    let x = 5;
    let x = x + 1; // Shadowing
    let x = x * 2;
    println!("The value of x: {}", x);

    // let spaces = "    ";
    // let spaces = spaces.len(); // Valid
    // let mut spaces = "    ";
    // spaces = spaces.len(); // Invalid

    let tuple: (i32, f64, u8) = (-1, 100_000.0, 255);
    let (x, y, z) = tuple; // This is called destructuring
    println!("x: {}, y: {}, z: {}", x, y, z);

    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    let arr = [3; 5]; // Same as let arr = [3, 3, 3, 3, 3];

    // A valid if statement example
    let condition = true;
    let number = if condition { 5 } else { 6 };

    // This is how you return a value from a loop
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {}", result);

    // This prints 3, 2, 1
    for number in (1..4).rev() {
        println!("Number in arr: {}", number);
    }
}

fn chapter_2() {
    println!("CHAPTER 2");
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(0, 101);
    println!("The secret number is: {}", secret_number);

    loop {
        println!("Please input your guess");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // With proper error handling, sort of
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Not a number");
                continue;
            }
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

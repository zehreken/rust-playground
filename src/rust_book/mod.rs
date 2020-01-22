use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub fn start_rust_book() {
    chapter_3();
    // chapter_2();
}

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

pub fn run() {
    let v: Vec<i32> = Vec::new();
    let mut v = vec![1, 2, 3]; // vec! macro for convenience

    v.push(4);
    v.push(5);

    let third = &v[2];
    let third = v.get(2); // Returns Options<&T>
                          // let hundredth = &v[99]; // This panics
    let hundredth = v.get(99); // Does not panic but returns None

    for i in &mut v {
        *i += 10; // Dereference first
        println!("{}", i);
    }

    // A vector that holds different types of data
    let row = vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Float(5.3),
        SpreadSheetCell::Text(String::from("text")),
    ];

    for r in &row {
        match r {
            SpreadSheetCell::Int(i) => println!("{:?}", i),
            _ => (),
        }
    }

    strings();
    hash_maps();
} // v goes out of scope and is freed here

fn strings() {
    let s = String::from("initial content");
    let mut s = "initial content".to_string();
    s.push_str(" more content");
    s.push('!'); // push method accepts a single character

    println!("{}", s);

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // s1 is moved here, so it can no longer be used

    let tic = String::from("tic");
    let tac = String::from("tac");
    let toe = String::from("toe");
    // let s = tic + "-" + &tac + "-" + &toe;
    let s = format!("{}-{}-{}", tic, tac, toe); // format does not take ownership of tic as opposed to the example above

    for c in s.chars() {
        println!("c: {}", c);
    }
    for b in s.bytes() {
        println!("b: {}", b);
    }

    let turkish = String::from("çğıöşü");
    for c in turkish.chars() {
        println!("c: {}", c);
    }
    for b in turkish.bytes() {
        println!("b: {}", b);
    }
}

use std::collections::HashMap;
fn hash_maps() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    // Another way to construct the same hash map
    let teams = vec![String::from("Blue"), String::from("Yellow")]; 
    let initial_scores = vec![10, 50];
    let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value); // ownership of field_name and field_value are moved to map, they are invalid from this point

    let team_name = String::from("Blue");
    println!("{} score: {}", team_name, scores.get(&team_name).unwrap()); // get returns Option<&V>

    for (key, value) in scores {
        println!("{}, {}", key, value);
    }

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 25); // This overwrites 10
    // entry method checks if a key has a value or not
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50); // Value for Blue does not change
    for (key, value) in scores {
        println!("{}, {}", key, value);
    }

    // Updating a value based on old value
    let text = "hello world wonderful world";
    let mut map= HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0); // or_insert returns a mutable reference(&mut V) to the value
        *count += 1;
    }

    for (key, value) in map {
        println!("{}, {}", key, value);
    }
}

#[derive(Debug)]
enum SpreadSheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

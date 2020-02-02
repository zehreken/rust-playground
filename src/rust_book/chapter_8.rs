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

#[derive(Debug)]
enum SpreadSheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

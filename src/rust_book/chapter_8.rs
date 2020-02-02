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
            SpreadSheetCell::Int(i) => {println!("{:?}", i)},
            _ => (),
        }
    }
} // v goes out of scope and is freed here

#[derive(Debug)]
enum SpreadSheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}
pub fn run() {
    let mut v: Vec<i32> = vec![0, 1, 2, 3, 4, 5];

    v = print_vector(v);

    v[0] = -1;

    print_vector_ref(&v);

    let reference_to_int = &4;

    match reference_to_int {
        &val => println!("Got a value via destructuring: {:?}", val),
    }

    print_int_ref(reference_to_int);

    let int: i32 = 3;
    let ref reference_to_int = 3; // This is a reference just because of the 'ref'

    print_int(int);
    print_int_ref(reference_to_int);
}

// Vector 'v' is moved and then return back
fn print_vector(v: Vec<i32>) -> Vec<i32> {
    println!("{:?}", v);

    v
}

fn print_vector_ref(v: &Vec<i32>) {
    println!("{:?}", v);
}

fn print_int(i: i32) {
    println!("{}", i);
}

fn print_int_ref(i: &i32) {
    println!("{}", i);
}

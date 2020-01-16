pub fn start_memory() {
    let mut a: Vec<i32> = vec![0, 1, 2, 3, 4, 5];

    a = print_vector(a);

    a[0] = -1;

    print_vector_ref(&a);

    let reference = &4;
    match reference {
        &val => println!("Got a value via destructuring: {:?}", val)
    }
    print_int_ref(reference);

    let not_a_reference = 3; // This is not a reference because the right side is not a reference
    let ref is_a_reference = 3; // This is a reference because of the 'ref' keyword, this is what ref is for

    print_int(not_a_reference);
    print_int_ref(is_a_reference);
}

// A is moved but the function returns the vector back
fn print_vector(a: Vec<i32>) -> Vec<i32> {
    println!("{:?}", a);

    a
}

fn print_vector_ref(a: &Vec<i32>) {
    println!("{:?}", a);
}

fn print_int(a: i32) {
    println!("{}", a);
}

fn print_int_ref(a: &i32) {
    println!("{}", a);
}

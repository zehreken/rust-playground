pub fn run() {
    unsafe_rust();
}

fn unsafe_rust() {
    let mut num = 5;

    let r1 = &num as *const i32; // By using as *const to cast an immutable reference
    let r2 = &mut num as *mut i32; // By using as *mut to cast a mutable reference

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    let address = 0x012345usize;
    let r = address as *const i32;
    unsafe {
        // println!("{:?}", *r); // Segmentation fault
    }

    unsafe {
        dangerous();
    }

    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    add_to_count(3);
    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}

unsafe fn dangerous() {}

extern "C" {
    fn abs(input: i32) -> i32;
}

static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

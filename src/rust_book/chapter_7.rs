// This chapter is about packages, crates, modules, namespaces
// libraries and the general structure of the project
pub fn run() {
    crate::rust_book::chapter_7::front_of_house::hosting::add_to_waitlist(); // Absolute path

    front_of_house::hosting::add_to_waitlist(); // Relative path
}

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

mod back_of_house {
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peach"),
            }
        }
    }

    fn fix_incorrect_order() {
        cook_order();
        super::serve_order(); // super refers to the parent module
    }

    fn cook_order() {}
}

fn serve_order() {}

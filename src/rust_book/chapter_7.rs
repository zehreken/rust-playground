// This chapter is about packages, crates, modules, namespaces
// libraries and the general structure of the project
pub fn run() {}

mod front_of_house {
    mod hosting {
        fn add_to_waitlist() {}
        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

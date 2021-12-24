#![allow(unused_variables)]

fn main() {
    /*
     * Crate: a library or binary
     * Crate root: the source file that rustc will start making the crate from, is also the root
     * module
     * Package: one or more crates that provide some funtionality. can have any number of binaries
     * but only at most one library
     *
     * This file is a binary crate for example
     */

    modules::call_greet();
    // this is possible because of the re-export
    modules::seat_at_table();

    let dishwasher = modules::back_of_house::dishwash::DishwasherType::HYUNDAI;
    let manager = modules::back_of_house::management::Manager {
        name: String::from("yooo"),
        age: 100,
        title: String::from("annoying person"),
    };
}

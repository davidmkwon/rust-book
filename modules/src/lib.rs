/*
* Modules can kind of be thought of as namespaces? Basically they group together some related
* functionality
*
* Because this is the crate root (it's lib.rs), everything in ths file is part of a module called
* `crate`. The structure of the current modules:

crate
└── front_of_house
    ├── hosting
    │   ├── add_to_waitlist
    │   └── seat_at_table
    └── serving
        ├── take_order
        ├── serve_order
        └── take_payment

* just like file systems, we can refer to modules by their absolute or relative path
*/
#![allow(dead_code)]
#![allow(unused_imports)]

//! the `//!` style comments are used to comment the item that contains these comments, in this
//! case the crate root.

// why doesn't this mod need to be pub? because it's a "sibling" of where the eat_at_restaurant
// function is, so we have access to it (both's parents is the crate mod).
mod front_of_house {
    // for structs, you need to say which fields are publicly accessible or not. so here user's
    // cannot access `server_name`
    pub struct Menu {
        pub is_dinner: bool,
        server_name: String,
    }

    // for enums, declaring it pub makes all the variants pub
    pub enum Drinks {
        WATER,
        SODA,
        WINE,
    }

    pub fn greet() {
        println!("hiya!");
    }

    // need to specify public!
    pub mod hosting {
        pub fn add_to_waitlist() {}
        pub fn seat_at_table() {
            // can access parent mod via `super`
            super::greet();
        }
    }

    // this moule is private
    mod serving {
        fn take_order() {}
        fn serve_order() {}
    }
}

// this should as an example in conjunction with `proc-macros` package
pub trait HelloMacro {
    fn hello_macro();
}

/// This is called a "documentation comment", with three slashes. It supports markdown and
/// is used to create HTML docs for your library.
///
/// Running `cargo doc` will create the HTML doc pages. Running `cargo doc --open` will open these
/// HTML docs in your browser. Running `cargo test` will run the examples in your docs!
///
/// # Examples
///
/// ```
/// use modules;
///
/// let some_rust_code = modules::eat_at_restaurant();
/// assert_eq!(some_rust_code, ());
/// ```
pub fn eat_at_restaurant() {
    // absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // relative path
    front_of_house::hosting::seat_at_table();
}

// this brings Drinks into current scope
use self::front_of_house::Drinks;
pub fn new_drink() -> Drinks {
    Drinks::WATER
}

// `use` idioms:
// - for functions, use up to the parent module
// - for structs/enums, use up to the struct/enum itself
use self::front_of_house::hosting;
pub fn call_greet() {
    hosting::seat_at_table();
}

// this is called "re-exporting" modules because code that uses this module will also get the use
// imports
pub use self::front_of_house::hosting::seat_at_table;

// avoid repetitive base paths
use std::io::{self, Read, Write};
// instead of:
// use std::io;
// use std::io::Read;
// use std::io::Write;

// declaring a module like this will look for a file or directory with the same name as this and
// declare it here. i think it's basically c-style includes the module here...
pub mod back_of_house;

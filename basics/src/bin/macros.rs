// https://doc.rust-lang.org/book/ch19-06-macros.html

#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]

use hello_macro::HelloMacro;
use hello_macro_derive::HelloMacro;

/*
 * Macros
 *
 * two types of macros:
 * 1. declarative macros with macro_rules!
 * 2. procedural macros:
 *    - custom #[derive] macros
 *    - attribute-like macros that define custom attributes for any item
 *    - function-like macros
 */

// declarative macros/macro_rules!
//
// kind of like match expression but for writing Rust code, ie matches against structure for
// Rust code and when matched replaces the code with other code
//
// define our own simplified vec macro:
//
// #[macro_export] indicates that this macro be brought into scope when the crate it is defined in
// is brought into scope
//
// a brief explanation:
// each () is a match arm. the $($x:expr) matches any expression to x. the * after this means that
// this pattern is matched 0 or more times. the $()* block indicates what the code that is matched
// will be replaced with
#[macro_export]
macro_rules! vecc {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

// procedural macros--see "../hello_macro"
#[derive(HelloMacro)]
struct Person;

fn main() {
    let v: Vec<i32> = vecc![1, 2, 3];
}

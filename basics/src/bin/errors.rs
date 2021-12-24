// https://doc.rust-lang.org/book/ch09-00-error-handling.html

#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]

use std::fs::File;
use std::io::{self, ErrorKind, Read};

fn main() {
    /*
     * Panics
     *
     * panics are basically when the program halts executing and will unwind the stack (unless you
     * configure rustc to "abort" instead, in which case you just exit and let OS clean the
     * memory). the output will show you on which line in the source code the panic macro was
     * called, so it won't always show you the line in which you're source code caused the panic,
     * in which case you should use the backtrace env var
     */
    let v = vec![1, 2, 3];
    let x = v[99];

    /*
     * Results
     *
     * when you don't want to necessarily stop execution when something fails
     */
    // opens a file and creates one if it doesn't exist, else panics
    let f = File::open("hello.txt");
    let f = match f {
        Ok(file) => file,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(file) => file,
                Err(e) => panic!("{:?}", e),
            },
            other => {
                panic!("{:?}", other);
            }
        },
    };

    // less verbose:
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("{:?}", error);
            })
        } else {
            panic!("{:?}", error);
        }
    });

    // easier ways to deal with results
    // 1. unwrap()--just panic with error message if Err
    let f = File::open("hello.txt").unwrap();
    // 1. expect()--panic with custom error message
    let f = File::open("hello.txt").expect("couldn't open hello.txt");

    // propagating errors
    // 1. kind of verbose way, but works
    fn read_string_from_file(file_name: &str) -> Result<String, io::Error> {
        let f = File::open(file_name);
        let mut f = match f {
            Ok(f) => f,
            Err(e) => return Err(e),
        };

        let mut s = String::new();
        match f.read_to_string(&mut s) {
            Ok(_) => Ok(s),
            Err(e) => Err(e),
        }
    }
    // 2. use ? operator
    //
    // basically ? will return val if a Result is Ok(val), and will return from the function Err(e)
    // if a result is Err(e).
    //
    // note: (will literally return from the function) the Err(e) variant if it's Err.
    fn read_string_from_file2(file_name: &str) -> Result<String, io::Error> {
        let mut f = File::open(file_name)?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;
        Ok(s)
    }
    // 2b. even shorter version
    fn read_string_from_file3(file_name: &str) -> Result<String, io::Error> {
        let mut s = String::new();
        File::open("hello.txt")?.read_to_string(&mut s)?;
        Ok(s)
    }

    /*
     * When to panic
     *
     * I think generally use Result because gives user ability to panic as chooses, but panic when
     * it's dangerous (array oob) or user has violated a function's argument contract.
     */
    // pretty cool way to enforce invariants on the type-level. Guess objects can only be created
    // via new() (because value isn't pub), and new() panics if the invariant (in this case btwn 1
    // and 100) is broken. so functions can now operate on Guess's and not need to check the
    // invariant anymore if a GUess object exists.
    pub struct Guess {
        value: i32,
    }

    impl Guess {
        pub fn new(value: i32) -> Guess {
            if value < 1 || value > 100 {
                panic!("Guess value must be between 1 and 100, got {}.", value);
            }

            Guess { value }
        }

        pub fn value(&self) -> i32 {
            self.value
        }
    }
}

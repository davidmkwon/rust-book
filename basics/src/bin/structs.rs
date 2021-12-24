// https://doc.rust-lang.org/book/ch05-00-structs.html

#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]

fn main() {
    // define structs
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    // you can use shorthand struct instantiation when parameters have same name as struct fields
    fn build_user(email: String, username: String) -> User {
        User {
            email,
            username,
            sign_in_count: 1,
            active: true,
        }
    }

    // note that fields don't need to be in the declared order
    let bob = User {
        active: false,
        email: String::from("bob@hotmail.com"),
        sign_in_count: 5,
        username: String::from("bobob1"),
    };

    // use '..' to copy rest of struct fields
    // note that the fields are copied using `=`, so basically move semantics apply. so now
    // `bob_copycat` has ownership over bob.email (the only non-`impl Copy` field that was copied)
    // so you can't use bob anymore
    let bob_copycat = User {
        email: String::from("bobcopy@gmail.com"),
        ..bob
    };

    // tuple structs are basically just named tuples types
    #[derive(Debug)]
    struct Color(i32, i32, i32);

    let mut black = Color(0, 0, 0);
    let r_black = black.0;
    // destructuring patterns:
    // https://users.rust-lang.org/t/how-to-destructure-a-tuple-struct/45296/3
    let Color(r, g, b) = black;
    dbg!(&black);

    // "Unit-like" structs
    struct Person;
    let p = Person;

    // methods are functions specific to structs. the first argument of them has to be of type
    // `Self`, but whether it's owned, mutable, or immutable is up to you
    impl Color {
        fn print(&self) {
            dbg!(self);
        }

        fn set_r(&mut self, r: i32) {
            self.0 = r;
        }

        // this is an associated function: this isn't a method since it doesn't have self as a
        // parameter
        fn new() -> Self {
            Color(0, 0, 0)
        }
    }
    black.print();
    black.set_r(1);
    let black2 = Color::new();
}

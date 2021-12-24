// https://doc.rust-lang.org/book/ch19-00-advanced-features.html

#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]

use std::{ops::Add, slice};

// unsafe (cont.):
//
// global/static variables: variables that can be accessed anywhere
//
// what's the diff between static and const variables?
// 1. static vars can be mutable
// 2. static vars have fixed addr in memory. const vars can duplicate their data when used, but
//    static vars will always refer to same data.
//
// immut
static HELLO_WORLD: &str = "Hello, world!";
// mut
static mut COUNTER: u32 = 0;
fn add_to_count(inc: u32) {
    // reading mutable static vars is unsafe
    unsafe {
        println!("{}", COUNTER);
    }

    // modifying mutable static vars is unsafe
    unsafe {
        COUNTER += inc;
    }
}

// unsafe (cont.):
//
// unsafe traits: traits whose methods have some unsafe invariant that can't be checked by
// compiler. implementing these traits is thus unsafe
unsafe trait Foo {
    fn hi();
}
unsafe impl Foo for i32 {
    fn hi() {
        println!("hi");
    }
}

fn main() {
    /*
     * Unsafe Rust
     *
     * `unsafe` lets you do:
     * 1. dereference a raw pointer
     * 2. call an unsafe function/method
     * 3. access or modify a mutable static variable
     * 4. implement an unsafe trait
     * 5. access fields of `union`s
     */

    // raw pointers: `*const T` and `*mut T`
    //
    // raw pointers:
    // 1. ignore borrowing rules (multiple immut and mut pointers)
    // 2. don't necessarily point to valid memory/can be null
    // 3. don't implement automatic cleanup

    // create (valid) raw pointer:
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    // only need unsafe to *dereference* raw pointers
    unsafe {
        println!("r1 is {}", *r1);
        println!("r2 is {}", *r2);
    }

    // raw pointer to random addr
    let address = 0x012345usize;
    let r = address as *const i32;

    // unsafe function: what it sounds like
    unsafe fn read_ptr(ptr: *const i32) -> i32 {
        *ptr
    }
    // call unsafe fn's in unsafe blocks
    unsafe {
        let val = read_ptr(r1);
    }

    // functions that use unsafe don't need to be marked as unsafe--in fact safe fn's often use
    // unsafe.
    //
    // example: split_at_mut()
    fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        let len = slice.len();
        let ptr = slice.as_mut_ptr();

        assert!(mid <= len);

        unsafe {
            (
                slice::from_raw_parts_mut(ptr, mid),
                slice::from_raw_parts_mut(ptr.add(mid), len - mid),
            )
        }
    }
    // safe functions can technically still have memory issues: the API writer is the one saying
    // that the function is safe, it is not necessarily provably so. for example this code would
    // have undefined behavior if it wasn't for the assert! statement we wrote above:
    //
    // let mut arr = [1, 2, 3];
    // let v = &mut arr;
    // let (x, y) = split_at_mut(v, 4);
    // println!("x: {:?}, y: {:?}", x, y);

    // you can call foreign code via `extern`, which creates/uses a FFI-Foreign Function Interface.
    // An FFI is a way for code written in one language to be called from another
    extern "C" {
        fn abs(input: i32) -> i32;
    }
    unsafe {
        println!("{}", abs(-3));
    }

    /*
     * Advaned Traits
     */
    // associated types in traits are diff than generics because it forces only one concrete type
    trait Iterator2 {
        type Item;
        fn next(&mut self) -> Option<Self::Item>;
    }
    // rather than this, where one type could impl Iterator3<i32>, Iterator3<String>, etc., which
    // would be confusing
    trait Iterator3<T> {
        fn next(&mut self) -> Option<T>;
    }

    // default type paramters: you can give a default type for a generic type paramter using
    // <PlaceholderType = ConcreteType>.
    //
    // ie the Add trait has the following definition:
    //
    // trait Add<Rhs = Self> {
    //     type Output;
    //     fn add(self, rhs: Rhs) -> Self::Output;
    // }
    //
    // so if you don't specify Rhs it just defaults to the type your're impling on.
    struct Person {
        name: String,
    }
    impl Add for Person {
        type Output = Person;

        fn add(self, rhs: Self) -> Self::Output {
            Person {
                name: self.name + &rhs.name,
            }
        }
    }

    // diff traits with the same method names:
    struct Me {
        name: String,
    }
    trait Hello {
        fn greet(&self);
    }
    trait Hi {
        fn greet(&self);
    }
    impl Hello for Me {
        fn greet(&self) {
            println!("hello, {}", self.name);
        }
    }
    impl Hi for Me {
        fn greet(&self) {
            println!("hi, {}", self.name);
        }
    }
    impl Me {
        fn greet(&self) {
            println!("me: {}", self.name);
        }
    }
    // to call greet defined in Me:
    let m = Me {
        name: "David".to_string(),
    };
    m.greet();
    // to call greet defined for the traits
    Hello::greet(&m);
    Hi::greet(&m);

    // diff traits with same name but don't take &self--use "fully qualified syntax"
    struct Me2 {
        name: String,
    }
    trait Hello2 {
        fn greet();
    }
    trait Hi2 {
        fn greet();
    }
    impl Hello2 for Me2 {
        fn greet() {
            println!("hello");
        }
    }
    impl Hi2 for Me2 {
        fn greet() {
            println!("hi");
        }
    }
    impl Me2 {
        fn greet() {
            println!("me");
        }
    }
    // to call greet defined in Me:
    let m = Me2 {
        name: "David".to_string(),
    };
    Me2::greet();
    // to call greet defined for the traits
    <Me2 as Hello2>::greet();
    <Me2 as Hi2>::greet();

    // super traits: a trait whose methods/types are depended on by another trait
    trait Greet {
        fn greet(&self) -> String;
    }
    // GreetPerson depends on the greet() method from Greet
    trait GreetPerson: Greet {
        fn greet_person(&self, name: &str) -> String {
            self.greet() + name
        }
    }

    /*
     * Advanced Types
     */
    // Type alises: just an drop-in name replacement for the type on the right. commonly used for
    // when you have a really long type that you have to keep repeating. also used for having a
    // consistent error types in Results in std--for example std::io aliases `type Result<T>` to
    // std::result::Result<T, std::io::Error> so that all Result<T>'s have std::io::Error as the
    // error type
    type Kilometers = i32;
    type ThreeTup<X, Y, Z> = (X, Y, Z);
    type Thunk = Box<dyn Fn() + Send + 'static>;

    // `!` is the never type. it's used to describe a value that never returns:
    //
    // ie
    // - `continue` is of type !
    // - `panic!` is of type !
    //
    // `!` values can be coerced into any other type in terms of type checking, so match statements
    // that are expected to return a value, but one arm returns `!` is okay:
    //
    // match self {
    //   Some(val) => val,
    //   None => panic!(""),
    // }

    // dynamically sized types: types whose size we only may know at runtime
    //
    // i.e `str` is a DST--we can only know at runtime how long the string is, which is why we put
    // it behind `&str` or `Box<str>`
    //
    // i.e any trait is a DST--we can only know at runtime the underlying type that implements it,
    // so we refer to trait objects through `&dyn Trait`, `Box<dyn Trait>`, etc.
    //
    // DSTs must always be put behind some kind of pointer so that we know at compile time how big
    // the reference to the DST is.
    //
    // The `Sized` trait tells whether a type's size is known at compile time. this trait is
    // automatically implemented for any type who's size is known at compile time. the compiler
    // implicitly marks generic type parameters as Sized:
    //
    // fn generic<T>(t: T);
    // .. ->
    // fn generic<T: Sized>(t: T);
    //
    // to get by this automatic restriction, you can explicitly mark the type paramter as ?Sized:
    //
    // fn generic<T: ?Sized>(t: &T);

    // Function pointers
    //
    // `fn` type is a function pointer. diff from Fn, which is for closures. moreover, fn is a
    // type, whereas Fn is a trait.
    //
    // the biggest diff obv is that a function is a function, in that it can only operate on
    // arguments that are passed in, unlike closures which can capture from the env. thus a
    // function implements Fn, FnMut, and FnOnce.
    fn add_one(x: i32) -> i32 {
        x + 1
    }
    fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
        f(arg) + f(arg)
    }
}

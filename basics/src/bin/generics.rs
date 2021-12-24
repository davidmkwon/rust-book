//https://doc.rust-lang.org/book/ch10-00-generics.html

#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]

/*
 * GENERICS
 */

// generic structs
struct Point<T> {
    x: T,
    y: T,
}
// add generic methods for it
//
// we need the <T> after `impl` to denote that we are adding methods for Point<T>, and that the T
// is a generic type here and not a concrete type. this is because you can add methods for a
// concrete type in Point<>, as shown below
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}
// add method for Point<i32>
impl Point<i32> {
    fn sum(&self) -> i32 {
        self.x + self.y
    }
}
// can have methods that have their own generics
impl<T> Point<T> {
    fn change_type<U>(p_u: Point<U>) -> Point<U> {
        Point { x: p_u.x, y: p_u.y }
    }
}

/*
 * TRAITS
 *
 * a way to specify behavior types have
 */
trait Summary {
    // any type that implements Summary trait must have a summarize function with this signature
    fn summarize(&self) -> String;
}
// implement trait
impl Summary for Point<i32> {
    fn summarize(&self) -> String {
        format!("{}, {}", self.x, self.y)
    }
}
// a note about traits:
//
// "one restriction to note with trait implementations is that we can implement a trait on a type
// only if either the trait or the type is local to our crate"

// you can have default impl for trait functions
trait Elongate {
    // default impl can call non-default funcs
    fn elongate(&self) -> String {
        String::from("elongated..") + &self.elongate_source()
    }

    fn elongate_source(&self) -> String;
}

// these are equivalent
fn call_elongate(e: impl Elongate) {
    println!("called elongate: {}", e.elongate());
}
// "trait bound" syntax
fn call_elongate2<T: Elongate>(e: T) {
    println!("called elongate: {}", e.elongate());
}
// multiple impls
fn call_summary_elongate<T: Summary + Elongate>(item: T) {
    println!(
        "called sum and elong: {}, {}",
        item.summarize(),
        item.elongate()
    );
}
// can also do this with "where" clause--looks less cluttered and easier to understand
fn call_summary_elongate2<T>(item: T)
where
    T: Summary + Elongate,
{
    println!(
        "called sum and elong: {}, {}",
        item.summarize(),
        item.elongate()
    );
}

// largest ele in i32 slice
fn largest(list: &[i32]) -> i32 {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// make above generic
//
// note that we declare T, the name of the generic type parameter, before any useage of it.
fn largest2<T>(list: &[T]) -> T
where
    T: PartialOrd + Copy,
{
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// solution around Copy, return ref instead
fn largest3<T>(list: &[T]) -> &T
where
    T: PartialOrd,
{
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// "blanket implementations" are when you implement a trait for types that implement a different
// trait:
//
// note here that any type that is impl Elongate is also impl Eelongate
trait Eelongate {
    fn eelongate(&self) -> String;
}
impl<T: Elongate> Eelongate for T {
    fn eelongate(&self) -> String {
        self.elongate()
    }
}

fn main() {
    /*
    * Lifetimes
    *
    * all references have lifetimes. sometimes you need to specify them.
    *
    * the borrow checker compares scopes to check whether references/borrows are valid
    *
    *
    * for example here it is clear that r outlives x, so an error occurs
    {
        let r;                // ---------+-- 'a
                              //          |
        {                     //          |
            let x = 5;        // -+-- 'b  |
            r = &x;           //  |       |
        }                     // -+       |
                              //          |
        println!("r: {}", r); //          |
    }
    */

    // the longest of two strings
    //
    // the lifetime paramter `'a` specifies that both x, y, and the returned string slice have the
    // same lifetime ('a)*. this allows the compiler to check that this condition is true. note that
    // the lifetime paramter doesn't change the lifetimes of x or y--it just gives a specification
    // that the compiler can check against to make sure it's true
    //
    // the value of 'a is set to the shorter lifetime between x and y when this function is called.
    //
    // *practically it means that the returned slice lives as long as the lesser of x and y
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }

    // example 1: string2 lives less than string1, but because we don't use result longer than
    // string2 lives, we're fine
    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }

    // example 2: this doesn't compile because even though result will have string1's value,
    // result's lifetime is bounded by string2
    //
    // let string1 = String::from("long string is long");
    // let result;
    // {
    //     let string2 = String::from("xyz");
    //     result = longest(string1.as_str(), string2.as_str());
    // }
    // println!("The longest string is {}", result);

    // Structs can have lifetime paramters if they hold values with lifetimes
    struct ImportantExcerpt<'a> {
        s: &'a str,
    }
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt { s: first_sentence };
    // ^i doesn't outlive first_sentence, so we chilling

    // note that this code compiles, even though it should have lifetimes marked.
    //
    // this is because of historical reasons, it was repetitive to write 'a each time
    fn first_word(s: &str) -> &str {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }

        &s[..]
    }
    fn first_word_long<'a>(s: &'a str) -> &'a str {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }

        &s[..]
    }

    // lifetime elision is what happens above, refers to how rustc will check some rules to see if
    // it can infer the lifetimes on its own.
    //
    // lifetime elision rules are specified here:
    // https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#lifetime-elision

    // the static lifetime is for values that life for the entire duration of the program, aka
    // string literals since they are in the binary

    // IN SUMMARY: "lifetime parameters specify which argument lifetime is connected to the lifetime
    // of the return value".
}

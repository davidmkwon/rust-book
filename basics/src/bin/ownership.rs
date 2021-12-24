// https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html

#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]

fn main() {
    /*
     * OWNERSHIP
     *
     * okay i think ownership in a nutshell is just RAII in C++. every value in Rust has an owner,
     * there can only be one owner at a time. when owner goes out of scope, value is dropped/freed.
     * assigning owned value to diff variable "moves" ownership of that value
     */

    // this creates lightweight copies, can still access x after
    let x = 5;
    let y = x;
    println!("x: {}, y: {}", x, y);

    // x here however is a heap-allocated value, and thus assigning it to y actually moves it
    //
    // this isn't just a "shallow copy", which would imply that y just copies the pointer and size
    // that make up a string rather than the underlying heap data. this also *invalidates* x, so
    // you can't use it after since it's been moved.
    //
    // because only y is "valid" now, only y's value will be freed when it goes out of scope and
    // not x's value
    let x = String::from("hiya");
    let y = x;

    // for integers you can still access after what should be a move because they implement trait
    // `Copy`, which basically means it's a lightweight stack copy. a type can't be `Copy` and
    // `Drop` because then that means it has some internal data that is not trivial to copy and
    // requires being dropped.

    // functions parameters have same semantics
    let x = String::from("hiya");
    fn take_ownership(x: String) {}
    take_ownership(x);
    // can't use x

    // can return ownership through ret val
    let mut x = String::from("hiya");
    fn take_ownership_and_return(x: String) -> String {
        x
    }
    x = take_ownership_and_return(x);

    /*
     * REFERENCES AND BORROWING
     */
    let x1 = &mut x;
    x1.push_str("hii");

    // okay whole point of references is that you can *either* have one mutable reference or any
    // number of immutable references at a time.

    /*
     * THE SLICE TYPE
     */
    let static_str: &str = "this is a static string";
    let string = String::from("hello");

    // so basically slices are just a immutable reference to some contiguous piece of data. so for
    // strings it's just &ref to the underlying bytes, and for static strings (strings that are
    // stored in the rust binary itself) it's the same.
    let slice = &static_str[1..5];
    let slice = &string[..3];
    println!("{}", slice);

    // other slices:
    let x = [1, 2, 3, 4];
    let slice: &[i32] = &x[..];
}

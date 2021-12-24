// https://doc.rust-lang.org/book/ch03-00-common-programming-concepts.html

#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]

// global scope
const DAYS_IN_YEAR: u16 = 356;

fn main() {
    /*
     * VARIABLES AND MUTABILITY
     */

    // writing 'mut' allows you to alter variables
    let mut x = 5;
    x = 6;

    // this won't work
    // let x = 5;
    // x = 6;

    // constants are ALWAYS immutable and must ALWAYS be annotated
    const HOURS_IN_DAY: u8 = 24;

    // constants can be anything that is a constant expression (no runtime evaluted code)
    const SEC_IN_HOUR: u16 = 60 * 60;

    // shadowing var is just rebinding the name
    let x = 3;
    let x = 4;

    /*
     * DATA TYPES
     */

    // integer data types. all the 'i's here can be replaced by 'u's
    let x: i8;
    let x: i16;
    let x: i32;
    let x: i64;
    let x: i128;
    let x: isize; // # of bits depends on architecture

    let x: u128 = 98_222; // decimal
    let x: u128 = 0x128; // hex
    let x: u128 = 0b1010101110101; // binary
    let x: u8 = b'a'; // bytes

    // float data types
    let x: f32;
    let x: f64;

    // bools
    let x: bool;

    // chars--different from bytes, one char = 4 bytes because unicode
    let x: char = 'z';
    println!("size of char is {}", std::mem::size_of::<char>());

    // compound types: tuples
    let x: (i32, u32, char) = (-234, 234, 'b');
    // access index of tuple
    x.0;
    x.1;
    x.2;

    // compound types: array
    let x = [1, 2, 3, 4];
    let x: [i32; 3] = [1, 3, 5];
    // make first ele repeat for brevity
    assert_eq!([3; 5], [3, 3, 3, 3, 3]);
    // rust panics at runtime if you access out-of-bounds ind:
    // let oob = x[5];

    /*
     * FUNCTIONS
     */

    fn func1(x: i32) -> () {
        println!("was passed {} as x", x);
    }

    fn func2(x: i32, y: i32) -> i32 {
        x + y // no `return` keyword means this expr is returned
    }

    // `let` and `fn` are *statements*. this is diff from *expressions*. statements do not return
    // values (they evaluate to unit type, like in ocaml), so you can't do stuff like
    // `let x = (let y = 6)` like you would do in C.
    //
    // expressions, on the other hand, evaluate to values. expressions don't end in semicolons,
    // though, which explains the logic above

    // block quotes create expressions
    let x = {
        let y = 3;
        y + 4
    };

    fn func3() -> () {
        let x = "this is pointless, man";
    }

    fn func4() -> &'static str {
        "this isn't pointless, man!"
    }

    /*
     * CONDITIONALS
     */

    // conditional expressions MUST evaluate to a bool, not like C where anything not 0 is true
    // technically called "if expressions"
    if 3 > 1 {
        println!("3 is greater than 1!");
    } else {
        println!("huh");
    }

    // this doesn't work:
    //
    // if 3 {
    //     println!("3");
    // } else {
    //     println!("huh");
    // }

    // no paren's noiceeee
    let number = 5;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // because conditionals are expressions, they yield a value
    let x = if 5 > 1 { 5 } else { 1 };

    // also each branch need to yield to same value. bad:
    //
    // let x = if 5 > 1 { 5 } else { 'y' };

    // loop forever until break
    let mut i = 0;
    loop {
        if i == 10 {
            break;
        }
        i += 1;
    }

    // you can have labels to loops
    let mut i = 0;
    'first_loop: loop {
        i += 1;
        let mut j = 0;

        'second_loop: loop {
            if i * j == 100 {
                break 'first_loop;
            } else if j == 10 {
                break 'second_loop;
            }
            j += 1;
        }
    }

    // loops can't break with a returned value
    let mut i = 0;
    let x = loop {
        i += 1;

        if i == 10 {
            break i * 2;
        }
    }; // x = 20

    // conditional loops
    let mut i = 0;
    while i < 10 {
        i += 1;
    }

    // for loops are more concise though, and less error-prone than while loops. for loops can't
    // have out-of-bounds exception for example
    let x = [1, 2, 3, 4];
    for ele in x {
        println!("{}", ele);
    }
}

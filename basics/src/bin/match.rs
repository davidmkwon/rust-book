// https://doc.rust-lang.org/book/ch18-00-patterns.html

#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]

fn main() {
    /*
     * Places where patterns can be used
     */
    // 1. match arms:
    //
    // match arm {
    //   PATTERN => EXPRESSION,
    //   PATTERN => EXPRESSION,
    //   PATTERN => EXPRESSION,
    // }
    //
    // note that match statements must be exhaustive. you can use either a variable or `_` as a
    // "catch all" for all the remaining cases.

    // 2. if let expressions:
    //
    // a way to check one match case and execute some code accordingly. you can have else if/elses.
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {}, as the background", color);
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }

    // 3. while let expressions:
    //
    // same as if let but continues to run as long as it matches
    let mut vec = vec![1, 2, 3];
    while let Some(val) = vec.pop() {
        println!("{}", val);
    }

    // 4. for loops:
    //
    // the value that directly follows the `for` keyword is the pattern
    let vec = vec![1, 2, 3];
    for (index, value) in vec.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    // 5. let statements:
    //
    // let PATTERN = EXPRESSION;
    let x = 5;
    let (x, y, z) = (1, 2, 3);

    // 6. function parameters:
    //
    // note that the pattern can't be refutable, that is, you have to cover every case with this
    // one pattern. so this won't work:
    //
    // fn hi(Some(x): Option<i32>);
    //
    // but this will:
    //
    // fn hi(&(x, y): &(i32, i32));

    /*
     * Refutability
     *
     * Irrefutable Pattern: a pattern that will match for any possible value
     * Refutable Pattern: a pattern that can fail to match for some value
     *
     * function parameters, let statements, and for loops can only have irrefutable patterns.
     */

    /*
     * Pattern syntax
     *
     * Just look at this:
     * - https://doc.rust-lang.org/book/ch18-03-pattern-syntax.html
     */
    enum Message {
        Hello { id: i32 },
    }
    let msg = Message::Hello { id: 5 };
    match msg {
        Message::Hello { id: 3..=5 } => {
            println!("");
        }
        Message::Hello { id: _ } => (),
    };
}

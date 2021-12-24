// https://doc.rust-lang.org/book/ch06-00-enums.html

#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]

fn main() {
    /*
     * ENUMS
     */
    // enum's express different variants that are still under one type
    enum IpAddrKind {
        V4(u8, u8, u8, u8),
        V6(String), // this variant has a String as an associated type
    }

    // construct variants, IpAddrKind::V4() and V6() variants are basically function calls that
    // return IpAddrKind type
    let four = IpAddrKind::V4(127, 0, 0, 1);
    let six = IpAddrKind::V6(String::from("127.0.0.1"));

    enum Message {
        Quit,
        Move { x: i32, y: i32 }, // has named fields like struct
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    // options are OP
    let some_num = Some(10);
    let no_num: Option<&str> = None;

    /*
     * PATTERN MATCHING
     */
    impl Message {
        fn as_str(&self) -> String {
            match self {
                Message::Quit => String::from("Quit"),
                Message::Move { .. } => String::from("Move"),
                Message::Write(s) => s.to_string(),
                Message::ChangeColor(r, _, _) => r.to_string(),
            }
        }
    }
    let m = Message::Quit;
    println!("{}", m.as_str());
    let m = Message::Write(String::from("write message"));
    println!("{}", m.as_str());

    // "if let" statements are basically when you want to do some action only if an enum is a
    // specific variant. so say below we only want to print out a message if the Message m is the
    // Write variant. if it is, we bind the String to s and go into the if statement.
    //
    // you can have an associated else that is the equivalent of having the "_" catch all case in a
    // match expression
    let m = Message::Quit;
    if let Message::Write(s) = m {
        let str = s.as_str();
        println!("{}", str);
    } else {
        println!("m is either a Quit, Move, or ChangeColor");
    }
}

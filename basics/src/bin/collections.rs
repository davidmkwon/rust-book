// https://doc.rust-lang.org/book/ch08-00-common-collections.html

#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]

use std::collections::HashMap;

fn main() {
    /*
     * Vectors
     */
    // create vec
    let v = Vec::<i32>::new();
    let v: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3];

    // modify vec
    let mut v = Vec::new();
    v.push(5);
    v.push(4);
    v.push(3);

    // read from vec
    let third: &i32 = &v[2];
    let third = match v.get(6) {
        Some(num) => num,
        None => {
            println!("3rd element does not exist");
            &-1
        }
    };
    println!("{}", third);

    // iterate vec
    for i in &v {
        println!("{}", *i);
    }
    for i in &mut v {
        *i += 3;
    }

    /*
     * Strings
     */
    // recap: &str is an immutable borrowed reference to a str (some UTF-8 sequence of string data)
    // stored somewhere, and with literal strings that "somewhere" is in the binary. String is an
    // owned type for a mutable UTF-8 encoded string
    let s = "initial";
    let s = "initial".to_string();
    let s = String::from("initial");

    // apend to string
    let mut s = String::from("initial");
    s.push_str(" contents");
    s.push('!');

    // add strings
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // note s1 has been moved into s3 here and can no longer be used
    println!("{}", s2); // s2 is not moved

    // can't index strings because utf-8
    // error: let H = s3[0];
    //
    // takeaway of the whole Strings chapter here: what you think is a character in a word might
    // not actually be one byte, so it basically gets super confusing when thinking about how to
    // index into strings. that's why rust has chars, which are 4 bytes

    /*
     * Hashmaps
     */
    // make them
    let mut map = HashMap::new();
    map.insert(3, String::from("three"));
    map.insert(4, String::from("four"));

    // makes same map as above
    let nums = vec![3, 4];
    let s_nums = vec![String::from("three"), String::from("four")];
    // the _ will make the compiler infer types
    let mut map: HashMap<_, _> = nums.into_iter().zip(s_nums.into_iter()).collect();

    // getting values
    let x = match map.get(&3) {
        Some(v) => v,
        None => panic!("no value breh"),
    };
    for (k, v) in &map {
        println!("key: {}, val: {}", k, v);
    }

    // updating values
    map.insert(3, String::from("cool three"));
    // entry() returns an Entry, upon which calling or_insert() will either put the value "five" in
    // for the key 5 if there was no value for 5 or do nothing. in either case it returns a &mut to
    // the value
    let v_five = map.entry(5).or_insert("five".to_string());
    v_five.push('!'); // because it's mutable

    // updating val based on old val
    //
    // this program counts num occurences for a char
    let mut frequency = HashMap::new();
    let input = "hello world wonderful world";
    for c in input.chars() {
        let count = frequency.entry(c).or_insert(0);
        *count += 1;
    }
    println!("{:?}", frequency);
}

//https://doc.rust-lang.org/book/ch13-00-functional-features.html

#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]

use std::collections::HashMap;

fn main() {
    /*
     * A closure is basically a lambda
     */
    // `||` captures parameter list
    let add_one = |num| num + 1;
    let two = add_one(1);

    // the compiler can infer the type of the arguments and return type usually. if not (or you
    // want more clarity) you can specifyc
    let add_one = |num: i32| -> i32 { num + 1 };
    let two = add_one(1);

    // note however that if you don't write annotations, the types can only be inferred once you
    // use the closure. moreover, once the closure is used the types are "set", and you can't use
    // it another way.
    let print_x = |x| {
        println!("{}", x);
    };
    print_x("yo");
    // this gives an error because compiler has now inferred the x parameter to have type &str:
    // print_x(3);

    // a struct that caches a closure's computation:
    struct Cacher<T, I, R>
    where
        T: Fn(I) -> R,
        I: Eq + std::hash::Hash + Copy,
    {
        calculation: T,
        cache: HashMap<I, R>,
    }
    impl<T, I, R> Cacher<T, I, R>
    where
        T: Fn(I) -> R,
        I: Eq + std::hash::Hash + Copy,
    {
        fn new(calculation: T) -> Cacher<T, I, R> {
            Cacher {
                calculation,
                cache: HashMap::new(),
            }
        }

        fn compute(&mut self, arg: I) -> &R {
            if self.cache.get(&arg).is_none() {
                let result = (self.calculation)(arg);
                self.cache.insert(arg, result);
                self.cache.get(&arg).unwrap()
            } else {
                self.cache.get(&arg).unwrap()
            }
        }
    }

    // closures can capture things from their environment like capture list in cpp
    let x = 4;
    let equal_to_x = |num| num == x;
    let is_equal = equal_to_x(4);

    // ^this only works with closures, this would fail if done with a function
    // fn equal_to_x2(num: i32) -> bool {
    //     num == x
    // }

    // all closures implement at least one of the traits `Fn`, `FnMut`, `FnOnce`.
    //
    // `Fn`:     borrows values from env immutably
    // `FnMut`:  borrows values from env mutably
    // `FnOnce`: takes ownership of values in env and moves them inside the closure. "Once" refers
    // to that you can only call this closure once since you can't take ownership more than once.
    //
    // Note that the compiler will infer what traits a given closure implements. All closures
    // implement FnOnce because they can be called once. Closures that don't move values impl
    // FnMut, and those that don't change them impl Fn.
    //
    // you can use keyword `move` to force closure to take ownership of env variables. this is
    // often done in creating threads. this however doesn't mean that move closures can't impl Fn
    // or FnMut, as what trait they impl is defined by by what the closure does with the captures
    // values, not how it captures them.

    let x = vec![1, 2, 3];
    let equal_to_x = move |z| z == x;
    // this is an error because x has now been moved (it wouldn't have been if we didn't write
    // move):
    // println!("{:?}", x);
    assert!(equal_to_x(vec![1, 2, 3]));

    /*
     * Iterators
     *
     * A unified way to expose a type as a sequence of items and to iterate over them.
     *
     * The trait:
     *
     * pub trait Iterator {
     *     type Item;
     *
     *     // note that next() takes mutable references to self because it mutates some state
     *     fn next(&mut self) -> Option<Self::Item>;
     * }
     *
     * .iter(): returns references, takes `&self`
     * .iter_mut(): returns mutable references, takes `&mut self`
     * .into_iter(): returns owned values, takes `self`
     */

    // you can call other functions that work on any impl Iterator, like map.
    let v1 = vec![1, 2, 3];
    let v2: Vec<i32> = v1.iter().map(|x| x + 3).collect();
    assert_eq!(vec![4, 5, 6], v2);

    // our own iterator
    struct Counter {
        count: u32,
    }
    impl Counter {
        fn new() -> Counter {
            Counter { count: 0 }
        }
    }
    impl Iterator for Counter {
        type Item = u32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.count < 5 {
                self.count += 1;
                Some(self.count)
            } else {
                None
            }
        }
    }

    // because we defined next() for our iterator, we can call all the other Iterator default
    // methods that just rely on the def of next():
    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();
    assert_eq!(18, sum);

    let mut map_iter = Counter::new().map(|x| x + 1);
    println!("{:?}", map_iter.next());

    /*
     * Performance
     *
     * tldr: compiler does OP optimizations that makes using iterators about the same as
     * performance (or sometimes even faster) than writing out the loops by hand--part of the
     * "zero-cost abstractions" principle.
     */
}

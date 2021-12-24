// https://doc.rust-lang.org/book/ch17-00-oop.html

#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]

fn main() {
    // you can do oop-like stuff mainly through trait objects. trait objects are basically pointers
    // to types that implement a specific trait. what's important is that the types don't have to
    // be the same, they just all need to impl the trait. you can only have trait objects behind a
    // pointer though (either &, Box<T>, etc.)
    //
    // the pointer to the trait object holds (i believe) a pointer to the actual instance of the
    // type, but also a pointer to a vtable-like structure that holds the functions for this type.

    // so Screen holds diff types that are Drawable
    trait Draw {
        fn draw(&self);
    }
    struct Line;
    impl Draw for Line {
        fn draw(&self) {
            println!("-----");
        }
    }
    struct Rect;
    impl Draw for Rect {
        fn draw(&self) {
            println!("-----");
            println!("|   |");
            println!("-----");
        }
    }
    let l = Box::new(Line {});
    let r = Box::new(Rect {});
    let v: Vec<Box<dyn Draw>> = vec![l, r];
    for component in v {
        component.draw();
    }

    // static dispatch: what generics do. basically at compile time the concrete types are
    // substituted in for the generics and there is no runtime penalty
    //
    // dynamic dispatch: what trait objects do. because there's no way to know at compile time the
    // underlying type of each trait object (i.e you add one of three impl trait types depending on
    // what the user inputs to a vec, so you have no way of knowing at compile time which concrete
    // types are in the vector), at runtime rust checks the internal pointer for the trait object
    // (i think it's equivalent of a vtable pointer in c++) to see which type's function to call.

    // object safety: the constraints by which a trait can be made into a trait object.
    //
    // object safe traits:
    // - don't have a return type of Self
    // - have no generic type paramters
    //
    // the reasoning for these is that because Rust doesn't know at runtime the concrete type
    // behind the trait objects, it can't know what Self is.

    struct Person {
        name: String,
    }
    impl Person {
        fn take(self: &Self) -> &str {
            &self.name
        }
    }

    let p = Person {
        name: "david".to_string(),
    };
    //let name = Person::take(&p);
    let name = p.take();
    println!("{}", name);
}

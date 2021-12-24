// https://doc.rust-lang.org/book/ch15-00-smart-pointers.html

#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]

use std::cell::{Ref, RefCell, RefMut};
use std::ops::Deref;
use std::rc::{Rc, Weak};

fn main() {
    /*
     * Box<T>
     *
     * A Box is simply a smart pointer that manages data on the heap.
     * - you can pass around an object to some Type whose size might not be know at compile time.
     * (but we know the size of a Box)
     * - transferring ownership of a large amount of data is trivial (copy pointer)
     * - own trait objects
     */
    println!("Box<T>: ");
    let b = Box::new(5);
    println!("b = {}", b);

    // boxes enable recursive types (types that have themselves as fields). because a box's size is
    // known, the compiler knows exactly how much space the recursive type will take in memory. if
    // we didn't use box the compiler would infinitely loop to determine how large the type is.
    //
    // a lisp-style const list:
    enum List<T> {
        Cons(T, Box<List<T>>),
        Nil,
    }
    println!("");

    /*
     * Deref Trait
     */
    // the thing that allows us to dereferences references (and smart pointers) is the Deref trait.
    //
    // if you tried comparing 5 with just y, then it would error because they're diff types
    println!("Deref: ");
    let x = 5;
    let y = Box::new(x);
    assert_eq!(5, *y);

    // our own box (except stores on stack instead)
    struct MyBox<T>(T);
    impl<T> MyBox<T> {
        fn new(t: T) -> MyBox<T> {
            MyBox(t)
        }
    }
    // make it dereferencable:
    impl<T> Deref for MyBox<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    // we can now dereference it
    let x = MyBox::new(5);
    assert_eq!(5, *x); // behind the scenes rustc makes `*x` -> `*(x.deref())`

    // deref coercion: when you pass a type that impl Deref as a reference to a function, rustc
    // will keep converting it to the type deref() returns for this type until it matches the
    // function's paramter type.
    //
    // so for example deref() for String gives &str. so if a function expects &str, and we give it
    // &String, we convert it to &str.
    fn hello(s: &str) {
        println!("{}", s);
    }
    let m = MyBox::new(String::from("yooo"));
    // note: this showcases the DFS nature of conversion. first the &MyBox<String> is turned into
    // &String, which is then turned into &str.
    hello(&m);

    // deref coerction for &mut done via impl DerefMut
    //
    // quick summary(?) of when deref coercion will convert:
    // 1. From &T to &U when T: Deref<Target=U>
    // 2. From &mut T to &mut U when T: DerefMut<Target=U>
    // 3. From &mut T to &U when T: Deref<Target=U>
    //
    // note rule 3: you can conert a &mut T to &T, but not vice versa.
    //
    // i think deref coerction just takes dc: &T and does &(*dc)?. aka just calls .deref() LMAO
    let m = &m;
    fn hello2(s: &String) {
        println!("{}", s);
    }
    hello2(&(*m));

    println!("");

    /*
     * Drop Trait
     *
     * The drop() function in the Drop trait specifies the code that is called when an object of
     * this type goes out of scope. Note that items are dropped in reverse order of how they're
     * created
     */
    println!("Drop: ");
    struct MyBox2(String);
    impl Drop for MyBox2 {
        fn drop(&mut self) {
            println!("dropping my box: {}", self.0);
        }
    }
    // mb2 is dropped them mb
    let mb = MyBox2("a".to_string());
    //let mb2 = MyBox2("b".to_string());
    let mb2 = mb;

    // you can manually call drop if you want
    let mb3 = MyBox2("yurt".to_string());
    drop(mb3);

    /*
     * Rc<T>
     *
     * Basically a non thread-safe shared_ptr. Will free underlying object when rc is 0.
     */
    println!("Rc<T>: ");
    // We can create a list where there can be multiple heads pointing to the rest of the list
    enum RcList<T> {
        Cons(T, Rc<RcList<T>>),
        Nil,
    }

    let a = Rc::new(RcList::Cons(
        5,
        Rc::new(RcList::Cons(10, Rc::new(RcList::Nil))),
    ));
    // we use Rc::clone() rather than a.clone() out of convention because the latter is
    // conventionally done when the clone call creates a deep copy, but Rc::clone() makes a
    // shallow-ish copy (only work is inc RC)
    let b = RcList::Cons(3, Rc::clone(&a));
    let c = RcList::Cons(4, Rc::clone(&a));
    // the ref count is 3, cuz a, b, c all refer inc a's RC. when c is dropped, the rc is now 2
    println!("rc of a: {}", Rc::strong_count(&a));
    drop(c);
    println!("rc of a after dropping c: {}", Rc::strong_count(&a));

    // the reason this wouldn't work is because Rc doesn't impl DerefMut, which allows you to
    // basically have the Rc be a mut reference. because it only impl Deref, you only have read
    // access to the underlying data.
    //
    // let mut x = Rc::new("hi".to_string());
    // let y = Rc::clone(&x);
    // x.push_str("yooo");
    //
    // if you're wondering why use Rc then if you can just have multiple immut refs, the answer is
    // that sometimes you don't know which will be the last immut ref. this leads to complicated
    // lifetime logic and stuff. With Rc, you can have as many immut refs as you want and you can
    // trust that the last one to be dropped will take care of freeing the underlying data.

    println!("");

    /*
     * RefCell<T>
     *
     * Emulates a Box<T>(single owner), but the reference checking (one &mut or any &) is checked
     * at runtime, and panics if violated rather than compile time checking.
     *
     * When to use? when you know that your code is obeying the borrowing rules but the compiler
     * doesn't think so. because rustc is conservative there might be times it doesn't approvate a
     * program even though you know it's correct.
     *
     * - Rc<T> enables multiple owners of the same data; Box<T> and RefCell<T> have single owners.
     * - Box<T> allows immutable or mutable borrows checked at compile time; Rc<T> allows only
     *   immutable borrows checked at compile time; RefCell<T> allows immutable or mutable borrows
     *   checked at runtime.
     * - Because RefCell<T> allows mutable borrows checked at runtime, you can mutate the value
     *   inside the RefCell<T> even when the RefCell<T> is immutable.
     *
     * Interior Mutability: design pattern where you can mutate data even when there are immutable
     * references to the data--RefCell uses this
     */
    println!("RefCell<T>: ");
    // example:
    //
    // so from an interface POV, logging shouldn't mutate the state of the logger. But this might
    // complicate the design of the underlying implementations that might happen to mutate some
    // state (but isn't visible to the user, all they care is that the Logger succesfully logged
    // the message). Then we can use a RefCell here to be able to mutate the data with just a immut
    // ref.
    trait Logger {
        fn log(&self, msg: &str) -> Result<(), String>;
    }
    struct VecLogger {
        logs: RefCell<Vec<String>>,
    }
    impl Logger for VecLogger {
        fn log(&self, msg: &str) -> Result<(), String> {
            self.logs.borrow_mut().push(msg.to_string());
            Ok(())
        }
    }

    // under the hood, RefCell just works by having an internal count of # &T's and &mut T's. it
    // enforces the same references rules the compiler does but just at runtime

    // borrow() and borrow_mut() return their own types, that implement Deref.
    let r = RefCell::new(5);
    let rb1: Ref<i32> = r.borrow();
    let rb2: Ref<i32> = r.borrow();
    let rb3: Ref<i32> = r.borrow();
    // NOTE: RefCell doesn't have access to the source code like rustc does, and can't tell that
    // rb1,2,3 are not used after creating rbm1 and thus it's valid to make rbm1. so we have to
    // deliberately drop them so that the internal refcounts in RefCell are decremented
    drop(rb1);
    drop(rb2);
    drop(rb3);
    let rbm1: RefMut<i32> = r.borrow_mut();
    // this would panic at runtime:
    // let rbm2: RefMut<i32> = r.borrow_mut();
    println!("{}", rbm1);

    // you can use RefCell with Rc to have multiple owners to see data that can be mutated
    //
    // i was pretty confused by this example ngl. I think at it's essence it's showing that
    // with Rc<RefCell<T>> each list can "own" this data (through the Rc), but even then you're
    // still able to mutate the data that they "own" via borrow_mut(). so BASICALLY you want to
    // just have all the List's refer to a common RefCell, but because a RefCell can only have one
    // owner (and we don't want to deal with annoying borrow stuff) we just wrap it in an Rc and
    // clone the Rc.
    #[derive(Debug)]
    enum MultipleList<T> {
        Cons(Rc<RefCell<T>>, Rc<MultipleList<T>>),
        Nil,
    }
    let value = Rc::new(RefCell::new(5));
    let value2 = Rc::clone(&value);
    let a = Rc::new(MultipleList::Cons(
        Rc::clone(&value),
        Rc::new(MultipleList::Nil),
    ));
    let b = MultipleList::Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = MultipleList::Cons(Rc::new(RefCell::new(7)), Rc::clone(&a));
    // mutate value!
    let mut vr = value.borrow_mut();
    *vr += 10;
    drop(vr);
    // mutate the data through a's "owned" value
    if let MultipleList::Cons(data, _) = &*a {
        *data.borrow_mut() += 3;
        println!("rc: {}", Rc::strong_count(data));
    }
    // mutate again, but don't drop the mutable ref yet. then the prints won't show current data
    // value in the lists because you need an immut ref to do that
    let mut vr = value.borrow_mut();
    *vr += 10;
    // we can see that all lists see this updated value
    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);

    println!("");

    /*
     * Reference Cycles Leak
     */
    println!("Reference Cycles: ");
    // this is a list where you can modify which CycleList the Cons points to as next
    #[derive(Debug)]
    enum CycleList<T> {
        Cons(T, RefCell<Rc<CycleList<T>>>),
        Nil,
    }
    impl<T> CycleList<T> {
        fn tail(&self) -> Option<&RefCell<Rc<CycleList<T>>>> {
            match self {
                CycleList::Cons(_, item) => Some(item),
                CycleList::Nil => None,
            }
        }
    }
    // create a list loop by pointing a to b and b to a
    let a = Rc::new(CycleList::Cons('a', RefCell::new(Rc::new(CycleList::Nil))));
    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());
    let b = Rc::new(CycleList::Cons('b', RefCell::new(Rc::clone(&a))));
    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());
    if let Some(a_tail) = a.tail() {
        *a_tail.borrow_mut() = Rc::clone(&b);
    }
    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));
    // This will cause a stack overflow because of this loop
    // println!("a next item = {:?}", a.tail());

    // where is the memory leak above? when we go out of scope here, b will be dropped first, so
    // the refcount of b is 1 (a refers to it). then a goes out of scope, and it's refcount will
    // also become 1. there is now a memory leak because neither will be freed

    // you can get a "weak" smart pointer from an Rc. the Rc will internally have a count of
    // regular references and also Weak references. it will free the data when # regular refs is 0
    // EVEN if there are still weak references.
    let r = Rc::new(5);
    let wr = Rc::downgrade(&r);
    drop(r);
    println!("strong rc count: {}", wr.strong_count());
    // so whenever you use a weak ptr you have to check that the underlying data is there. you can
    // do via .upgrade() which returns Some(Rc<T>) if it exists else None
    if let Some(wr) = wr.upgrade() {
        println!("{}", wr);
    } else {
        println!("there are no strong references anymore");
    }

    // a good use case of Weak: a tree, where you want parent's to point to their children but also
    // children to point back to their parents. can't use Rc for both or else we will have loop
    #[derive(Debug)]
    struct TreeNode {
        value: i32,
        parent: RefCell<Weak<TreeNode>>,
        children: RefCell<Vec<Rc<TreeNode>>>,
    }
    let leaf = Rc::new(TreeNode {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(Vec::new()),
    });
    let parent = Rc::new(TreeNode {
        value: 10,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });
    // update the leaf to get weak pointer to parent
    *leaf.parent.borrow_mut() = Rc::downgrade(&parent);
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    println!("");
    println!("Other");
}

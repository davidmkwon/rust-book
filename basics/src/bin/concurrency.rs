// https://doc.rust-lang.org/book/ch16-00-concurrency.html

#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]

use std::{
    sync::{mpsc, Arc, Mutex, MutexGuard},
    thread,
    time::Duration,
};

fn main() {
    /*
     * Threads
     *
     * Rust threads are 1-1 mapped to OS ones. It doesn't support green threads because it must
     * maintain almost no runtime overhead.
     */
    println!("Threads");
    // pass closures to threads
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi #{} from spawned thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    // wait for thread to complete
    handle.join().unwrap();
    println!("");

    // when capturing variables from the env in a closure, we often use `move` with the closure to
    // move ownership of the variables into the closure. if the closure just took vars as a
    // reference, we don't know how long the thread might live, and it might outlive the lifetime
    // of the variable it references, so that wouldn't work. thus we must move the variable into
    // the closure and thus the thread.
    let mut v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        v.push(4);
    });
    // if rust let us do this without moving, this could happen! which would be bad!
    // drop(v);
    handle.join().unwrap();

    /*
     * Message passing
     */
    println!("Message passing");
    // `mpsc` means "multiple producer, single consumer" which is exactly what it sounds like.
    //  this returns a transmitter (`tx`) and receiver (`rx`)
    let (tx, rx) = mpsc::channel();
    // move transmitter to thread and send data on it
    thread::spawn(move || {
        // im pretty sure send is non-blocking
        tx.send("hi".to_string()).unwrap();
    });
    // this checks if something has been received, doesn't block
    let recv_immed = rx.try_recv();
    // this blocks
    let recv = rx.recv().unwrap();
    println!("received {}", recv);

    // we can iterate over received messages, the iteration will end when the channel is closed
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    for received in rx {
        println!("received {}", received);
    }

    // example that emphasizes the "multiple producer"
    let (tx1, rx) = mpsc::channel();
    let tx2 = tx1.clone();
    // send on tx1
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    // send on tx2
    thread::spawn(move || {
        let vals = vec![
            String::from("ayo"),
            String::from("fruhm"),
            String::from("da"),
            String::from("threag"),
        ];

        for val in vals {
            tx2.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    for received in rx {
        println!("received {}", received);
    }
    println!("");

    /*
     * Shared state concurrency
     */
    println!("Shared state concurrency");
    // mutex = mutual exclusion :)
    let m = Mutex::new(5);
    {
        // lock() returns an Result where Err means that another thread with the lock paniced, and
        // thus getting the lock is impossible
        let mut num: MutexGuard<i32> = m.lock().unwrap();
        *num = 6;
        // the MutexGuard is dropped here which unlocks the mutex
    }
    println!("m = {:?}", m);

    // using a mutex across threads
    //
    // - we can't use just a mutex, as ownership rules will prevent having it across multiple
    //   threads
    // - we can't use an Rc<Mutex> and clone them and send them into threads because Rc doesn't
    //   impl Send. this is because the Rc doesn't make sure that inc and dec of the refcount is
    //   thread-safe
    //
    // Arc: thread safe Rc
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            let mut count = counter.lock().unwrap();
            *count += 1;
        }));
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("counter: {:?}", counter);

    // just like RefCell is paired with Rc, Mutex is often paired with Arc
    // - both RefCell and Mutex provide interior mutability, you can mutate internal state with an
    //   immut ref that you have in Rc/Arc
    println!("");

    /*
     * Send and Sync
     *
     * impl Send means that ownership of this type can be moved across threads. any type composed
     * of all Send types makes the type Send also.
     *
     * impl Sync means that referencing T from other threads is safe. that is, &T is Send. any
     * types composed of all Sync types makes the type Sync also.
     *
     * Rc is not Send or Sync. RefCell is not Sync
     *
     * implementing Send or Sync for types is inherently unsafe, because types whose fields are all
     * Send/Sync are automatically Send/Sync, so if you have to manually implement it then it's
     * unsafe.
     */
    println!("Send and Sync");
    println!("");
}

use std::collections::vec_deque::VecDeque;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate,
}

pub struct ThreadPool {
    workers: VecDeque<Worker>,
    sender: mpsc::Sender<Message>,
}

impl ThreadPool {
    pub fn new(n: usize) -> ThreadPool {
        assert!(n > 0);

        let mut workers = VecDeque::with_capacity(n);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        for id in 0..n {
            workers.push_back(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Message::NewJob(Box::new(f));
        self.sender.send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        // send shutdown to all the workers
        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        // join the worker thread
        for worker in &mut self.workers {
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let msg = receiver.lock().unwrap().recv().unwrap();
            match msg {
                Message::Terminate => {
                    println!("terminating worker {}", id);
                    break;
                }
                Message::NewJob(job) => {
                    println!("running job on worker {}", id);
                    job();
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

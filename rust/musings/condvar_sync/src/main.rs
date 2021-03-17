// Learning Source:
// Lecture 11: Synchronization | Stanford CS 110L (Safety in Systems Programming)

use std::sync::{Arc, Condvar, Mutex};

use rand::Rng;
use std::collections::VecDeque;
use std::{thread, time};

fn rand_sleep() {
    let mut rng = rand::thread_rng();
    thread::sleep(time::Duration::from_millis(rng.gen_range(0..30)));
}

#[derive(Clone)]
pub struct SemaPlusPlus<T> {
    // semaphores idiomatically consists of a pair of (Mutex<_>, Condvar)
    // you use the Condvar to wait until you are able to acquire the data that the Mutex guards.
    queue_and_cv: Arc<(Mutex<VecDeque<T>>, Condvar)>,
}

impl<T> SemaPlusPlus<T> {
    pub fn new() -> Self {
        SemaPlusPlus {
            queue_and_cv: Arc::new((Mutex::new(VecDeque::new()), Condvar::new())),
        }
    }

    // Enqueues onto the VecDeque as a send operation.
    pub fn send(&self, message: T) {
        // acquire a lock to exclusively push onto the VecDeque.
        let mut queue = self.queue_and_cv.0.lock().unwrap();
        queue.push_back(message);

        // signal to all threads waiting on the Condvar that data exists within it to read
        // under contention only 1 thread will 'win' the lock and be able to read from or write to
        // the VecDeque
        if queue.len() == 1 {
            self.queue_and_cv.1.notify_all();
        }
    }

    // Dequeues from the VecDeque as receive operation.
    pub fn recv(&self) -> T {
        let mut queue = self
            .queue_and_cv
            .1
            // current thread of execution is blocked until the predicate within the wait_thile
            // evaluates to false signifying that value exists within VecDeque to be read.
            .wait_while(self.queue_and_cv.0.lock().unwrap(), |queue| {
                queue.is_empty()
            })
            .unwrap();

        // safe the unwrap; Condvar guarantees value exists in the VecDeque to read.
        queue.pop_front().unwrap()
    }
}

const NUM_THREADS: usize = 16;

fn main() {
    let sem: SemaPlusPlus<String> = SemaPlusPlus::new();
    let mut handles = Vec::new();

    for i in 0..NUM_THREADS {
        let sem_clone = sem.clone();
        let handle = thread::spawn(move || {
            rand_sleep();

            // Message to be sent is a string.
            sem_clone.send(format!("thread {} just finished!", i))
        });
        handles.push(handle);
    }

    for _ in 0..NUM_THREADS {
        // print the message received
        println!("{}", sem.recv());
    }

    // wait for all threads to finish.
    for handle in handles {
        handle.join().unwrap();
    }
}

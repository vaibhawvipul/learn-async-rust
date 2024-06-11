// we will explore channels in this project
// channels are a way to communicate between threads
// we will create a channel and send a message from one thread to another
// we will also see how to send multiple messages and receive them

// to begin with I am using this video as a reference - https://www.youtube.com/watch?v=b4mS5UPHh20

use std::sync::{Arc, Mutex, Condvar};

pub struct Sender<T> {
    inner: Arc<Mutex<Inner<T>>>,
}

pub struct Receiver<T> {
    inner: Arc<Mutex<Inner<T>>>,
}

struct Inner<T> {
    buffer: Vec<T>,
    available: Condvar,
}

pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let inner = Arc::new(Mutex::new(Inner { buffer: Vec::new(), available: Condvar::new() }));
    let sender = Sender {
        inner: inner.clone(),
    };
    let receiver = Receiver {
        inner: inner.clone(),
    };
    (sender, receiver)
}


#[tokio::main]
async fn main() {
    println!("Hello, world!");
}

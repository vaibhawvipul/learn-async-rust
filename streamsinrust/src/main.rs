// Note: This is my notes on streams in Rust. It is not my original work.
// reference - 
// 1. https://www.qovery.com/blog/a-guided-tour-of-streams-in-rust/
// 2. https://tokio.rs/tokio/tutorial/streams
// 3. https://subscription.packtpub.com/book/programming/9781805128137/9/ch09lvl1sec48/a-mental-model-of-an-async-runtime

// A stream is an async iterator that produces a sequence of values over time, rather than all at once.

// sync iterator
// pub trait iterator {
//     type Item;
//     fn next(&mut self) -> Option<Self::Item>;
// }
// Using an option is useful when receiving a None, as it can inform you that the iterator does not have any elements left, and is now exhausted.

// Stream: An async iterator
// pub trait Stream {
//     type Item;
//     fn poll_next(
//          self: Pin<&mut Self>, 
//          cx: &mut Context
//     ) -> Poll<Option<Self::Item>>;
// }

// Poll: An enum that represents the result of a future or stream being polled.

// simplified version of Stream
// pub trait Stream {
//     type Item;
//     fn next(&mut self) -> impl Future<Output = Option<Self::Item>>;
// }

// If we use async/await syntax, we can write the next method as an async function that returns an Option<Self::Item>:
// pub trait Stream {
//     type Item;
//     async fn next(&mut self) -> Option<Self::Item>;
// }

// Here, we can see that the stream trait is equivalent to the Iterator trait, only with an async keyword in front of the function.


// Some definitions
// Future: A future is a value that represents an asynchronous computation that will eventually produce a value or an error. It is a type of promise that can be used to represent the result of an asynchronous computation.

// Poll: The poll method is used to check if a future has completed. It returns a Poll enum, which can be one of three variants: Read or Pending.
// enum Poll<T> {
//     Ready(T),
//     Pending,
// }

// Context: The Context struct is used to provide information about the current task to the poll method. It contains information such as the waker, which is used to wake up the task when it is ready to be polled again.

// Sinks provide support for asynchronous writing of data.
// Executors are responsible for running asynchronous tasks.

// A common scenario, where you don’t know when your value is ready because it is waiting on an external factor (i.e a network socket with data ready to be read, getting the lock of a mutex). Therefore, instead of wasting CPU time, the future registers the Waker to be called, by passing it to a reactor (kqueue, epoll, io completion) or something that gets notified when a value is potentially ready.

// Pin: The Pin type is used to prevent a value from being moved in memory. It is used to ensure that the value remains at a fixed location in memory, which is important for safety and soundness.
// read - https://fasterthanli.me/articles/pin-and-suffering

use futures::stream::{self, StreamExt};
use tokio;

#[tokio::main]
async fn main() {
    println!("Hello, world! This is a stream example.");
    assert_stream().await;
}

async fn assert_stream() {
    // Create a stream that produces the numbers 1, 2, 3, 4, 5
    // this function creates a stream that, whenever polled, returns a Poll::Ready(next value from the iterator). Basically, the future is never yielding/awaiting, it is always ready when polled
    let stream = stream::iter(vec![1,2,3,4,5]);
    // Stream::iter is useful for tests, when you don’t care about the async/await stuff, and are only interested in the stream of values.
    assert_eq!(stream.collect::<Vec<_>>().await, vec![1,2,3,4,5]);
}
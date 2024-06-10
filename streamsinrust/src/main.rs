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

// Promise: A promise is a value that represents the result of an asynchronous computation. It is used to represent the eventual result of an asynchronous computation, such as a network request or a file read.

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

    // lazily generate an infinite stream of numbers using repeat_with
    let mut curr = 1;
    let add_one = stream::repeat_with(move || {
        let res = curr;
        curr *= 2;
        res
    });
    assert_eq!(add_one.take(5).collect::<Vec<_>>().await, vec![1,2,4,8,16]);

    // Combine two streams into one
    let stream1 = stream::iter(vec![1,2,3]);
    let stream2 = stream::iter(vec![4,5,6]);
    let combined = stream1.chain(stream2);
    assert_eq!(combined.collect::<Vec<_>>().await, vec![1,2,3,4,5,6]);

    // unfold is a function that creates a stream from an initial state and a closure that generates the next value and state
    let curr = 1;
    let unfold = stream::unfold(curr, |state| async move {
        if state < 10 {
            let next_state = state + 1;
            let next_value = state * 2;
            Some((next_value, next_state))
        } else {
            None
        }
    });
    assert_eq!(unfold.collect::<Vec<_>>().await, vec![2,4,6,8,10,12,14,16,18]);

    // difference between repeat_with and unfold
    // repeat_with is a stream that produces the same value over and over again, while unfold is a stream that produces a sequence of values based on an initial state and a closure that generates the next value and state.

    // zip is a function that combines two streams into one, producing a stream of pairs
    let stream1 = stream::iter(vec![1,2,3]);
    let stream2 = stream::iter(vec![4,5,6]);
    let zipped = stream1.zip(stream2);
    assert_eq!(zipped.collect::<Vec<_>>().await, vec![(1,4), (2,5), (3,6)]);

    // streams vs fused streams
    // A fused stream is a stream that has been exhausted, and will always return None when polled. This is useful when you want to create a stream that is done producing values, but you still want to return a stream instead of an iterator.

    // Fuse a stream such that poll_next will never again be called once it has finished. This method can be used to turn any Stream into a FusedStream.
    // Normally, once a stream has returned None from poll_next any further calls could exhibit bad behavior such as block forever, panic, never return, etc.

    // fused stream example
    let mut stream = stream::iter(vec![1,2,3]).fuse();
    assert_eq!(stream.next().await, Some(1));
    assert_eq!(stream.next().await, Some(2));
    assert_eq!(stream.next().await, Some(3));
    assert_eq!(stream.next().await, None);
    assert_eq!(stream.next().await, None);
}
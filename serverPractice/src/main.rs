use std::thread;
use std::thread::sleep;
use std::time::Duration;
use futures::executor::block_on;

//These all code belong to multi threaded 
fn get_two_sites() {
    let thread_one = thread::spawn(|| println!("from two site 1"));
    let thread_two = thread::spawn(|| println!("from two site 2"));

    // Wait for both threads to complete.
    thread_one.join().expect("thread one panicked");
    thread_two.join().expect("thread two panicked");
}

fn get_one_sites() {
    let thread_one = thread::spawn(|| println!("from one site"));
}
/**
fn main() {
    get_one_sites();
    sleep(Duration::from_millis(1000));
    get_two_sites();
} **/

// Async Code 


//async fn do_something() { /* ... */ } //demo code

//It always return a future and future always need to be run a executor 

// `block_on` blocks the current thread until the provided future has run to
// completion. Other executors provide more complex behavior, like scheduling
// multiple futures onto the same thread.
//use futures::executor::block_on;

async fn hello_world() {
    println!("hello, world!");
}

async fn hello_future() {
    println!("hello, world!");
}


fn main() {
    let future = hello_world(); // Nothing is printed
    sleep(Duration::from_millis(5000));
    let future1 = hello_future();
    block_on(future); // `future` is run and "hello, world!" is printed
   futures::join!(future,future1);
}
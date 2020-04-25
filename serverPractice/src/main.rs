use std::thread;
use std::thread::sleep;
use std::time::Duration;

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

fn main() {
    get_one_sites();
    sleep(Duration::from_millis(1000));
    get_two_sites();
}

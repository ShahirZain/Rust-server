use std::thread;
use std::thread::sleep;
use std::time::Duration;
use futures::executor::block_on;
use std::fmt::Debug;


//These all code belong to multi threaded 
// fn get_two_sites() {
//     let thread_one = thread::spawn(|| println!("from two site 1"));
//     let thread_two = thread::spawn(|| println!("from two site 2"));

//     // Wait for both threads to complete.
//     thread_one.join().expect("thread one panicked");
//     thread_two.join().expect("thread two panicked");
// }

// fn get_one_sites() {
//     let thread_one = thread::spawn(|| println!("from one site"));
// }



// fn main() {
//     get_one_sites();
//     sleep(Duration::from_millis(1000));
//     get_two_sites();
// } 

// Async Code 


//async fn do_something() { /* ... */ } //demo code

//It always return a future and future always need to be run a executor 

// `block_on` blocks the current thread until the provided future has run to
// completion. Other executors provide more complex behavior, like scheduling
// multiple futures onto the same thread.
//use futures::executor::block_on;


// async fn hello_world() {
    
//     sleep(Duration::from_millis(5000));
//     println!("hello, world!");
// }

// async fn hello_future() {
//     println!("hello, world!");
// }
// async fn joinedFunc(){
//     let future = hello_world(); // Nothing is printed
//     let future1 = hello_future();
//     futures::join!(future,future1);
// }

// fn main() {
    
//     block_on(joinedFunc()); // `future` is run and "hello, world!" is printed

// } 

async fn learnSong()-> String {
    let song = "learning song";
    sleep(Duration::from_millis(5000));
    return song.to_string();
}

async fn sing(_song: String){
    println!("singing song")
}

async fn dance(){
    println!("dancing");
}

async fn learn_and_sing() {
    // Wait until the song has been learned before singing it.
    // We use `.await` here rather than `block_on` to prevent blocking the
    // thread, which makes it possible to `dance` at the same time.
    let song = learnSong().await;
    sing(song).await;
}

async fn async_main_1(){
    println!("test");
    let f1 = learn_and_sing();
    let f2 = dance();
    futures::join!(f1,f2);
}

fn main (){
  //  let song = learnSong();

    sleep(Duration::from_millis(2000));
   block_on( async_main_1() );
   //async_main();
}



// async fn get_sum() -> f32 {

//     65.2
// }

// async fn calculate_grade(sum : f32) {

//     thread::sleep(Duration::from_secs(3));
//     if sum > 50.0 {
//         println!("Candidate is passed");
//     } else {
//         println!("Candidate is failed");
//     }
// }

// async fn print_sum(sum : f32) {
    
//     println!("Sum is {}",sum);
// }

// async fn get_sum_and_calculate_grade() {
    
//     let sum = get_sum().await;
//     calculate_grade(sum).await;
// }

// async fn oscillator() {
    
//     let f1 = get_sum_and_calculate_grade();
//     let f2 = print_sum(50.62);

//     futures::join!(f2, f1);
// }

// fn main() {
//     block_on(oscillator());
// }
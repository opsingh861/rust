use std::{
    thread::{sleep, spawn},
    time::Duration,
};

fn main() {
    let thread1 = spawn(|| {
        println!("This is from thread 1 and it started");
        sleep(Duration::new(4, 0));
        println!("Thread 1 ended");
    });
    let thread2 = spawn(|| {
        println!("This is from thread 2 and it started");
        for i in 1..10 {
            print!("{} ", i);

            if i == 9 {
                println!("");
            }
        }
        println!("Thread 2 ended");
    });

    /*
    joining the threads are mendatory otherwise main thread will die after spawning new threads, but using join we will tell main thread to wait
    till child threads are alive
     */

    thread1.join().unwrap();
    thread2.join().unwrap();

    // spawn(|| print!("Something")).join().unwrap(); // we can also join like this

}

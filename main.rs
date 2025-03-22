// Multithreading for faster execution

use std::thread;

fn main() {
    let num_threads = 10; // Use 10 threads to aim for 10x speedup
    let total = 100_000_000; // Total numbers to print
    let chunk_size = total / num_threads; // Divide work among threads

    // Store all thread handles
    let mut handles = vec![];

    for thread_id in 0..num_threads {
        // Define the start and end range for each thread
        let start = thread_id * chunk_size;
        let end = if thread_id == num_threads - 1 {
            total
        } else {
            start + chunk_size
        };

        // Spawn a thread to handle the assigned range
        let handle = thread::spawn(move || {
            for i in start..end {
                println!("{}", i);
            }
        });

        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
}

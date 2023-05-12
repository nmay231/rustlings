// threads2.rs
// Execute `rustlings hint threads2` or use the `hint` watch subcommand for a hint.
// Building on the last exercise, we want all of the threads to complete their work but this time
// the spawned threads need to be in charge of updating a shared value: JobStatus.jobs_completed

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

fn main() {
    let status = Arc::new(Mutex::new(JobStatus { jobs_completed: 0 }));
    let handles = (0..10)
        .map(|index| {
            let status = Arc::clone(&status);
            thread::spawn(move || {
                thread::sleep(Duration::from_millis(1000));
                {
                    // Is it a good idea to name all versions the same name? I think so as long as the scopes are not too long.
                    let mut status = status.lock().unwrap();
                    status.jobs_completed += 1;
                }
            })
        })
        .collect::<Vec<_>>(); // We collect to start the threads all at once

    for handle in handles {
        handle.join().unwrap();
        {
            let status = status.lock().unwrap();
            println!("jobs completed {}", status.jobs_completed);
        }
    }
}

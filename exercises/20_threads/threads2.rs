// Building on the last exercise, we want all of the threads to complete their
// work. But this time, the spawned threads need to be in charge of updating a
// shared value: `JobStatus.jobs_done`

use std::{
    sync::Arc,
    sync::Mutex,
    thread,
    time::Duration
};

struct JobStatus {
    jobs_done: u32,
}

fn main() {
    // TODO: `Arc` isn't enough if you want a **mutable** shared state.
    let status = Arc::new(Mutex::new(JobStatus { jobs_done: 0 }));

    let mut handles = Vec::new();
    for i in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            for _ in 0..50 {
                if i % 2 == 0 {
                    thread::sleep(Duration::from_millis(300 + i * 10));
                } else {
                    thread::sleep(Duration::from_millis(250 + 100 - i)); 
                }

                // TODO: You must take an action before you update a shared value.
                let mut data = status_shared.lock().unwrap();
                data.jobs_done += 1;
                // README ==> Mutex is automatically released when data reference is out of scope !!
            }
        });
        handles.push(handle);
    }

    // Waiting for all jobs to complete.
    for handle in handles {
        handle.join().unwrap();
    }

    // TODO: Print the value of `JobStatus.jobs_done`.
    println!("Jobs done: {}", status.lock().unwrap().jobs_done);
}

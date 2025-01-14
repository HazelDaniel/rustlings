// Building on the last exercise, we want all of the threads to complete their
// work. But this time, the spawned threads need to be in charge of updating a
// shared value: `JobStatus.jobs_done`

use std::{sync::Arc, thread, thread::Builder, time::Duration};
use std::sync::Mutex;

struct JobStatus {
    jobs_done: u32,
}

fn main() {
    // TODO: `Arc` isn't enough if you want a **mutable** shared state.
    let status: Arc<Mutex<JobStatus>> = Arc::new(Mutex::new(JobStatus { jobs_done: 0 }));

    let mut handles = Vec::new();
    for _ in 0..10 {
        let status_shared = Arc::clone(&status);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(250));

            // TODO: You must take an action before you update a shared value.
            (*status_shared.lock().unwrap()).jobs_done += 1;
        });
        handles.push(handle);
    }

    let x = Builder::new().name("new thread".to_owned()).stack_size(1024 * 4);


    // Waiting for all jobs to complete.
    for handle in handles {
        handle.join().unwrap();
    }

    let shared_status = Arc::clone(&status);
    let x_handle = x.spawn(move || {
        (*shared_status.lock().unwrap()).jobs_done = (*shared_status.lock().unwrap()).jobs_done;
    });

    x_handle.unwrap().join().unwrap();

    // TODO: Print the value of `JobStatus.jobs_done`.
    println!("Jobs done: {}", status.as_ref().lock().unwrap().jobs_done);


}

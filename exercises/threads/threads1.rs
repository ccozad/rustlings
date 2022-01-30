// threads1.rs
// Make this compile! Execute `rustlings hint threads1` for hints :)
// The idea is the thread spawned on line 22 is completing jobs while the main thread is
// monitoring progress until 10 jobs are completed. Because of the difference between the
// spawned threads' sleep time, and the waiting threads sleep time, when you see 6 lines
// of "waiting..." and the program ends without timing out when running,
// you've got it :)

use std::sync::{Mutex, Arc};
use std::thread;
use std::time::Duration;

struct JobStatus {
    jobs_completed: u32,
}

impl JobStatus {
    fn new() -> JobStatus {
        JobStatus { jobs_completed: 0 }
    }
}

fn main() {
    let job_status = Arc::new(Mutex::new(JobStatus::new()));
    let workers = 0..10;
    let handles: Vec<_> = workers.into_iter().map(|id| {
        let status_shared = Arc::clone(&job_status);
        thread::spawn(move || {
            let elapsed_time = (25 - (7 % (id + 1))) * 10 ;
            thread::sleep(Duration::from_millis(elapsed_time));
            let mut status = status_shared.lock().unwrap();
            status.jobs_completed +=1;
            println!("Job {} completed in {}ms, jobs completed: {}", id, elapsed_time, status.jobs_completed);
        }) 
    }).collect();

    for h in handles {
        h.join().unwrap();
    }
    
    println!("Total jobs completed: {}", job_status.lock().unwrap().jobs_completed);
    /*while status.jobs_completed < 10 {
        println!("Main thread waiting... ");
        thread::sleep(Duration::from_millis(500));
    }*/
}

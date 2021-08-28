extern crate job_scheduler;
use job_scheduler::{JobScheduler, Job};
use std::time::Duration;

fn main() {
    let mut sched = JobScheduler::new();

    sched.add(Job::new("1/5 * * * * *".parse().unwrap(), || {
        println!("I get executed every 5 seconds!");
    }));

    loop {
        sched.tick();

        std::thread::sleep(Duration::from_millis(500));
    }
}
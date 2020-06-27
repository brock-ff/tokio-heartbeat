use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::time::interval;

#[tokio::main]
async fn main() {
    // set up polling conditions
    let mut interval = interval(Duration::from_millis(1000));
    let is_complete = Arc::new(Mutex::new(false));
    /*
    Because we need `is_consumed` at the end to signal the task ending as
    well as for checking inside the thread whether we need to poll again, we
    clone it into a new variable `thread_complete`.

    `thread_complete` points to the same value as `is_complete`, but is
    treated as its own variable. This is necessary because using `move`
    consumes (takes ownership of) any variable in the new thread's scope.
     */
    let thread_complete = Arc::clone(&is_complete);

    // spawn new thread
    tokio::spawn(async move {
        let thread_complete = Arc::clone(&thread_complete);
        loop {
            if *thread_complete.lock().unwrap() == false {
                interval.tick().await;
                println!("thump");
            } else {
                println!("stopping heartbeat");
                break;
            }
        }
    });

    // do some work (simulate endpoint handler)
    println!("handling endpoint...");
    std::thread::sleep(Duration::from_millis(3000));

    // finish work, send exit condition to thread
    let mut is_complete = is_complete.lock().unwrap();
    *is_complete = true;
    println!("Done!");

    // wait to see heartbeat stop
    std::thread::sleep(Duration::from_millis(1000));
}

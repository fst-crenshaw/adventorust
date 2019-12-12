use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Arc;
use std::thread;

const KEY: &str = "yzbqklnj";

fn main() {
    // Make a vector to hold the children which are spawned.
    let mut children = vec![];

    // Create an arc for threads to communicate with each other.
    let val = Arc::new(AtomicU64::new(0)); // The next value to try.
    let success = Arc::new(AtomicBool::new(false)); // Whether one of the threads found an answer.

    // Spawn 10 threads to calculate MD5 hashes and find out whether the
    // digest begins with some number of zeroes.
    for _ in 0..10 {
        let val = Arc::clone(&val); // Clone the Arc, increasing the ref count.
        let success = Arc::clone(&success);

        children.push(thread::spawn(move || {
            loop {
                // Did some other thread get the answer?
                let s = success.load(Ordering::Relaxed);
                if s == true {
                    return;
                }

                // Construct the MD5 input.
                let v = val.fetch_add(1, Ordering::SeqCst);
                let mut try_this = String::from(KEY);
                try_this.push_str(&v.to_string());

                // Compute the digest.
                let digest = md5::compute(try_this.into_bytes());
                let hex_str = format!("{:x}", digest);

                if hex_str.starts_with("00000") {
                    // Set success to true, indicating to all the other
                    // threads to stop working.
                    success.fetch_or(true, Ordering::SeqCst);

                    println!("{:?}, {:?}", v, digest);
                    return;
                }
            }
        }));
    }

    for child in children {
        // Wait for the thread to finish. Returns a result.
        let _ = child.join();
    }
}

// Task:
//   Given a vector of 5 items controlled by a RwLock, two threads
//   attempt to randomly remove 3 things each from the vector. Do this
//   without a panic.

// Docs: https://doc.rust-lang.org/std/sync/struct.RwLock.html
use std::sync::RwLock;

fn main() {

    // Create a RwLocked vector of five elements.
    let lv = RwLock::new(vec![1, 2, 3, 4, 5]);
	
    println!("Vector contents: {:?}", lv);

    let val1 = lv.write().unwrap().pop();

    println!("Popped value: {:?}", val1);

    // This doesn't work!  Can't just read from a RwLock and expect to
    // alter the vector.
    // -- > let val2 = lv.read().unwrap().pop();
}

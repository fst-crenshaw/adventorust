/// Task:
///
///   Given a vector of 5 items controlled by a RwLock, two async
///   functions attempt to randomly remove 3 things each from the
///   vector. Do this without a panic.

// Docs:
//  https://doc.rust-lang.org/std/sync/struct.RwLock.html
use std::sync::RwLock;
use rand::Rng;

// Docs:
//  https://doc.rust-lang.org/std/sync/struct.Arc.html
use std::sync::Arc;

// `block_on` blocks the current thread until the provided future has
// run to completion. Other executors provide more complex behavior,
// like scheduling multiple futures onto the same thread.
// Docs:
//  https://rust-lang.github.io/async-book/01_getting_started/04_async_await_primer.html
use futures::executor::block_on;

/// Given a vector, pop some random value from the vector.  Returns
/// None if the vector is empty.
fn pop_random(v: &mut Vec<u32>) -> Option<u32> {

    if v.len() == 0 {
	return None;
    }

    // Instantiate a random number generator.
    let mut rng = rand::thread_rng();

    // Generate a random number between 0 and the length of the
    // vector.  Then remove than random item.
    let rando = rng.gen_range(0, v.len());
    Some(v.remove(rando as usize))
}

async fn writer(rw: Arc<RwLock<Vec<u32>>>, result: &mut Vec<u32>) {

    let writer = rw.write();
    
    // The result of write() is a Result because there's the
    // possibility that the lock has been poisoned by a panic
    // that occurred on another reader or writer.
    match writer {
	Err(_) => (),
	Ok(mut vect) => {
	    let p = pop_random(&mut vect);
	    match p {
		Some(el) => result.push(el),
		None => (),
	    }
	}
    }
}

async fn async_worker() {

    // Create a RwLocked vector of five elements.
    let lv = RwLock::new(vec![1, 2, 3, 4, 5]);

    // Create an Arc, a shared reference to the RwLock
    let shared_ref = Arc::new(lv);
    println!("Initial vector contents: {:?}", shared_ref);	

    let mut result1: Vec<u32> = Vec::with_capacity(3);
    let mut result2: Vec<u32> = Vec::with_capacity(3);  

    for _ in 0..3 {
	let sr1 = Arc::clone(&shared_ref);
	let sr2 = Arc::clone(&shared_ref);
	
	let f1 = writer(sr1, &mut result1);
	let f2 = writer(sr2, &mut result2);
	
	futures::join!(f1, f2);
    }
	
    println!("Resulting vector contents: {:?}", result1);
    println!("Resulting vector contents: {:?}", result2);
    println!("Initial vector is now:  {:?}", shared_ref);
}

// Katie mentions Lazy static.  "One of my favorite utility crates
// that have really scary code inside of them."  -- Rave review.
fn main()  {
    block_on(async_worker());
}

#[cfg(test)]
mod test {
    // Must include the functions you are testing.
    use super::pop_random;
    use std::collections::HashSet;
    
    #[test]
    fn empty_vector_works() {
	let mut empty_vec = vec![];
	assert_eq!(pop_random(&mut empty_vec), None);

	// The above two lines could also have been:
	//    assert_eq!(pop_random(vec![].as_mut()), None);
    }
    
    #[test]
    fn singleton_vector_works() {
	assert_eq!(pop_random(vec![1].as_mut()), Some(1));
    }

    #[test]
    fn fiver_vector_works() {
	let mut input_vector = vec![1, 2, 3, 4, 5];

	// Pop a random value from the vector.  The set of values in
	// the original input vector MUST be equal to the popped-from
	// vector plus the popped value.
	let p = pop_random(&mut input_vector).unwrap();
	let expected: HashSet<_> = vec![1, 2, 3, 4, 5].iter().cloned().collect();
	let result = std::iter::once(p).chain(input_vector).collect::<HashSet<u32>>();

	// The two sets must have no difference.  For example, if 3 was
	// randomly popped from the vector, then [1, 2, 3, 4, 5] and
	// [1, 2, 4, 5] + 3 are the same.
	assert!(expected.difference(&result).collect::<Vec<&u32>>().is_empty());
    }

    #[test]
    fn big_one_works() {
	let mut input_vector1: Vec<u32> = Vec::with_capacity(100);
	let mut input_vector2: Vec<u32> = Vec::with_capacity(100);

	for i in 0..100 as u32 {
	    input_vector1.push(i);
	    input_vector2.push(i);
	}
	
	let p1 = pop_random(&mut input_vector1).unwrap();
	let p2 = pop_random(&mut input_vector2).unwrap();
	
	// This test is brittle, but it's fairly unlikely that p1 and
	// p2 would be the same.
	assert_ne!(p1, p2);
    }
}

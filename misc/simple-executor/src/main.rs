use std::sync::mpsc::Receiver;
use std::sync::mpsc::SyncSender;
use std::sync::mpsc::sync_channel;
use futures::FutureExt;
use futures::{join, future};

use std::{
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll, Waker},
    thread,
    time::Duration,
};

use futures::task::waker_ref;
use futures::future::BoxFuture;
use futures::task::ArcWake;

//use futures_intrusive::timer::TimerFuture;

// Task executor that receives tasks off a channel and runs them.
struct Executor {
	// Receiver: a channel
	// treating the channel like a queue (channels have an internal queue)
	// reference counted task
	ready_queue: Receiver<Arc<Task>>,
}

// A future that can reschedule itself to be polled by an `Executor`.
struct Task {
	// In-progress future that should be pushed to completion.
	// ... of static and no output
	// Mutex makes it thread-safe
	// BoxFuture: we don't know the size of the thing at compile time (a
	// Box), with a future in it
	future: Mutex<Option<BoxFuture<'static, ()>>>,

	// Handle to place the task itself back onto the task queue.
	// other end of the channel; to reschedule itself, the task sends itself
	//down the channel'
	task_sender: SyncSender<Arc<Task>>,
}

// Spawner spawns new futures onto the task channel.
#[derive(Clone)]
struct Spawner {
	task_sender: SyncSender<Arc<Task>>,
}

// Spins up an executor and spawner that know how to talk to each other.
fn new_executor_and_spawner() -> (Executor, Spawner) {
	// Maximum number of tasks to allow queueing in the channel at once.
	const MAX_QUEUED_TASKS: usize = 10_000;

	let (task_sender, ready_queue) = sync_channel(MAX_QUEUED_TASKS);

	(Executor { ready_queue }, Spawner { task_sender })
}

// impl: implements functionality for some type
impl Spawner {
	fn spawn(&self, future: impl Future<Output = ()> + 'static + Send) {
		// boxed(): a pointer type for heap allocation
		let future = future.boxed();
		let task = Arc::new(Task {
			future: Mutex::new(Some(future)),
			task_sender: self.task_sender.clone(),
		});

		// send the task down the channel to say the task is ready to execute
		self.task_sender.send(task).expect("too many tasks queued");
	}
}

// This is where a reacter would fit in. This one is very dumb and always wakes
// the task up. But you might want to add logic here, such as epoll'ing to
// decide whether to wake up.
impl ArcWake for Task {
	// Self: keyword referring to the implementing type
	// here, that's Task, since Task implements ArcWake
	fn wake_by_ref(arc_self: &Arc<Self>) {
		let cloned = arc_self.clone();

		arc_self.task_sender.send(cloned).expect("too many tasks queued");
		//arc_self.task_sender.send(arc_self).expect("too many tasks queued");
	}
}

impl Executor {
	fn run(&self) {
		while let Ok(task) = self.ready_queue.recv() {
			// Take the future, and if it has not yet completed (is
			// still Some), poll it in an attempt to complete it.
			let mut future_slot = task.future.lock().unwrap();

			// .take(): taking ownership of the future
			if let Some(mut future) = future_slot.take() {
				// Create a `LocalWaker` from the task itself.
				let waker = waker_ref(&task);
				let context = &mut Context::from_waker(&*waker);

				println!("context: {:?}", context);

				if let Poll::Pending = future.as_mut().poll(context) {
					// We are not done processing the
					// future, so put it back in its task to
					// be run again in the future.
					*future_slot = Some(future);
				}
			}
		}
	}
}

pub struct TimerFuture {
	shared_state: Arc<Mutex<SharedState>>,
}

impl TimerFuture {
    /// Create a new `TimerFuture` which will complete after the provided
    /// timeout.
    pub fn new(duration: Duration) -> Self {
        let shared_state = Arc::new(Mutex::new(SharedState {
            completed: false,
            waker: None,
        }));

        // Spawn the new thread
        let thread_shared_state = shared_state.clone();
        thread::spawn(move || {
            thread::sleep(duration);
            let mut shared_state = thread_shared_state.lock().unwrap();
            // Signal that the timer has completed and wake up the last
            // task on which the future was polled, if one exists.
            shared_state.completed = true;
            if let Some(waker) = shared_state.waker.take() {
                waker.wake()
            }
        });

        TimerFuture { shared_state }
    }
}

impl Future for TimerFuture {
    type Output = ();
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // Look at the shared state to see if the timer has already completed.
        let mut shared_state = self.shared_state.lock().unwrap();
        if shared_state.completed {
            Poll::Ready(())
        } else {
            // Set waker so that the thread can wake up the current task
            // when the timer has completed, ensuring that the future is polled
            // again and sees that `completed = true`.
            //
            // It's tempting to do this once rather than repeatedly cloning
            // the waker each time. However, the `TimerFuture` can move between
            // tasks on the executor, which could cause a stale waker pointing
            // to the wrong task, preventing `TimerFuture` from waking up
            // correctly.
            //
            // N.B. it's possible to check for this using the `Waker::will_wake`
            // function, but we omit that here to keep things simple.
            shared_state.waker = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

// State shared by all of the tasks.
struct SharedState {
	// whether or not the sleep time has elapsed
	completed: bool,

	// waker for the task that the 'TimerFuture' is running on
	waker: Option<Waker>,
}

fn main() {
	let (executor1, spawner1) = new_executor_and_spawner();
	let (executor2, spawner2) = new_executor_and_spawner();

	// Spawn a task to print before and after waiting on a timer.
	spawner1.spawn(async {
		println!("howdy!");

		let timerA = TimerFuture::new(Duration::new(1, 0));
		let timerB = TimerFuture::new(Duration::new(4, 0));

		join!(timerA, timerB);

		println!("task 1: done!");
	});

	spawner2.spawn(async {
		println!("another task!");
		TimerFuture::new(Duration::new(1, 0)).await;
		println!("task 2: done!");
	});

	// Drop the spawner so that our executor knows it is finished and won't
	// receive more incoming tasks to run.
	// drop: disposes of a value
	drop(spawner1);
	drop(spawner2);

	// Run the executor until the task queue is empty. This will print
	// 'howdy!', pause, and then print 'done!'.
	executor1.run();
	executor2.run();
}

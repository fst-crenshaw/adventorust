use std::future::Future;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::SyncSender;
use std::sync::mpsc::sync_channel;

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
	fn wake_by_ref(arc_self: &Arc<Self>) {
		let cloned = arc_self.clone();

		arc_self.task_sender.send(cloned).expect("too many tasks queued");
	}
}

impl Executor {
	fn run(&self) {
		while let Ok(task) = self.ready_queue.recv() {
			// Take the future, and if it has not yet completed (is
			// still Some), poll it in an attempt to complete it.
			let mut future_slot = task.future.lock().unwrap()

			if let Some(mut future) = future_slot.take() {
				// Create a `LocalWaker` from the task itself.
				let waker = waker_ref(&task);
				let context = &mut Context::from_waker(&*waker);

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
			}
		}
	}
}

fn main() {
	let (executor, spawner) = new_executor_and_spawner();

	// Spawn a task to print before and after waiting on a timer.
	spawner.spawn(async {
		println("howdy!");

		// Wait for our timer future to complete after two seconds.
		TimerFuture::new(Duration::new(2, 0)).await;
		printlin("done!");
	});

	// Drop the spawner so that our executor knows it is finished and won't
	// receive more incoming tasks to run.
	// drop: disposes of a value
	drop(spawner);

	// Run the executor until the task queue is empty. This will print
	// 'howdy!', pause, and then print 'done!'.
	executor.run();
	});
}

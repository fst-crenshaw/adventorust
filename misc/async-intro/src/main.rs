extern crate async_std;
use async_std::{fs::File, io, prelude::*, task};

async fn read_file(path: &str) -> io::Result<String> {
    let mut file = File::open(path).await?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;
    Ok(contents)
}

fn main() {

    /* Spawn accepts a future and executes it on a task.  It returns
     * a JoinHandle.  The task does the bookkeeping for executing the
     * Future.  Is the future done?  Where is it in memory?  What is its
     * current state? */
    let reader_task = task::spawn(
        /* This is an async block, necessary to call an async function.
         * Async blocks return a value of type Future. */
        async { 
            let result = read_file("data.csv").await;
            match result {
                Ok(s) => println!("{}", s),
                Err(e) => println!("Error reading file: {:?}", e)
            }
        });
    println!("Started task!");
    task::block_on(reader_task);
    println!("Stopped task!");
}


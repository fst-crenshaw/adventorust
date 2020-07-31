use futures::stream::StreamExt;
use rand::Rng;
use std::{thread, time};
use tokio::net::TcpListener;
use tracing::warn;
use tracing::Level;
use tracing_subscriber;

fn do_work(id: u8) {
    // doing "work"
    let rand_num = time::Duration::from_millis(rand::thread_rng().gen_range(0, 10));
    thread::sleep(rand_num);

    warn!("sleep[{}]: {:?}", id, rand_num);
}

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:6142";
    let mut listener = TcpListener::bind(addr).await.unwrap();

    let subscriber = tracing_subscriber::fmt()
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("no global suscriber has been set");

    // Here we convert the `TcpListener` to a stream of incoming connections
    // with the `incoming` method.
    let server = async move {
        let mut incoming = listener.incoming();
        // .next() accepts the connection
        while let Some(conn) = incoming.next().await {
            println!("Incoming connection...");
            match conn {
                Err(e) => eprintln!("accept failed = {:?}", e),
                Ok(mut sock) => {
                    // Spawn the future that echos the data and returns how
                    // many bytes were copied as a concurrent task.
                    println!("Open socket...");
                    tokio::spawn(async move {
                        // Split up the reading and writing parts of the
                        // socket.
                        let (mut reader, mut writer) = sock.split();

                        println!("Writing to socket...");

                        // read from reader
                        let mut buf = [0 as u8; 10];
                        let n = reader.peek(&mut buf).await.unwrap();
                        println!("Read({}): {:?}", n, buf);
                        let id = buf[0];

                        do_work(id);

                        match tokio::io::copy(&mut reader, &mut writer).await {
                            Ok(amt) => {
                                do_work(id);

                                let s = tracing::warn_span!("finished writing");
                                let _g = s.enter();
                                let double = amt * 2;
                                let squared = amt * amt;
                                tracing::warn!(
                                    id = id,
                                    nwritten = amt,
                                    double = double,
                                    squared = squared,
                                    "computed values"
                                );

                                println!("wrote {} bytes", amt);
                            }
                            Err(err) => {
                                eprintln!("IO error {:?}", err);
                            }
                        }
                    });
                }
            }
        }
    };

    println!("Server running on localhost:6142");

    // Start the server and block this async fn until `server` spins down.
    server.await;
}

use futures::stream::StreamExt;
use tokio::net::TcpListener;
use tokio::net::TcpStream;
use tokio::prelude::*;
use tracing::{span, Level};

#[tokio::main]
async fn main() {
    // a builder for `FmtSubscriber`.
    let subscriber = tracing_subscriber::fmt()
        // all spans/events with a level higher than TRACE (e.g, debug, info, warn, etc.)
        // will be written to stdout.
        .with_max_level(Level::TRACE)
        // completes the builder
        .finish();
    // and sets the constructed `Subscriber` as the default.
    tracing::subscriber::set_global_default(subscriber).expect("no global subscriber has been set");

    let addr = "127.0.0.1:6142";
    let mut listener = TcpListener::bind(addr).await.unwrap();

    // Here we convert the `TcpListener` to a stream of incoming connections
    // with the `incoming` method.
    let server = async move {
        let mut incoming = listener.incoming();
        while let Some(conn) = incoming.next().await {
            println!("incoming connection...");
            match conn {
                Err(e) => eprintln!("accept failed = {:?}", e),
                Ok(mut sock) => {
                    // Spawn the future that echos the data and returns how
                    // many bytes were copied as a concurrent task.
                    println!("opened socket...");
                    tokio::spawn(async move {
                        // Split up the reading and writing parts of the
                        // socket.
                        let (mut reader, mut writer) = sock.split();

                        println!("writing to socket...");
                        match tokio::io::copy(&mut reader, &mut writer).await {
                            Ok(amt) => {
                                let warn_span = tracing::warn_span!("finished writing");
                                let _enter = warn_span.enter();
                                let double = amt * 2;
                                let squared = amt * amt;
                                tracing::warn!(
                                    nwritten = amt,
                                    double = double,
                                    squared = squared,
                                    "computed values"
                                );
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

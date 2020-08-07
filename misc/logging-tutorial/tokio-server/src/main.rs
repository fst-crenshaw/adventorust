use futures::stream::StreamExt;
use rand::Rng;
use std::{thread, time};
use tokio::net::TcpListener;
use tracing::{info, warn};
use tracing_subscriber::filter::EnvFilter;
use tracing_subscriber::layer::Layer;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{fmt, reload, Registry};

fn do_work(id: u8) {
    // doing "work"
    let rand_num = time::Duration::from_millis(rand::thread_rng().gen_range(0, 10));
    thread::sleep(rand_num);

    info!("sleep[{}]: {:?}", id, rand_num);
}

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:6142";
    let mut listener = TcpListener::bind(addr).await.unwrap();

    let filter = EnvFilter::from_default_env()
        .add_directive("error".parse().unwrap())
        .add_directive("[{{id=1}}]=trace".parse().unwrap());

    /* a subscriber is a collection of layers */
    /* a filter is a layer */
    /* a format is .. */
    let (layer, _handle) = reload::Layer::new(filter);
    let layer = layer.and_then(fmt::Layer::default());
    /* Flame graph subscriber ?! How do I get one? */

    /* a registry is where all the layers go */
    /* a subscriber belongs to a registry because a registry is a part of a subscriber */

    /* subscriber: register their interest and index all the call sites (warn!/trace!/etc)
     * a single actor makes decisions so its consistent:
     * at runtime the events you call with the callsites are relative to all the layers */

    /* bc we can compose multiple subscribers, the registry takes care of callsite stuff.
     * whenever an event happens it passes it along to each layer, so layers can 'subscribe' to
     * the events that are happening */
    let subscriber = layer.with_subscriber(Registry::default());
    subscriber.init();

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

                                let s = tracing::warn_span!("request", id = id, "finished writing");
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

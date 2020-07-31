use tokio::net::TcpStream;
use tokio::prelude::*;
use rayon::prelude::*;

async fn make_a_request() {
    let mut stream = TcpStream::connect("127.0.0.1:6142").await.unwrap();
    println!("created stream");
    let result = stream.write(b"hello world\n").await;
    println!("wrote to stream; success={:?}", result.is_ok());
}

#[tokio::main]
async fn main() {
    let (first, second, third, fourth) = tokio::join!(
        make_a_request(),
        make_a_request(),
        make_a_request(),
        make_a_request(),
    );
}

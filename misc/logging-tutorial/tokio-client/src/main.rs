use tokio::net::TcpStream;
use tokio::prelude::*;

async fn make_a_request(id: u8) {
    let mut stream = TcpStream::connect("127.0.0.1:6142").await.unwrap();
    println!("created stream");

    let result = stream.write(&[id]).await;
    println!("wrote to stream; success={:?}", result.is_ok());

    let mut buf = [0 as u8; 31];
    stream.read(&mut buf).await.unwrap();
    println!("Read: {:?}", buf);
}

#[tokio::main]
async fn main() {
    let (_first, _second, _third, _fourth) = tokio::join!(
        make_a_request(1),
        make_a_request(2),
        make_a_request(3),
        make_a_request(4),
    );
}

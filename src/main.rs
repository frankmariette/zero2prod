use std::net::TcpListener;
use zero2prod::run;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed ot bind random port");
    let port = listener.local_addr().unwrap().port();
    println!("Server running at 127.0.0.1:{}", port);
    run(listener)?.await
}
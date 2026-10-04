use std::net::TcpListener;

use z2p::startup::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // port 0 means random available port
    run(TcpListener::bind("127.0.0.1:0")?)?.await
}

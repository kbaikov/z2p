use std::net::TcpListener;

use z2p::configuration::get_configuration;
use z2p::startup::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = get_configuration().expect("Failed to read configuration.");

    // port 0 means random available port
    let address = format!("127.0.0.1:{}", configuration.application_port);

    run(TcpListener::bind(address)?)?.await
}

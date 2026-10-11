use std::net::TcpListener;

use crate::email_client::EmailClient;
use crate::routes::health_check::health_check;
use crate::routes::subscriptions::subscribe;

use actix_web::dev::Server;
use actix_web::{App, HttpServer, web};
use sqlx::PgPool;

use tracing_actix_web::TracingLogger;

/// # Errors
///
/// Will return `Err` if cannot bind a listener for accepting incoming connection requests.
pub fn run(
    listener: TcpListener,
    connection: PgPool,
    email_client: EmailClient,
) -> Result<Server, std::io::Error> {
    // wrap connection in Arc
    let db_pool = web::Data::new(connection);
    let email_client = web::Data::new(email_client);
    let server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(db_pool.clone())
            .app_data(email_client.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}

use std::net::TcpListener;

use crate::routes::health_check::health_check;
use crate::routes::subscriptions::subscribe;

use actix_web::dev::Server;
use actix_web::{App, HttpServer, web};
use sqlx::PgPool;

/// # Errors
///
/// Will return `Err` if cannot bind a listener for accepting incoming connection requests.
pub fn run(listener: TcpListener, connection: PgPool) -> Result<Server, std::io::Error> {
    // wrap connection in Arc
    let db_pool = web::Data::new(connection);
    let server = HttpServer::new(move || {
        App::new()
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}

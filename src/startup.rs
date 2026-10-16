use std::net::TcpListener;

use crate::configuration::Settings;
use crate::email_client::EmailClient;
use crate::routes::health_check::health_check;
use crate::routes::subscriptions::subscribe;
use crate::routes::subscriptions_confirm::confirm;
use sqlx::postgres::PgPoolOptions;

use actix_web::dev::Server;
use actix_web::{App, HttpServer, web};
use sqlx::PgPool;

use tracing_actix_web::TracingLogger;

pub struct Application {
    port: u16,
    server: Server,
}

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
            .route("/subscriptions/confirm", web::get().to(confirm))
            .app_data(db_pool.clone())
            .app_data(email_client.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}

impl Application {
    pub async fn build(configuration: Settings) -> Result<Self, std::io::Error> {
        let connection_pool = get_connection_pool(&configuration);

        let sender_email = configuration
            .email_client
            .sender()
            .expect("invalid email client address.");
        let timeout = configuration.email_client.timeout();
        let email_client = EmailClient::new(
            configuration.email_client.base_url,
            sender_email,
            configuration.email_client.authorization_token,
            timeout,
        );

        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );
        let listener = TcpListener::bind(address)?;
        let port = listener.local_addr().unwrap().port();
        let server = run(listener, connection_pool, email_client)?;
        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

pub fn get_connection_pool(configuration: &Settings) -> sqlx::Pool<sqlx::Postgres> {
    PgPoolOptions::new().connect_lazy_with(configuration.database.with_db())
}

use actix_web::{HttpResponse, web};

#[derive(serde::Deserialize)]
pub struct Parameters {
    subscription_token: String,
}

/// It is enough to add a function parameter of type web::Query<Parameter> to confirm
/// to instruct actix-web to only call the handler if the extraction was successful.
/// If the extraction failed a 400 Bad Request is automatically returned to the caller.
#[tracing::instrument(name = "Confirm a pending subscriber", skip(_parameters))]
pub async fn confirm(_parameters: web::Query<Parameters>) -> HttpResponse {
    HttpResponse::Ok().finish()
}

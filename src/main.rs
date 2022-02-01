use serde::Deserialize;
use tower_http::trace::TraceLayer;

use axum::{
    extract::Extension, http::StatusCode, response::IntoResponse, routing::post, AddExtensionLayer,
    Json, Router,
};

use lettre::{
    transport::smtp::authentication::Credentials, AsyncSmtpTransport, AsyncTransport, Message,
    Tokio1Executor,
};

use dotenv::dotenv;
use serde_json::json;
use std::sync::Arc;
use tracing::instrument;

#[derive(Deserialize, Debug)]
struct ContactRequest {
    subject: String,
    email: String,
    body: String,
}

#[derive(Debug)]
struct State {
    smtp_transport: AsyncSmtpTransport<Tokio1Executor>,
    email_from: String,
    email_to: String,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    std::env::set_var("RUST_LOG", "tower_http=debug");
    tracing_subscriber::fmt::init();

    let number_of_yaks = 3;
    tracing::debug!(number_of_yaks, "preparing to shave yaks");

    let smtp_username = dotenv::var(&"SMTP_USERNAME").unwrap();
    let smtp_password = dotenv::var(&"SMTP_PASSWORD").unwrap();
    let smtp_server = dotenv::var(&"SMTP_SERVER").unwrap();
    let smtp_port = dotenv::var(&"SMTP_PORT").unwrap().parse::<u16>().unwrap();

    let email_from = dotenv::var(&"EMAIL_FROM").unwrap();
    let email_to = dotenv::var(&"EMAIL_TO").unwrap();

    let smtp_credentials = Credentials::new(smtp_username, smtp_password);

    let smtp_transport: AsyncSmtpTransport<Tokio1Executor> =
        AsyncSmtpTransport::<Tokio1Executor>::relay(&smtp_server)
            .unwrap()
            .credentials(smtp_credentials.clone())
            .port(smtp_port)
            .build();

    let state = Arc::new(State {
        smtp_transport,
        email_from,
        email_to,
    });

    let listen_addr = dotenv::var(&"LISTEN_ADDR").unwrap();

    println!("listening on {}", listen_addr);

    let app = Router::new()
        .route("/api/contact_request", post(contact_request))
        .layer(TraceLayer::new_for_http())
        .layer(AddExtensionLayer::new(state));

    axum::Server::bind(&listen_addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[instrument]
async fn contact_request(
    Extension(state): Extension<Arc<State>>,
    Json(contact_request): Json<ContactRequest>,
) -> impl IntoResponse {
    let email = Message::builder()
        .from(state.email_from.parse().unwrap())
        .reply_to(contact_request.email.parse().unwrap())
        .to(state.email_to.parse().unwrap())
        .subject(contact_request.subject)
        .body(contact_request.body)
        .unwrap();

    match state.smtp_transport.send(email).await {
        Ok(i) => {
            tracing::info!("sending email: {:#?}", i);
            (StatusCode::CREATED, Json(json!({"status": "ok"})))
        }
        Err(e) => {
            tracing::error!("err sending email: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": "ko"})),
            )
        }
    }
}

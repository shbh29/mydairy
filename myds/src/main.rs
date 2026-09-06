use axum::{
    extract::Json,
    http::StatusCode,
    routing::post,
    Router,
};
use crate::journal_error::JournalError;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct JournalEntry {
    text: String,
}

mod journal_error;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/journal", post(journal));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn journal(Json(je): Json<JournalEntry> ) -> Result<(StatusCode, String), JournalError> {
    /*Ok((
        StatusCode::CREATED,
        format!("Received Journal Entry, \n{}!\n", je.text)
    ))*/
    Err(JournalError::SomethingWentWrong)
}

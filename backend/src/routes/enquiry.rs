use axum::{Json, Router, extract::State, http::StatusCode, response::IntoResponse, routing::post};
use common::EnquiryRequest;

use crate::{AppState, errors::AppError};

const MAX_FIELD_LEN: usize = 256;
const MAX_MESSAGE_LEN: usize = 4096;

pub fn router() -> Router<AppState> {
    Router::new().route("/enquiry", post(submit))
}

async fn submit(
    State(state): State<AppState>,
    Json(req): Json<EnquiryRequest>,
) -> Result<impl IntoResponse, AppError> {
    // Honeypot tripped: claim success without storing anything.
    if !req.website.is_empty() {
        return Ok(StatusCode::ACCEPTED);
    }

    let name = req.name.trim();
    let email = req.email.trim();
    let message = req.message.trim();

    if name.is_empty() || name.len() > MAX_FIELD_LEN {
        return Err(AppError::BadRequest("invalid name".into()));
    }
    if email.is_empty() || email.len() > MAX_FIELD_LEN || !email.contains('@') {
        return Err(AppError::BadRequest("invalid email".into()));
    }
    if message.is_empty() || message.len() > MAX_MESSAGE_LEN {
        return Err(AppError::BadRequest("invalid message".into()));
    }

    // TODO(zero-trust): encrypt PII columns (name, email, message) at the
    //   application layer before storing — key from env only, never on disk.
    // TODO(spam): add IP-based rate limiting (e.g. tower-governor).
    // TODO(gdpr): scheduled purge of enquiries older than the retention period.
    // TODO: notify the tutor by email on new enquiries.
    sqlx::query("INSERT INTO enquiries (id, name, email, message) VALUES (?, ?, ?, ?)")
        .bind(uuid::Uuid::new_v4().to_string())
        .bind(name)
        .bind(email)
        .bind(message)
        .execute(&state.pool)
        .await?;

    Ok(StatusCode::ACCEPTED)
}

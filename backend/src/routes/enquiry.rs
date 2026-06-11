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

#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use common::EnquiryRequest;
    use sqlx::SqlitePool;
    use sqlx::sqlite::SqlitePoolOptions;
    use tower::ServiceExt;

    use crate::AppState;

    async fn test_app() -> (axum::Router, SqlitePool) {
        // One connection only: every `sqlite::memory:` connection is a
        // separate database, so a larger pool would lose the migrations.
        let pool = SqlitePoolOptions::new()
            .max_connections(1)
            .connect("sqlite::memory:")
            .await
            .unwrap();
        sqlx::migrate!("./migrations").run(&pool).await.unwrap();

        let state = AppState {
            pool: pool.clone(),
            leptos_options: leptos::prelude::LeptosOptions::builder()
                .output_name("test")
                .build(),
        };
        (super::router().with_state(state), pool)
    }

    fn post_json(body: String) -> Request<Body> {
        Request::builder()
            .method("POST")
            .uri("/enquiry")
            .header("content-type", "application/json")
            .body(Body::from(body))
            .unwrap()
    }

    fn enquiry(name: &str, email: &str, message: &str, website: &str) -> String {
        serde_json::to_string(&EnquiryRequest {
            name: name.into(),
            email: email.into(),
            message: message.into(),
            website: website.into(),
        })
        .unwrap()
    }

    async fn stored(pool: &SqlitePool) -> Vec<(String, String, String)> {
        sqlx::query_as("SELECT name, email, message FROM enquiries")
            .fetch_all(pool)
            .await
            .unwrap()
    }

    #[tokio::test]
    async fn valid_enquiry_is_accepted_and_stored_trimmed() {
        let (app, pool) = test_app().await;

        let response = app
            .oneshot(post_json(enquiry(
                "  Jane  ",
                " jane@example.com ",
                " Hello there ",
                "",
            )))
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::ACCEPTED);
        assert_eq!(
            stored(&pool).await,
            vec![(
                "Jane".to_string(),
                "jane@example.com".to_string(),
                "Hello there".to_string()
            )]
        );
    }

    #[tokio::test]
    async fn honeypot_claims_success_but_stores_nothing() {
        let (app, pool) = test_app().await;

        let response = app
            .oneshot(post_json(enquiry(
                "Bot",
                "bot@spam.example",
                "Buy things",
                "https://spam.example",
            )))
            .await
            .unwrap();

        // Bots must not be able to tell they were caught.
        assert_eq!(response.status(), StatusCode::ACCEPTED);
        assert!(stored(&pool).await.is_empty());
    }

    #[tokio::test]
    async fn invalid_fields_are_rejected_and_not_stored() {
        let cases = [
            enquiry("", "a@b.example", "hi", ""),
            enquiry("   ", "a@b.example", "hi", ""),
            enquiry(&"x".repeat(257), "a@b.example", "hi", ""),
            enquiry("Jane", "", "hi", ""),
            enquiry("Jane", "not-an-email", "hi", ""),
            enquiry("Jane", &format!("{}@b.example", "x".repeat(257)), "hi", ""),
            enquiry("Jane", "a@b.example", "", ""),
            enquiry("Jane", "a@b.example", &"x".repeat(4097), ""),
        ];

        for (i, body) in cases.into_iter().enumerate() {
            let (app, pool) = test_app().await;
            let response = app.oneshot(post_json(body)).await.unwrap();
            assert_eq!(
                response.status(),
                StatusCode::BAD_REQUEST,
                "case {i} should be rejected"
            );
            assert!(stored(&pool).await.is_empty(), "case {i} must not store");
        }
    }

    #[tokio::test]
    async fn missing_honeypot_field_is_treated_as_human() {
        let (app, pool) = test_app().await;

        // Older cached form bundles may omit `website` entirely; serde must
        // default it to empty rather than reject the request.
        let response = app
            .oneshot(post_json(
                r#"{"name":"Jane","email":"jane@example.com","message":"Hello"}"#.to_string(),
            ))
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::ACCEPTED);
        assert_eq!(stored(&pool).await.len(), 1);
    }
}

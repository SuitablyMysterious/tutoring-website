use crate::AppState;
use axum::{Router, routing::get};

pub fn router() -> Router<AppState> {
    Router::new().route("/health", get(|| async { "ok" }))
}

#[cfg(test)]
mod tests {
    use axum::body::Body;
    use axum::http::{Request, StatusCode};
    use tower::ServiceExt;

    use crate::AppState;

    #[tokio::test]
    async fn health_returns_ok() {
        let state = AppState {
            pool: sqlx::SqlitePool::connect_lazy("sqlite::memory:").unwrap(),
            leptos_options: leptos::prelude::LeptosOptions::builder()
                .output_name("test")
                .build(),
        };
        let app = super::router().with_state(state);

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/health")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
        let body = axum::body::to_bytes(response.into_body(), 1024)
            .await
            .unwrap();
        assert_eq!(&body[..], b"ok");
    }
}

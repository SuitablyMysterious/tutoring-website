use crate::AppState;
use axum::Router;

mod enquiry;
mod health;

pub fn router() -> Router<AppState> {
    Router::new()
        .merge(enquiry::router())
        .merge(health::router())
}

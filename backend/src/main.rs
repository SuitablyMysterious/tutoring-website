use axum::Router;
use frontend::app::{App, shell};
use leptos::prelude::*;
use leptos_axum::{LeptosRoutes, generate_route_list};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

mod db;
mod errors;
mod routes;

#[derive(Clone, axum::extract::FromRef)]
pub struct AppState {
    pub pool: sqlx::SqlitePool,
    pub leptos_options: LeptosOptions,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    // NOTE(gdpr): request traces include client IPs, which are personal data.
    // Keep log retention short and documented.
    tracing_subscriber::registry()
        .with(
            EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "backend=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Relative to the working directory — `cargo leptos watch` runs the
    // binary from the workspace root. `mode=rwc` lets SQLite create the file.
    let database_url =
        std::env::var("DATABASE_URL").unwrap_or_else(|_| "sqlite://db.sqlite?mode=rwc".to_string());
    let pool = db::init_pool(&database_url)
        .await
        .expect("failed to initialise database");

    // Reads LEPTOS_* env vars: set automatically by `cargo leptos watch`;
    // must be provided by the service unit in production (site root, addr).
    let conf = get_configuration(None).expect("failed to read leptos configuration");
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(App);

    let state = AppState {
        pool,
        leptos_options: leptos_options.clone(),
    };

    let app = Router::new()
        .nest("/api", routes::router())
        .leptos_routes(&state, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler::<AppState, _>(shell))
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("failed to bind port");

    tracing::info!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.expect("server error");
}

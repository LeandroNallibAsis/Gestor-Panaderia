pub mod auth;
pub mod health;
pub mod stores;
pub mod users;

use axum::{
    routing::{get, post},
    Router,
};
use tower_http::{
    cors::{Any, CorsLayer},
    trace::TraceLayer,
};

use crate::config::AppState;

/// Construye el enrutador principal de Axum con middlewares CORS y tracing.
pub fn build_router(state: AppState) -> Router {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    Router::new()
        .route("/health", get(health::health_check))
        .route("/api/v1/auth/login", post(auth::login))
        .route("/api/v1/auth/refresh", post(auth::refresh))
        .route("/api/v1/auth/me", get(auth::me))
        .route("/api/v1/users", get(users::list_users).post(users::create_user))
        .route(
            "/api/v1/users/:id",
            get(users::get_user).patch(users::update_user),
        )
        .route(
            "/api/v1/stores",
            get(stores::list_stores).post(stores::create_store),
        )
        .route(
            "/api/v1/stores/:id",
            get(stores::get_store).patch(stores::update_store),
        )
        .route("/api/v1/stores/:id/history", get(stores::get_store_history))
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}

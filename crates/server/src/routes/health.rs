use axum::{extract::State, http::StatusCode, Json};
use chrono::Utc;
use panaderia_core::HealthResponse;

use crate::config::AppState;

/// `GET /health` - Verifica conectividad con PostgreSQL y devuelve estado del servicio.
pub async fn health_check(State(state): State<AppState>) -> (StatusCode, Json<HealthResponse>) {
    let db_ok = sqlx::query_scalar::<_, i32>("SELECT 1")
        .fetch_one(&state.db)
        .await
        .is_ok();

    let (status_code, status_text, db_text) = if db_ok {
        (StatusCode::OK, "ok", "up")
    } else {
        (StatusCode::SERVICE_UNAVAILABLE, "degraded", "down")
    };

    let body = HealthResponse {
        status: status_text.to_string(),
        database: db_text.to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        timestamp: Utc::now(),
    };

    (status_code, Json(body))
}

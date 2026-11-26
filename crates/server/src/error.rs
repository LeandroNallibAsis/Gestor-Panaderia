use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use panaderia_core::{ApiErrorBody, ApiErrorDetail, FieldViolation, ValidationErrors};
use thiserror::Error;

/// Error unificado de la capa HTTP que serializa respuestas JSON estructuradas.
#[derive(Debug, Error)]
pub enum AppError {
    #[error("Petición inválida: {0}")]
    BadRequest(String),

    #[error("Error de validación")]
    Validation(#[from] ValidationErrors),

    #[error("No autenticado: {0}")]
    Unauthorized(String),

    #[error("Acceso denegado: {0}")]
    Forbidden(String),

    #[error("Recurso no encontrado: {0}")]
    NotFound(String),

    #[error("Conflicto de datos: {0}")]
    Conflict(String),

    #[error("Error de base de datos: {0}")]
    Database(#[from] sqlx::Error),

    #[error("Servicio no disponible: {0}")]
    ServiceUnavailable(String),

    #[error("Error interno: {0}")]
    Internal(String),
}

impl AppError {
    fn into_parts(self) -> (StatusCode, &'static str, String, Option<Vec<FieldViolation>>) {
        match self {
            Self::BadRequest(msg) => (StatusCode::BAD_REQUEST, "bad_request", msg, None),
            Self::Validation(err) => (
                StatusCode::UNPROCESSABLE_ENTITY,
                "validation_error",
                err.message,
                Some(err.violations),
            ),
            Self::Unauthorized(msg) => (StatusCode::UNAUTHORIZED, "unauthorized", msg, None),
            Self::Forbidden(msg) => (StatusCode::FORBIDDEN, "forbidden", msg, None),
            Self::NotFound(msg) => (StatusCode::NOT_FOUND, "not_found", msg, None),
            Self::Conflict(msg) => (StatusCode::CONFLICT, "conflict", msg, None),
            Self::ServiceUnavailable(msg) => (
                StatusCode::SERVICE_UNAVAILABLE,
                "service_unavailable",
                msg,
                None,
            ),
            Self::Database(err) => {
                if let sqlx::Error::Database(db_err) = &err {
                    if db_err.code().as_deref() == Some("23505") {
                        return (
                            StatusCode::CONFLICT,
                            "conflict",
                            "Ya existe un registro con ese identificador único".to_string(),
                            None,
                        );
                    }
                    if db_err.code().as_deref() == Some("23503") {
                        return (
                            StatusCode::BAD_REQUEST,
                            "foreign_key_violation",
                            "Referencia a recurso inexistente".to_string(),
                            None,
                        );
                    }
                }
                if matches!(err, sqlx::Error::RowNotFound) {
                    return (
                        StatusCode::NOT_FOUND,
                        "not_found",
                        "Recurso no encontrado".to_string(),
                        None,
                    );
                }
                tracing::error!(error = %err, "Error de base de datos no controlado");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "database_error",
                    "Error interno de base de datos".to_string(),
                    None,
                )
            }
            Self::Internal(msg) => {
                tracing::error!(detail = %msg, "Error interno del servidor");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "internal_error",
                    "Error interno del servidor".to_string(),
                    None,
                )
            }
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, code, message, details) = self.into_parts();
        let body = ApiErrorBody {
            error: ApiErrorDetail {
                code: code.to_string(),
                message,
                details,
            },
        };
        (status, Json(body)).into_response()
    }
}

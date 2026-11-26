use axum::{extract::State, Json};
use panaderia_core::{
    AuthUser, LoginRequest, LoginResponse, RefreshRequest, Role, TokenKind, UserStatus,
};
use sqlx::Row;

use crate::{
    auth::{create_token_pair, decode_token, verify_password, CurrentUser},
    config::AppState,
    error::AppError,
};

fn row_to_auth_user(row: &sqlx::postgres::PgRow) -> Result<AuthUser, AppError> {
    let role_name: String = row.get("role_name");
    let role = role_name
        .parse::<Role>()
        .map_err(|e| AppError::Internal(format!("Rol inválido en DB: {e}")))?;

    let status_str: String = row.get("status");
    let status = status_str.parse::<UserStatus>().unwrap_or(UserStatus::Activo);

    Ok(AuthUser {
        id: row.get("id"),
        role_id: row.get("role_id"),
        role,
        name: row.get("name"),
        email: row.get("email"),
        phone: row.get("phone"),
        status,
        is_active: row.get("is_active"),
    })
}

/// `POST /api/v1/auth/login`
pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, AppError> {
    payload.validate()?;
    let email = payload.email.trim().to_ascii_lowercase();

    let row = sqlx::query(
        r#"
        SELECT
            u.id,
            u.role_id,
            r.name AS role_name,
            u.name,
            u.email,
            u.phone,
            u.password_hash,
            u.status,
            u.is_active
        FROM users u
        INNER JOIN roles r ON r.id = u.role_id
        WHERE LOWER(u.email) = $1
        "#,
    )
    .bind(&email)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::Unauthorized("Credenciales inválidas".to_string()))?;

    let password_hash: String = row.get("password_hash");
    if !verify_password(&payload.password, &password_hash)? {
        return Err(AppError::Unauthorized("Credenciales inválidas".to_string()));
    }

    let user = row_to_auth_user(&row)?;
    if !user.is_active {
        return Err(AppError::Forbidden(
            "La cuenta de usuario está desactivada".to_string(),
        ));
    }

    sqlx::query("UPDATE users SET last_activity_at = NOW(), updated_at = NOW() WHERE id = $1")
        .bind(user.id)
        .execute(&state.db)
        .await?;

    let tokens = create_token_pair(
        user.id,
        user.role,
        &state.jwt_secret,
        state.access_ttl_seconds,
        state.refresh_ttl_seconds,
    )?;

    Ok(Json(LoginResponse {
        access_token: tokens.access_token,
        refresh_token: tokens.refresh_token,
        token_type: "Bearer".to_string(),
        expires_in: tokens.expires_in,
        user,
    }))
}

/// `POST /api/v1/auth/refresh`
pub async fn refresh(
    State(state): State<AppState>,
    Json(payload): Json<RefreshRequest>,
) -> Result<Json<LoginResponse>, AppError> {
    let token = payload.refresh_token.trim();
    if token.is_empty() {
        return Err(AppError::BadRequest(
            "El campo refresh_token es obligatorio".to_string(),
        ));
    }

    let claims = decode_token(token, &state.jwt_secret, TokenKind::Refresh)?;

    let row = sqlx::query(
        r#"
        SELECT
            u.id,
            u.role_id,
            r.name AS role_name,
            u.name,
            u.email,
            u.phone,
            u.status,
            u.is_active
        FROM users u
        INNER JOIN roles r ON r.id = u.role_id
        WHERE u.id = $1
        "#,
    )
    .bind(claims.sub)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::Unauthorized("Usuario no encontrado".to_string()))?;

    let user = row_to_auth_user(&row)?;
    if !user.is_active {
        return Err(AppError::Forbidden(
            "La cuenta de usuario está desactivada".to_string(),
        ));
    }

    let tokens = create_token_pair(
        user.id,
        user.role,
        &state.jwt_secret,
        state.access_ttl_seconds,
        state.refresh_ttl_seconds,
    )?;

    Ok(Json(LoginResponse {
        access_token: tokens.access_token,
        refresh_token: tokens.refresh_token,
        token_type: "Bearer".to_string(),
        expires_in: tokens.expires_in,
        user,
    }))
}

/// `GET /api/v1/auth/me`
pub async fn me(
    State(state): State<AppState>,
    current_user: CurrentUser,
) -> Result<Json<AuthUser>, AppError> {
    let row = sqlx::query(
        r#"
        SELECT
            u.id,
            u.role_id,
            r.name AS role_name,
            u.name,
            u.email,
            u.phone,
            u.status,
            u.is_active
        FROM users u
        INNER JOIN roles r ON r.id = u.role_id
        WHERE u.id = $1
        "#,
    )
    .bind(current_user.id)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::NotFound("Usuario no encontrado".to_string()))?;

    let user = row_to_auth_user(&row)?;
    Ok(Json(user))
}

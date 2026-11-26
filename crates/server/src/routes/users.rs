use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use panaderia_core::{CreateUserRequest, Role, UpdateUserRequest, UserPublic, UserStatus};
use sqlx::Row;
use uuid::Uuid;

use crate::{
    auth::{hash_password, require_jefe, CurrentUser},
    config::AppState,
    error::AppError,
};

fn row_to_user_public(row: &sqlx::postgres::PgRow) -> Result<UserPublic, AppError> {
    let role_name: String = row.get("role_name");
    let role = role_name
        .parse::<Role>()
        .map_err(|e| AppError::Internal(format!("Rol inválido en DB: {e}")))?;

    let status_str: String = row.get("status");
    let status = status_str.parse::<UserStatus>().unwrap_or(UserStatus::Activo);

    Ok(UserPublic {
        id: row.get("id"),
        role,
        name: row.get("name"),
        email: row.get("email"),
        phone: row.get("phone"),
        assigned_vehicle: row.get("assigned_vehicle"),
        status,
        shift: row.get("shift"),
        is_active: row.get("is_active"),
        last_activity_at: row.get("last_activity_at"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    })
}

async fn fetch_user_public_by_id(state: &AppState, user_id: Uuid) -> Result<UserPublic, AppError> {
    let row = sqlx::query(
        r#"
        SELECT
            u.id,
            r.name AS role_name,
            u.name,
            u.email,
            u.phone,
            u.assigned_vehicle,
            u.status,
            u.shift,
            u.is_active,
            u.last_activity_at,
            u.created_at,
            u.updated_at
        FROM users u
        INNER JOIN roles r ON r.id = u.role_id
        WHERE u.id = $1
        "#,
    )
    .bind(user_id)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("Usuario {user_id} no encontrado")))?;

    row_to_user_public(&row)
}

/// `GET /api/v1/users`
pub async fn list_users(
    State(state): State<AppState>,
    _current_user: CurrentUser,
) -> Result<Json<Vec<UserPublic>>, AppError> {
    let rows = sqlx::query(
        r#"
        SELECT
            u.id,
            r.name AS role_name,
            u.name,
            u.email,
            u.phone,
            u.assigned_vehicle,
            u.status,
            u.shift,
            u.is_active,
            u.last_activity_at,
            u.created_at,
            u.updated_at
        FROM users u
        INNER JOIN roles r ON r.id = u.role_id
        ORDER BY u.created_at DESC
        "#,
    )
    .fetch_all(&state.db)
    .await?;

    let users = rows
        .iter()
        .map(row_to_user_public)
        .collect::<Result<Vec<_>, _>>()?;

    Ok(Json(users))
}

/// `POST /api/v1/users` (solo `jefe`)
pub async fn create_user(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Json(payload): Json<CreateUserRequest>,
) -> Result<(StatusCode, Json<UserPublic>), AppError> {
    require_jefe(&current_user)?;
    payload.validate()?;

    let role_id: Uuid = sqlx::query_scalar("SELECT id FROM roles WHERE name = $1")
        .bind(payload.role.as_str())
        .fetch_optional(&state.db)
        .await?
        .ok_or_else(|| {
            AppError::Internal(format!(
                "No existe el rol '{}' en la tabla roles",
                payload.role.as_str()
            ))
        })?;

    let password_hash = hash_password(&payload.password)?;
    let email = payload.email.trim().to_ascii_lowercase();
    let shift = payload
        .shift
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .unwrap_or("Mañana")
        .to_string();
    let status = payload.status.unwrap_or(UserStatus::Activo).as_str();

    let new_id: Uuid = sqlx::query_scalar(
        r#"
        INSERT INTO users (
            role_id,
            name,
            email,
            phone,
            password_hash,
            assigned_vehicle,
            status,
            shift
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        RETURNING id
        "#,
    )
    .bind(role_id)
    .bind(payload.name.trim())
    .bind(&email)
    .bind(payload.phone.trim())
    .bind(&password_hash)
    .bind(payload.assigned_vehicle.as_deref().map(str::trim))
    .bind(status)
    .bind(&shift)
    .fetch_one(&state.db)
    .await?;

    let user = fetch_user_public_by_id(&state, new_id).await?;
    Ok((StatusCode::CREATED, Json(user)))
}

/// `GET /api/v1/users/:id`
pub async fn get_user(
    State(state): State<AppState>,
    _current_user: CurrentUser,
    Path(id): Path<Uuid>,
) -> Result<Json<UserPublic>, AppError> {
    let user = fetch_user_public_by_id(&state, id).await?;
    Ok(Json(user))
}

/// `PATCH /api/v1/users/:id`
pub async fn update_user(
    State(state): State<AppState>,
    current_user: CurrentUser,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateUserRequest>,
) -> Result<Json<UserPublic>, AppError> {
    if !current_user.role.is_jefe() && current_user.id != id {
        return Err(AppError::Forbidden(
            "Solo el rol 'jefe' puede modificar otros usuarios".to_string(),
        ));
    }
    if !current_user.role.is_jefe() && (payload.role.is_some() || payload.is_active.is_some()) {
        return Err(AppError::Forbidden(
            "Solo el rol 'jefe' puede cambiar el rol o el estado activo de un usuario".to_string(),
        ));
    }

    payload.validate()?;

    // Verificar existencia antes de actualizar.
    let _existing = fetch_user_public_by_id(&state, id).await?;

    let role_id = if let Some(role) = payload.role {
        let found: Uuid = sqlx::query_scalar("SELECT id FROM roles WHERE name = $1")
            .bind(role.as_str())
            .fetch_optional(&state.db)
            .await?
            .ok_or_else(|| AppError::Internal(format!("Rol '{}' no encontrado", role.as_str())))?;
        Some(found)
    } else {
        None
    };

    let password_hash = if let Some(password) = &payload.password {
        Some(hash_password(password)?)
    } else {
        None
    };

    let email = payload
        .email
        .as_deref()
        .map(|s| s.trim().to_ascii_lowercase());
    let name = payload.name.as_deref().map(str::trim).map(str::to_string);
    let phone = payload.phone.as_deref().map(str::trim).map(str::to_string);
    let assigned_vehicle = payload
        .assigned_vehicle
        .as_deref()
        .map(str::trim)
        .map(str::to_string);
    let shift = payload.shift.as_deref().map(str::trim).map(str::to_string);
    let status = payload.status.map(|s| s.as_str().to_string());

    sqlx::query(
        r#"
        UPDATE users
        SET
            role_id = COALESCE($2, role_id),
            name = COALESCE($3, name),
            email = COALESCE($4, email),
            phone = COALESCE($5, phone),
            password_hash = COALESCE($6, password_hash),
            assigned_vehicle = COALESCE($7, assigned_vehicle),
            status = COALESCE($8, status),
            shift = COALESCE($9, shift),
            is_active = COALESCE($10, is_active),
            updated_at = NOW()
        WHERE id = $1
        "#,
    )
    .bind(id)
    .bind(role_id)
    .bind(name)
    .bind(email)
    .bind(phone)
    .bind(password_hash)
    .bind(assigned_vehicle)
    .bind(status)
    .bind(shift)
    .bind(payload.is_active)
    .execute(&state.db)
    .await?;

    let updated = fetch_user_public_by_id(&state, id).await?;
    Ok(Json(updated))
}

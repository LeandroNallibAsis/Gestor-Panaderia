use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::domain::{OrderStatus, Role, Transaction, UserStatus};

/// Tipo de token JWT emitido por el servidor.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TokenKind {
    Access,
    Refresh,
}

/// Claims incluidos dentro de los JWT de acceso y refresco.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtClaims {
    pub sub: Uuid,
    pub role: Role,
    pub token_type: TokenKind,
    pub exp: usize,
    pub iat: usize,
    pub jti: Uuid,
}

/// Credenciales enviadas en `POST /api/v1/auth/login`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

/// Resumen público del usuario autenticado incluido en login/me.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthUser {
    pub id: Uuid,
    pub role_id: Uuid,
    pub role: Role,
    pub name: String,
    pub email: String,
    pub phone: String,
    pub status: UserStatus,
    pub is_active: bool,
}

/// Respuesta de autenticación con par de tokens JWT.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoginResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
    pub expires_in: i64,
    pub user: AuthUser,
}

/// Payload enviado en `POST /api/v1/auth/refresh`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefreshRequest {
    pub refresh_token: String,
}

/// Payload para crear un usuario en `POST /api/v1/users`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub name: String,
    pub email: String,
    pub phone: String,
    pub password: String,
    pub role: Role,
    pub assigned_vehicle: Option<String>,
    pub shift: Option<String>,
    pub status: Option<UserStatus>,
}

/// Payload parcial para actualizar un usuario en `PATCH /api/v1/users/:id`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateUserRequest {
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub password: Option<String>,
    pub role: Option<Role>,
    pub assigned_vehicle: Option<String>,
    pub shift: Option<String>,
    pub status: Option<UserStatus>,
    pub is_active: Option<bool>,
}

/// Representación pública de un usuario sin campos sensibles (`password_hash`).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPublic {
    pub id: Uuid,
    pub role: Role,
    pub name: String,
    pub email: String,
    pub phone: String,
    pub assigned_vehicle: Option<String>,
    pub status: UserStatus,
    pub shift: String,
    pub is_active: bool,
    pub last_activity_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Payload para crear una tienda en `POST /api/v1/stores`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateStoreRequest {
    pub name: String,
    pub zone: String,
    pub address: String,
    pub contact_name: String,
    pub phone: String,
    pub delivery_window: String,
    pub credit_limit: Option<f64>,
    pub delivery_notes: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub active: Option<bool>,
}

/// Payload parcial para actualizar una tienda en `PATCH /api/v1/stores/:id`.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateStoreRequest {
    pub name: Option<String>,
    pub zone: Option<String>,
    pub address: Option<String>,
    pub contact_name: Option<String>,
    pub phone: Option<String>,
    pub delivery_window: Option<String>,
    pub pending_balance: Option<f64>,
    pub credit_limit: Option<f64>,
    pub delivery_notes: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub active: Option<bool>,
}

/// DTO de respuesta de una tienda con coordenadas geográficas extraídas.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoreDto {
    pub id: Uuid,
    pub name: String,
    pub zone: String,
    pub address: String,
    pub contact_name: String,
    pub phone: String,
    pub delivery_window: String,
    pub pending_balance: f64,
    pub credit_limit: f64,
    pub delivery_notes: String,
    pub active: bool,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Resumen de pedido asociado al historial de una tienda.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoreOrderSummary {
    pub id: Uuid,
    pub code: String,
    pub ordered_at: DateTime<Utc>,
    pub delivery_window: String,
    pub boxes_count: i32,
    pub total_amount: f64,
    pub status: OrderStatus,
}

/// Respuesta de `GET /api/v1/stores/:id/history`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoreHistoryResponse {
    pub store: StoreDto,
    pub orders: Vec<StoreOrderSummary>,
    pub transactions: Vec<Transaction>,
}

/// Respuesta del endpoint `GET /health`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub database: String,
    pub version: String,
    pub timestamp: DateTime<Utc>,
}

/// Estructura estándar de error JSON devuelta por el servidor.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiErrorBody {
    pub error: ApiErrorDetail,
}

/// Detalle interno del error HTTP.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiErrorDetail {
    pub code: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<Vec<crate::validation::FieldViolation>>,
}

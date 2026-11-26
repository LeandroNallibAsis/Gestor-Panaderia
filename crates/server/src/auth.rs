use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use axum::{
    async_trait,
    extract::FromRequestParts,
    http::{header::AUTHORIZATION, request::Parts},
};
use chrono::Utc;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use panaderia_core::{JwtClaims, Role, TokenKind};
use sqlx::Row;
use uuid::Uuid;

use crate::{config::AppState, error::AppError};

/// Par de tokens generados tras login o refresh.
#[derive(Debug, Clone)]
pub struct TokenPair {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: i64,
}

/// Usuario autenticado extraído desde el JWT en rutas protegidas.
#[derive(Debug, Clone)]
pub struct CurrentUser {
    pub id: Uuid,
    pub role: Role,
}

/// Exige que el usuario autenticado tenga rol `jefe`.
pub fn require_jefe(user: &CurrentUser) -> Result<(), AppError> {
    if !user.role.is_jefe() {
        return Err(AppError::Forbidden(
            "Esta acción requiere rol 'jefe'".to_string(),
        ));
    }
    Ok(())
}

/// Genera un hash Argon2id seguro para almacenar en base de datos.
pub fn hash_password(password: &str) -> Result<String, AppError> {
    let salt = SaltString::encode_b64(Uuid::new_v4().as_bytes())
        .map_err(|e| AppError::Internal(format!("No se pudo generar sal Argon2: {e}")))?;
    Argon2::default()
        .hash_password(password.as_bytes(), &salt)
        .map(|hash| hash.to_string())
        .map_err(|e| AppError::Internal(format!("No se pudo hashear la contraseña: {e}")))
}

/// Verifica una contraseña en texto plano frente a su hash Argon2id.
pub fn verify_password(password: &str, password_hash: &str) -> Result<bool, AppError> {
    let parsed_hash = PasswordHash::new(password_hash)
        .map_err(|e| AppError::Internal(format!("Hash de contraseña corrupto: {e}")))?;
    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}

/// Emite un par de tokens JWT (acceso y refresco).
pub fn create_token_pair(
    user_id: Uuid,
    role: Role,
    jwt_secret: &str,
    access_ttl_seconds: i64,
    refresh_ttl_seconds: i64,
) -> Result<TokenPair, AppError> {
    let access_token = issue_token(
        user_id,
        role,
        TokenKind::Access,
        jwt_secret,
        access_ttl_seconds,
    )?;
    let refresh_token = issue_token(
        user_id,
        role,
        TokenKind::Refresh,
        jwt_secret,
        refresh_ttl_seconds,
    )?;

    Ok(TokenPair {
        access_token,
        refresh_token,
        expires_in: access_ttl_seconds,
    })
}

fn issue_token(
    user_id: Uuid,
    role: Role,
    token_type: TokenKind,
    jwt_secret: &str,
    ttl_seconds: i64,
) -> Result<String, AppError> {
    let now = Utc::now().timestamp();
    let exp = (now + ttl_seconds).max(now) as usize;
    let claims = JwtClaims {
        sub: user_id,
        role,
        token_type,
        exp,
        iat: now.max(0) as usize,
        jti: Uuid::new_v4(),
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(jwt_secret.as_bytes()),
    )
    .map_err(|e| AppError::Internal(format!("Error al firmar JWT: {e}")))
}

/// Decodifica y valida la firma/expiración de un JWT y verifica el tipo esperado.
pub fn decode_token(
    token: &str,
    jwt_secret: &str,
    expected_kind: TokenKind,
) -> Result<JwtClaims, AppError> {
    let mut validation = Validation::default();
    validation.validate_exp = true;

    let token_data = decode::<JwtClaims>(
        token,
        &DecodingKey::from_secret(jwt_secret.as_bytes()),
        &validation,
    )
    .map_err(|_| AppError::Unauthorized("Token inválido o expirado".to_string()))?;

    if token_data.claims.token_type != expected_kind {
        return Err(AppError::Unauthorized(
            "Tipo de token incorrecto para esta operación".to_string(),
        ));
    }

    Ok(token_data.claims)
}

#[async_trait]
impl FromRequestParts<AppState> for CurrentUser {
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get(AUTHORIZATION)
            .and_then(|value| value.to_str().ok())
            .ok_or_else(|| AppError::Unauthorized("Falta cabecera Authorization".to_string()))?;

        let token = auth_header
            .strip_prefix("Bearer ")
            .or_else(|| auth_header.strip_prefix("bearer "))
            .map(str::trim)
            .filter(|t| !t.is_empty())
            .ok_or_else(|| {
                AppError::Unauthorized("Formato esperado: Authorization: Bearer <token>".to_string())
            })?;

        let claims = decode_token(token, &state.jwt_secret, TokenKind::Access)?;

        // Verifica en base de datos que el usuario siga activo y mantiene su rol actualizado.
        let row = sqlx::query(
            r#"
            SELECT u.is_active, r.name AS role_name
            FROM users u
            INNER JOIN roles r ON r.id = u.role_id
            WHERE u.id = $1
            "#,
        )
        .bind(claims.sub)
        .fetch_optional(&state.db)
        .await?;

        let row = row.ok_or_else(|| {
            AppError::Unauthorized("Usuario asociado al token no encontrado".to_string())
        })?;

        let is_active: bool = row.get("is_active");
        if !is_active {
            return Err(AppError::Unauthorized(
                "La cuenta de usuario está desactivada".to_string(),
            ));
        }

        let role_name: String = row.get("role_name");
        let role = role_name
            .parse::<Role>()
            .map_err(|e| AppError::Internal(format!("Rol en base de datos inválido: {e}")))?;

        Ok(Self {
            id: claims.sub,
            role,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn argon2_hash_and_verify() {
        let password = "SuperSecretBakery2026!";
        let hash = hash_password(password).expect("debe generar hash");
        assert!(hash.starts_with("$argon2id$"));
        assert!(verify_password(password, &hash).expect("debe verificar"));
        assert!(!verify_password("wrong-password", &hash).expect("debe verificar"));
    }

    #[test]
    fn jwt_issue_and_validate() {
        let user_id = Uuid::new_v4();
        let secret = "test-secret-key-at-least-32-chars-long";
        let pair = create_token_pair(user_id, Role::Jefe, secret, 3600, 86400)
            .expect("debe emitir tokens");

        let access_claims =
            decode_token(&pair.access_token, secret, TokenKind::Access).expect("access válido");
        assert_eq!(access_claims.sub, user_id);
        assert_eq!(access_claims.role, Role::Jefe);

        let refresh_claims =
            decode_token(&pair.refresh_token, secret, TokenKind::Refresh).expect("refresh válido");
        assert_eq!(refresh_claims.sub, user_id);

        assert!(decode_token(&pair.access_token, secret, TokenKind::Refresh).is_err());
        assert!(decode_token(&pair.access_token, "otro-secreto", TokenKind::Access).is_err());
    }
}

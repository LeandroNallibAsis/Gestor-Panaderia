use sqlx::PgPool;
use std::env;

/// Configuración del servidor leída desde variables de entorno.
#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub jwt_secret: String,
    pub server_host: String,
    pub server_port: u16,
    pub osrm_url: String,
    pub access_ttl_seconds: i64,
    pub refresh_ttl_seconds: i64,
}

impl Config {
    pub fn from_env() -> Result<Self, String> {
        let database_url = env::var("DATABASE_URL")
            .map_err(|_| "Falta la variable de entorno DATABASE_URL".to_string())?;
        let jwt_secret = env::var("JWT_SECRET")
            .map_err(|_| "Falta la variable de entorno JWT_SECRET".to_string())?;
        if jwt_secret.trim().len() < 16 {
            return Err("JWT_SECRET debe tener al menos 16 caracteres".to_string());
        }

        let server_host = env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
        let server_port = env::var("SERVER_PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse::<u16>()
            .map_err(|_| "SERVER_PORT debe ser un puerto válido (0-65535)".to_string())?;
        let osrm_url = env::var("OSRM_URL").unwrap_or_else(|_| "http://localhost:5000".to_string());

        let access_ttl_seconds = env::var("JWT_ACCESS_TTL_SECONDS")
            .ok()
            .and_then(|v| v.parse::<i64>().ok())
            .unwrap_or(3600);
        let refresh_ttl_seconds = env::var("JWT_REFRESH_TTL_SECONDS")
            .ok()
            .and_then(|v| v.parse::<i64>().ok())
            .unwrap_or(60 * 60 * 24 * 7);

        Ok(Self {
            database_url,
            jwt_secret,
            server_host,
            server_port,
            osrm_url,
            access_ttl_seconds,
            refresh_ttl_seconds,
        })
    }

    pub fn bind_addr(&self) -> String {
        format!("{}:{}", self.server_host, self.server_port)
    }
}

/// Estado compartido inyectado en todos los handlers de Axum.
#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub jwt_secret: String,
    pub osrm_url: String,
    pub access_ttl_seconds: i64,
    pub refresh_ttl_seconds: i64,
}

impl AppState {
    pub fn new(db: PgPool, config: &Config) -> Self {
        Self {
            db,
            jwt_secret: config.jwt_secret.clone(),
            osrm_url: config.osrm_url.clone(),
            access_ttl_seconds: config.access_ttl_seconds,
            refresh_ttl_seconds: config.refresh_ttl_seconds,
        }
    }
}

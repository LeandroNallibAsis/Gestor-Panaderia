use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use server::{build_router, AppState, Config};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "server=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = Config::from_env().map_err(|err| {
        tracing::error!(error = %err, "Configuración inválida");
        err
    })?;

    tracing::info!("Conectando a PostgreSQL...");
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&config.database_url)
        .await?;

    tracing::info!("Ejecutando migraciones SQLx...");
    sqlx::migrate!("./migrations").run(&pool).await?;

    let state = AppState::new(pool, &config);
    let app = build_router(state);

    let addr = config.bind_addr();
    let listener = TcpListener::bind(&addr).await?;
    tracing::info!(address = %addr, osrm_url = %config.osrm_url, "Servidor Panadería Control escuchando");

    axum::serve(listener, app).await?;
    Ok(())
}

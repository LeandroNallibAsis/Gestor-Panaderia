use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use panaderia_core::{
    CreateStoreRequest, OrderStatus, StoreDto, StoreHistoryResponse, StoreOrderSummary,
    Transaction, TransactionType, UpdateStoreRequest,
};
use serde::Deserialize;
use sqlx::Row;
use uuid::Uuid;

use crate::{auth::CurrentUser, config::AppState, error::AppError};

#[derive(Debug, Deserialize, Default)]
pub struct StoreListFilter {
    pub active: Option<bool>,
    pub zone: Option<String>,
}

fn row_to_store_dto(row: &sqlx::postgres::PgRow) -> StoreDto {
    StoreDto {
        id: row.get("id"),
        name: row.get("name"),
        zone: row.get("zone"),
        address: row.get("address"),
        contact_name: row.get("contact_name"),
        phone: row.get("phone"),
        delivery_window: row.get("delivery_window"),
        pending_balance: row.get("pending_balance"),
        credit_limit: row.get("credit_limit"),
        delivery_notes: row.get("delivery_notes"),
        active: row.get("active"),
        latitude: row.get("latitude"),
        longitude: row.get("longitude"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
    }
}

async fn fetch_store_by_id(state: &AppState, store_id: Uuid) -> Result<StoreDto, AppError> {
    let row = sqlx::query(
        r#"
        SELECT
            id,
            name,
            zone,
            address,
            contact_name,
            phone,
            delivery_window,
            pending_balance::DOUBLE PRECISION AS pending_balance,
            credit_limit::DOUBLE PRECISION AS credit_limit,
            delivery_notes,
            active,
            ST_Y(location::geometry) AS latitude,
            ST_X(location::geometry) AS longitude,
            created_at,
            updated_at
        FROM stores
        WHERE id = $1
        "#,
    )
    .bind(store_id)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("Tienda {store_id} no encontrada")))?;

    Ok(row_to_store_dto(&row))
}

/// `GET /api/v1/stores`
pub async fn list_stores(
    State(state): State<AppState>,
    _current_user: CurrentUser,
    Query(filter): Query<StoreListFilter>,
) -> Result<Json<Vec<StoreDto>>, AppError> {
    let zone_filter = filter
        .zone
        .as_deref()
        .map(str::trim)
        .filter(|z| !z.is_empty())
        .map(str::to_string);

    let rows = sqlx::query(
        r#"
        SELECT
            id,
            name,
            zone,
            address,
            contact_name,
            phone,
            delivery_window,
            pending_balance::DOUBLE PRECISION AS pending_balance,
            credit_limit::DOUBLE PRECISION AS credit_limit,
            delivery_notes,
            active,
            ST_Y(location::geometry) AS latitude,
            ST_X(location::geometry) AS longitude,
            created_at,
            updated_at
        FROM stores
        WHERE ($1::BOOLEAN IS NULL OR active = $1)
          AND ($2::TEXT IS NULL OR LOWER(zone) = LOWER($2))
        ORDER BY name ASC
        "#,
    )
    .bind(filter.active)
    .bind(zone_filter)
    .fetch_all(&state.db)
    .await?;

    let stores = rows.iter().map(row_to_store_dto).collect();
    Ok(Json(stores))
}

/// `POST /api/v1/stores`
pub async fn create_store(
    State(state): State<AppState>,
    _current_user: CurrentUser,
    Json(payload): Json<CreateStoreRequest>,
) -> Result<(StatusCode, Json<StoreDto>), AppError> {
    payload.validate()?;

    let credit_limit = payload.credit_limit.unwrap_or(0.0);
    let delivery_notes = payload.delivery_notes.as_deref().unwrap_or("").trim();
    let active = payload.active.unwrap_or(true);

    let new_id: Uuid = sqlx::query_scalar(
        r#"
        INSERT INTO stores (
            name,
            zone,
            address,
            contact_name,
            phone,
            delivery_window,
            credit_limit,
            delivery_notes,
            active,
            location
        )
        VALUES (
            $1,
            $2,
            $3,
            $4,
            $5,
            $6,
            $7::DOUBLE PRECISION,
            $8,
            $9,
            CASE
                WHEN $10::DOUBLE PRECISION IS NOT NULL AND $11::DOUBLE PRECISION IS NOT NULL
                THEN ST_SetSRID(ST_MakePoint($11, $10), 4326)::geography
                ELSE NULL
            END
        )
        RETURNING id
        "#,
    )
    .bind(payload.name.trim())
    .bind(payload.zone.trim())
    .bind(payload.address.trim())
    .bind(payload.contact_name.trim())
    .bind(payload.phone.trim())
    .bind(payload.delivery_window.trim())
    .bind(credit_limit)
    .bind(delivery_notes)
    .bind(active)
    .bind(payload.latitude)
    .bind(payload.longitude)
    .fetch_one(&state.db)
    .await?;

    let store = fetch_store_by_id(&state, new_id).await?;
    Ok((StatusCode::CREATED, Json(store)))
}

/// `GET /api/v1/stores/:id`
pub async fn get_store(
    State(state): State<AppState>,
    _current_user: CurrentUser,
    Path(id): Path<Uuid>,
) -> Result<Json<StoreDto>, AppError> {
    let store = fetch_store_by_id(&state, id).await?;
    Ok(Json(store))
}

/// `PATCH /api/v1/stores/:id`
pub async fn update_store(
    State(state): State<AppState>,
    _current_user: CurrentUser,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdateStoreRequest>,
) -> Result<Json<StoreDto>, AppError> {
    payload.validate()?;

    // Verificar existencia antes de actualizar.
    let _existing = fetch_store_by_id(&state, id).await?;

    let name = payload.name.as_deref().map(str::trim).map(str::to_string);
    let zone = payload.zone.as_deref().map(str::trim).map(str::to_string);
    let address = payload
        .address
        .as_deref()
        .map(str::trim)
        .map(str::to_string);
    let contact_name = payload
        .contact_name
        .as_deref()
        .map(str::trim)
        .map(str::to_string);
    let phone = payload.phone.as_deref().map(str::trim).map(str::to_string);
    let delivery_window = payload
        .delivery_window
        .as_deref()
        .map(str::trim)
        .map(str::to_string);
    let delivery_notes = payload
        .delivery_notes
        .as_deref()
        .map(str::trim)
        .map(str::to_string);

    sqlx::query(
        r#"
        UPDATE stores
        SET
            name = COALESCE($2, name),
            zone = COALESCE($3, zone),
            address = COALESCE($4, address),
            contact_name = COALESCE($5, contact_name),
            phone = COALESCE($6, phone),
            delivery_window = COALESCE($7, delivery_window),
            pending_balance = COALESCE($8::DOUBLE PRECISION, pending_balance),
            credit_limit = COALESCE($9::DOUBLE PRECISION, credit_limit),
            delivery_notes = COALESCE($10, delivery_notes),
            active = COALESCE($11, active),
            location = CASE
                WHEN $12::DOUBLE PRECISION IS NOT NULL AND $13::DOUBLE PRECISION IS NOT NULL
                THEN ST_SetSRID(ST_MakePoint($13, $12), 4326)::geography
                ELSE location
            END,
            updated_at = NOW()
        WHERE id = $1
        "#,
    )
    .bind(id)
    .bind(name)
    .bind(zone)
    .bind(address)
    .bind(contact_name)
    .bind(phone)
    .bind(delivery_window)
    .bind(payload.pending_balance)
    .bind(payload.credit_limit)
    .bind(delivery_notes)
    .bind(payload.active)
    .bind(payload.latitude)
    .bind(payload.longitude)
    .execute(&state.db)
    .await?;

    let updated = fetch_store_by_id(&state, id).await?;
    Ok(Json(updated))
}

/// `GET /api/v1/stores/:id/history`
pub async fn get_store_history(
    State(state): State<AppState>,
    _current_user: CurrentUser,
    Path(id): Path<Uuid>,
) -> Result<Json<StoreHistoryResponse>, AppError> {
    let store = fetch_store_by_id(&state, id).await?;

    let order_rows = sqlx::query(
        r#"
        SELECT
            id,
            code,
            ordered_at,
            delivery_window,
            boxes_count,
            total_amount::DOUBLE PRECISION AS total_amount,
            status
        FROM orders
        WHERE store_id = $1
        ORDER BY ordered_at DESC
        LIMIT 50
        "#,
    )
    .bind(id)
    .fetch_all(&state.db)
    .await?;

    let orders = order_rows
        .iter()
        .map(|row| {
            let status_str: String = row.get("status");
            let status = status_str
                .parse::<OrderStatus>()
                .unwrap_or(OrderStatus::Pendiente);
            StoreOrderSummary {
                id: row.get("id"),
                code: row.get("code"),
                ordered_at: row.get("ordered_at"),
                delivery_window: row.get("delivery_window"),
                boxes_count: row.get("boxes_count"),
                total_amount: row.get("total_amount"),
                status,
            }
        })
        .collect();

    let tx_rows = sqlx::query(
        r#"
        SELECT
            id,
            store_id,
            order_id,
            user_id,
            transaction_type,
            category,
            concept,
            amount::DOUBLE PRECISION AS amount,
            vehicle,
            occurred_at,
            created_at
        FROM transactions
        WHERE store_id = $1
        ORDER BY occurred_at DESC
        LIMIT 50
        "#,
    )
    .bind(id)
    .fetch_all(&state.db)
    .await?;

    let transactions = tx_rows
        .iter()
        .map(|row| {
            let tx_type_str: String = row.get("transaction_type");
            let transaction_type = tx_type_str
                .parse::<TransactionType>()
                .unwrap_or(TransactionType::Cobro);
            Transaction {
                id: row.get("id"),
                store_id: row.get("store_id"),
                order_id: row.get("order_id"),
                user_id: row.get("user_id"),
                transaction_type,
                category: row.get("category"),
                concept: row.get("concept"),
                amount: row.get("amount"),
                vehicle: row.get("vehicle"),
                occurred_at: row.get("occurred_at"),
                created_at: row.get("created_at"),
            }
        })
        .collect();

    Ok(Json(StoreHistoryResponse {
        store,
        orders,
        transactions,
    }))
}

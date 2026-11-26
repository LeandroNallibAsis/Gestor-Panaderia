use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;
use uuid::Uuid;

/// Roles del sistema para control de acceso (RBAC).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Role {
    Jefe,
    Repartidor,
    Produccion,
}

impl Role {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Jefe => "jefe",
            Self::Repartidor => "repartidor",
            Self::Produccion => "produccion",
        }
    }

    pub const fn is_jefe(self) -> bool {
        matches!(self, Self::Jefe)
    }
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for Role {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_ascii_lowercase().as_str() {
            "jefe" => Ok(Self::Jefe),
            "repartidor" => Ok(Self::Repartidor),
            "produccion" | "producción" => Ok(Self::Produccion),
            other => Err(format!("Rol inválido: '{other}'")),
        }
    }
}

/// Estado operativo actual de un usuario.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum UserStatus {
    #[default]
    Activo,
    EnRuta,
    Obrador,
    Descanso,
}

impl UserStatus {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Activo => "activo",
            Self::EnRuta => "en_ruta",
            Self::Obrador => "obrador",
            Self::Descanso => "descanso",
        }
    }
}

impl fmt::Display for UserStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for UserStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_ascii_lowercase().as_str() {
            "activo" => Ok(Self::Activo),
            "en_ruta" => Ok(Self::EnRuta),
            "obrador" => Ok(Self::Obrador),
            "descanso" => Ok(Self::Descanso),
            other => Err(format!("Estado de usuario inválido: '{other}'")),
        }
    }
}

/// Entidad completa de usuario en el dominio.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub role_id: Uuid,
    pub role: Role,
    pub name: String,
    pub email: String,
    pub phone: String,
    #[serde(default, skip_serializing)]
    pub password_hash: String,
    pub assigned_vehicle: Option<String>,
    pub status: UserStatus,
    pub shift: String,
    pub is_active: bool,
    pub last_activity_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Tienda o punto de venta cliente.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Store {
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

/// Materia prima / ingrediente de obrador.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ingredient {
    pub id: Uuid,
    pub name: String,
    pub unit: String,
    pub current_stock: f64,
    pub min_stock: f64,
    pub unit_cost: f64,
    pub storage_location: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Producto terminado elaborado en panadería.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    pub id: Uuid,
    pub sku: String,
    pub name: String,
    pub unit: String,
    pub unit_price: f64,
    pub current_stock: f64,
    pub min_stock: f64,
    pub storage_location: String,
    pub active: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Línea de receta que asocia un producto con la cantidad de ingrediente necesaria.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecipeItem {
    pub id: Uuid,
    pub product_id: Uuid,
    pub ingredient_id: Uuid,
    pub quantity_required: f64,
    pub unit: String,
    pub notes: Option<String>,
}

/// Estado de un lote de producción.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum BatchStatus {
    #[default]
    EnFermentacion,
    Horneando,
    Listo,
}

impl BatchStatus {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EnFermentacion => "en_fermentacion",
            Self::Horneando => "horneando",
            Self::Listo => "listo",
        }
    }
}

/// Lote de producción en obrador.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProductionBatch {
    pub id: Uuid,
    pub product_id: Uuid,
    pub batch_code: String,
    pub target_units: i32,
    pub completed_units: i32,
    pub oven_name: Option<String>,
    pub temperature_c: Option<f64>,
    pub remaining_minutes: i32,
    pub status: BatchStatus,
    pub assigned_user_id: Option<Uuid>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

/// Tipo de movimiento de inventario.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StockMovementType {
    Entrada,
    Salida,
    Ajuste,
    Produccion,
}

impl StockMovementType {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Entrada => "entrada",
            Self::Salida => "salida",
            Self::Ajuste => "ajuste",
            Self::Produccion => "produccion",
        }
    }
}

/// Registro histórico de cambio de stock (materia prima o producto terminado).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StockMovement {
    pub id: Uuid,
    pub ingredient_id: Option<Uuid>,
    pub product_id: Option<Uuid>,
    pub movement_type: StockMovementType,
    pub quantity: f64,
    pub reason: String,
    pub reference_id: Option<Uuid>,
    pub created_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

/// Estado del ciclo de vida de un pedido.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum OrderStatus {
    #[default]
    Pendiente,
    EnHorno,
    EnReparto,
    Entregado,
    Cancelado,
}

impl OrderStatus {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Pendiente => "pendiente",
            Self::EnHorno => "en_horno",
            Self::EnReparto => "en_reparto",
            Self::Entregado => "entregado",
            Self::Cancelado => "cancelado",
        }
    }
}

impl FromStr for OrderStatus {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_ascii_lowercase().as_str() {
            "pendiente" => Ok(Self::Pendiente),
            "en_horno" => Ok(Self::EnHorno),
            "en_reparto" => Ok(Self::EnReparto),
            "entregado" => Ok(Self::Entregado),
            "cancelado" => Ok(Self::Cancelado),
            other => Err(format!("Estado de pedido inválido: '{other}'")),
        }
    }
}

/// Pedido realizado por o para una tienda.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub id: Uuid,
    pub code: String,
    pub store_id: Uuid,
    pub created_by: Option<Uuid>,
    pub assigned_driver_id: Option<Uuid>,
    pub delivery_window: String,
    pub boxes_count: i32,
    pub total_amount: f64,
    pub status: OrderStatus,
    pub cash_on_delivery: bool,
    pub notes: Option<String>,
    pub ordered_at: DateTime<Utc>,
    pub delivered_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Línea individual dentro de un pedido.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderItem {
    pub id: Uuid,
    pub order_id: Uuid,
    pub product_id: Uuid,
    pub quantity: f64,
    pub unit_price: f64,
    pub subtotal: f64,
}

/// Estado de una ruta de reparto.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum RouteStatus {
    #[default]
    Planificada,
    EnCurso,
    Completada,
}

/// Ruta logística diaria asignada a un repartidor.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Route {
    pub id: Uuid,
    pub name: String,
    pub driver_id: Option<Uuid>,
    pub route_date: NaiveDate,
    pub status: RouteStatus,
    pub total_distance_km: f64,
    pub estimated_duration_min: i32,
    pub osrm_geometry: Option<String>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}

/// Estado de una parada dentro de una ruta.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum RouteStopStatus {
    #[default]
    Pendiente,
    EnCamino,
    Entregado,
    Incidencia,
}

/// Parada individual de entrega dentro de una ruta.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RouteStop {
    pub id: Uuid,
    pub route_id: Uuid,
    pub store_id: Uuid,
    pub order_id: Option<Uuid>,
    pub stop_order: i32,
    pub delivery_window: String,
    pub boxes_count: i32,
    pub amount_to_collect: f64,
    pub distance_km: f64,
    pub status: RouteStopStatus,
    pub arrived_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub notes: Option<String>,
}

/// Posición GPS reportada por un dispositivo de reparto.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GpsPosition {
    pub id: Uuid,
    pub user_id: Uuid,
    pub route_id: Option<Uuid>,
    pub latitude: f64,
    pub longitude: f64,
    pub speed_kmh: Option<f64>,
    pub heading: Option<f64>,
    pub accuracy_m: Option<f64>,
    pub recorded_at: DateTime<Utc>,
}

/// Tipo de aviso o recordatorio operativo.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum ReminderType {
    Urgente,
    Stock,
    Ruta,
    #[default]
    Recordatorio,
}

/// Aviso o recordatorio interno.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reminder {
    pub id: Uuid,
    pub title: String,
    pub message: String,
    pub reminder_type: ReminderType,
    pub target_role: Option<Role>,
    pub target_user_id: Option<Uuid>,
    pub is_read: bool,
    pub due_at: Option<DateTime<Utc>>,
    pub created_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

/// Tipo de movimiento económico / financiero.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TransactionType {
    Ingreso,
    Gasto,
    Cobro,
    Ajuste,
}

impl TransactionType {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Ingreso => "ingreso",
            Self::Gasto => "gasto",
            Self::Cobro => "cobro",
            Self::Ajuste => "ajuste",
        }
    }
}

impl FromStr for TransactionType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_ascii_lowercase().as_str() {
            "ingreso" => Ok(Self::Ingreso),
            "gasto" => Ok(Self::Gasto),
            "cobro" => Ok(Self::Cobro),
            "ajuste" => Ok(Self::Ajuste),
            other => Err(format!("Tipo de transacción inválido: '{other}'")),
        }
    }
}

/// Transacción económica (cobros a tiendas, gastos de ruta, ingresos).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub id: Uuid,
    pub store_id: Option<Uuid>,
    pub order_id: Option<Uuid>,
    pub user_id: Option<Uuid>,
    pub transaction_type: TransactionType,
    pub category: String,
    pub concept: String,
    pub amount: f64,
    pub vehicle: Option<String>,
    pub occurred_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn role_roundtrip_and_serialization() {
        assert_eq!(Role::Jefe.as_str(), "jefe");
        assert_eq!(Role::Repartidor.as_str(), "repartidor");
        assert_eq!(Role::Produccion.as_str(), "produccion");

        assert_eq!("jefe".parse::<Role>().unwrap(), Role::Jefe);
        assert_eq!("REPARTIDOR".parse::<Role>().unwrap(), Role::Repartidor);
        assert_eq!("producción".parse::<Role>().unwrap(), Role::Produccion);
        assert!("desconocido".parse::<Role>().is_err());

        let json = serde_json::to_string(&Role::Produccion).unwrap();
        assert_eq!(json, "\"produccion\"");
        let parsed: Role = serde_json::from_str("\"repartidor\"").unwrap();
        assert_eq!(parsed, Role::Repartidor);
    }

    #[test]
    fn role_permissions_helper() {
        assert!(Role::Jefe.is_jefe());
        assert!(!Role::Repartidor.is_jefe());
        assert!(!Role::Produccion.is_jefe());
    }
}

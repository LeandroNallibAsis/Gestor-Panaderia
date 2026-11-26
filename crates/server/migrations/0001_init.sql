-- Esquema inicial de Panadería Control (PostgreSQL 16 + PostGIS 3.4)

CREATE EXTENSION IF NOT EXISTS "pgcrypto";
CREATE EXTENSION IF NOT EXISTS postgis;

-- 1. Roles del sistema
CREATE TABLE IF NOT EXISTS roles (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL UNIQUE,
    description TEXT NOT NULL DEFAULT '',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

INSERT INTO roles (name, description) VALUES
    ('jefe', 'Dirección y administración general de la panadería'),
    ('repartidor', 'Logística, rutas de reparto, entregas y cobros'),
    ('produccion', 'Obrador, hornos, recetas, lotes y control de stock')
ON CONFLICT (name) DO NOTHING;

-- 2. Usuarios
CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    role_id UUID NOT NULL REFERENCES roles(id) ON DELETE RESTRICT,
    name TEXT NOT NULL,
    email TEXT NOT NULL UNIQUE,
    phone TEXT NOT NULL,
    password_hash TEXT NOT NULL,
    assigned_vehicle TEXT,
    status TEXT NOT NULL DEFAULT 'activo',
    shift TEXT NOT NULL DEFAULT 'Mañana',
    is_active BOOLEAN NOT NULL DEFAULT TRUE,
    last_activity_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_users_role_id ON users(role_id);
CREATE INDEX IF NOT EXISTS idx_users_email_lower ON users(LOWER(email));

-- 3. Tiendas / Puntos de venta
CREATE TABLE IF NOT EXISTS stores (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL,
    zone TEXT NOT NULL,
    address TEXT NOT NULL,
    contact_name TEXT NOT NULL,
    phone TEXT NOT NULL,
    delivery_window TEXT NOT NULL,
    pending_balance NUMERIC(12, 2) NOT NULL DEFAULT 0.00 CHECK (pending_balance >= 0),
    credit_limit NUMERIC(12, 2) NOT NULL DEFAULT 0.00 CHECK (credit_limit >= 0),
    delivery_notes TEXT NOT NULL DEFAULT '',
    active BOOLEAN NOT NULL DEFAULT TRUE,
    location geography(Point, 4326),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_stores_location ON stores USING GIST (location);
CREATE INDEX IF NOT EXISTS idx_stores_zone ON stores(zone);
CREATE INDEX IF NOT EXISTS idx_stores_active ON stores(active);

-- 4. Ingredientes / Materias primas
CREATE TABLE IF NOT EXISTS ingredients (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL UNIQUE,
    unit TEXT NOT NULL,
    current_stock NUMERIC(12, 3) NOT NULL DEFAULT 0.000 CHECK (current_stock >= 0),
    min_stock NUMERIC(12, 3) NOT NULL DEFAULT 0.000 CHECK (min_stock >= 0),
    unit_cost NUMERIC(12, 2) NOT NULL DEFAULT 0.00 CHECK (unit_cost >= 0),
    storage_location TEXT NOT NULL DEFAULT 'Almacén principal',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- 5. Productos terminados
CREATE TABLE IF NOT EXISTS products (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    sku TEXT NOT NULL UNIQUE,
    name TEXT NOT NULL,
    unit TEXT NOT NULL DEFAULT 'unidad',
    unit_price NUMERIC(12, 2) NOT NULL DEFAULT 0.00 CHECK (unit_price >= 0),
    current_stock NUMERIC(12, 3) NOT NULL DEFAULT 0.000 CHECK (current_stock >= 0),
    min_stock NUMERIC(12, 3) NOT NULL DEFAULT 0.000 CHECK (min_stock >= 0),
    storage_location TEXT NOT NULL DEFAULT 'Obrador',
    active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- 6. Recetas (relación producto <-> ingrediente)
CREATE TABLE IF NOT EXISTS recipes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    product_id UUID NOT NULL REFERENCES products(id) ON DELETE CASCADE,
    ingredient_id UUID NOT NULL REFERENCES ingredients(id) ON DELETE RESTRICT,
    quantity_required NUMERIC(12, 3) NOT NULL CHECK (quantity_required > 0),
    unit TEXT NOT NULL,
    notes TEXT,
    UNIQUE (product_id, ingredient_id)
);

CREATE INDEX IF NOT EXISTS idx_recipes_product_id ON recipes(product_id);

-- 7. Lotes de producción
CREATE TABLE IF NOT EXISTS production_batches (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    product_id UUID NOT NULL REFERENCES products(id) ON DELETE RESTRICT,
    batch_code TEXT NOT NULL UNIQUE,
    target_units INTEGER NOT NULL CHECK (target_units > 0),
    completed_units INTEGER NOT NULL DEFAULT 0 CHECK (completed_units >= 0),
    oven_name TEXT,
    temperature_c NUMERIC(5, 1),
    remaining_minutes INTEGER NOT NULL DEFAULT 0 CHECK (remaining_minutes >= 0),
    status TEXT NOT NULL DEFAULT 'en_fermentacion',
    assigned_user_id UUID REFERENCES users(id) ON DELETE SET NULL,
    started_at TIMESTAMPTZ,
    completed_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_production_batches_status ON production_batches(status);

-- 8. Movimientos de stock
CREATE TABLE IF NOT EXISTS stock_movements (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    ingredient_id UUID REFERENCES ingredients(id) ON DELETE CASCADE,
    product_id UUID REFERENCES products(id) ON DELETE CASCADE,
    movement_type TEXT NOT NULL,
    quantity NUMERIC(12, 3) NOT NULL,
    reason TEXT NOT NULL DEFAULT '',
    reference_id UUID,
    created_by UUID REFERENCES users(id) ON DELETE SET NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT chk_stock_movement_target CHECK (ingredient_id IS NOT NULL OR product_id IS NOT NULL)
);

CREATE INDEX IF NOT EXISTS idx_stock_movements_created_at ON stock_movements(created_at DESC);

-- 9. Pedidos
CREATE TABLE IF NOT EXISTS orders (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    code TEXT NOT NULL UNIQUE,
    store_id UUID NOT NULL REFERENCES stores(id) ON DELETE RESTRICT,
    created_by UUID REFERENCES users(id) ON DELETE SET NULL,
    assigned_driver_id UUID REFERENCES users(id) ON DELETE SET NULL,
    delivery_window TEXT NOT NULL,
    boxes_count INTEGER NOT NULL DEFAULT 0 CHECK (boxes_count >= 0),
    total_amount NUMERIC(12, 2) NOT NULL DEFAULT 0.00 CHECK (total_amount >= 0),
    status TEXT NOT NULL DEFAULT 'pendiente',
    cash_on_delivery BOOLEAN NOT NULL DEFAULT FALSE,
    notes TEXT,
    ordered_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    delivered_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_orders_store_id ON orders(store_id);
CREATE INDEX IF NOT EXISTS idx_orders_status ON orders(status);
CREATE INDEX IF NOT EXISTS idx_orders_ordered_at ON orders(ordered_at DESC);

-- 10. Líneas de pedido
CREATE TABLE IF NOT EXISTS order_items (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    order_id UUID NOT NULL REFERENCES orders(id) ON DELETE CASCADE,
    product_id UUID NOT NULL REFERENCES products(id) ON DELETE RESTRICT,
    quantity NUMERIC(12, 3) NOT NULL CHECK (quantity > 0),
    unit_price NUMERIC(12, 2) NOT NULL CHECK (unit_price >= 0),
    subtotal NUMERIC(12, 2) NOT NULL CHECK (subtotal >= 0)
);

CREATE INDEX IF NOT EXISTS idx_order_items_order_id ON order_items(order_id);

-- 11. Rutas de reparto
CREATE TABLE IF NOT EXISTS routes (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name TEXT NOT NULL,
    driver_id UUID REFERENCES users(id) ON DELETE SET NULL,
    route_date DATE NOT NULL DEFAULT CURRENT_DATE,
    status TEXT NOT NULL DEFAULT 'planificada',
    total_distance_km NUMERIC(10, 2) NOT NULL DEFAULT 0.00 CHECK (total_distance_km >= 0),
    estimated_duration_min INTEGER NOT NULL DEFAULT 0 CHECK (estimated_duration_min >= 0),
    osrm_geometry TEXT,
    started_at TIMESTAMPTZ,
    completed_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_routes_driver_date ON routes(driver_id, route_date);

-- 12. Paradas de ruta
CREATE TABLE IF NOT EXISTS route_stops (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    route_id UUID NOT NULL REFERENCES routes(id) ON DELETE CASCADE,
    store_id UUID NOT NULL REFERENCES stores(id) ON DELETE RESTRICT,
    order_id UUID REFERENCES orders(id) ON DELETE SET NULL,
    stop_order INTEGER NOT NULL CHECK (stop_order > 0),
    delivery_window TEXT NOT NULL DEFAULT '',
    boxes_count INTEGER NOT NULL DEFAULT 0 CHECK (boxes_count >= 0),
    amount_to_collect NUMERIC(12, 2) NOT NULL DEFAULT 0.00 CHECK (amount_to_collect >= 0),
    distance_km NUMERIC(10, 2) NOT NULL DEFAULT 0.00 CHECK (distance_km >= 0),
    status TEXT NOT NULL DEFAULT 'pendiente',
    arrived_at TIMESTAMPTZ,
    completed_at TIMESTAMPTZ,
    notes TEXT
);

CREATE INDEX IF NOT EXISTS idx_route_stops_route_order ON route_stops(route_id, stop_order);

-- 13. Posiciones GPS en tiempo real / histórico
CREATE TABLE IF NOT EXISTS gps_positions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    route_id UUID REFERENCES routes(id) ON DELETE SET NULL,
    location geography(Point, 4326) NOT NULL,
    speed_kmh NUMERIC(6, 2),
    heading NUMERIC(5, 2),
    accuracy_m NUMERIC(6, 2),
    recorded_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_gps_positions_location ON gps_positions USING GIST (location);
CREATE INDEX IF NOT EXISTS idx_gps_positions_user_time ON gps_positions(user_id, recorded_at DESC);

-- 14. Avisos y recordatorios
CREATE TABLE IF NOT EXISTS reminders (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title TEXT NOT NULL,
    message TEXT NOT NULL,
    reminder_type TEXT NOT NULL DEFAULT 'recordatorio',
    target_role TEXT,
    target_user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    is_read BOOLEAN NOT NULL DEFAULT FALSE,
    due_at TIMESTAMPTZ,
    created_by UUID REFERENCES users(id) ON DELETE SET NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_reminders_target ON reminders(target_role, target_user_id, is_read);

-- 15. Transacciones económicas (cobros, gastos de ruta, ingresos)
CREATE TABLE IF NOT EXISTS transactions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    store_id UUID REFERENCES stores(id) ON DELETE SET NULL,
    order_id UUID REFERENCES orders(id) ON DELETE SET NULL,
    user_id UUID REFERENCES users(id) ON DELETE SET NULL,
    transaction_type TEXT NOT NULL,
    category TEXT NOT NULL DEFAULT 'general',
    concept TEXT NOT NULL,
    amount NUMERIC(12, 2) NOT NULL CHECK (amount >= 0),
    vehicle TEXT,
    occurred_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_transactions_store_id ON transactions(store_id, occurred_at DESC);
CREATE INDEX IF NOT EXISTS idx_transactions_occurred_at ON transactions(occurred_at DESC);

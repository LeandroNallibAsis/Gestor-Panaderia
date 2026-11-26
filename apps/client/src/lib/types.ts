export type Role = 'jefe' | 'repartidor' | 'produccion';

export type NavTab =
	| 'panel'
	| 'stock'
	| 'mapa'
	| 'pedidos'
	| 'mas'
	| 'ruta'
	| 'gastos'
	| 'avisos'
	| 'produccion'
	| 'tiendas'
	| 'usuarios'
	| 'finanzas';

export type OrderStatus = 'pendiente' | 'en_horno' | 'en_reparto' | 'entregado';

export type UserStatus = 'activo' | 'en_ruta' | 'obrador' | 'descanso';

export interface StoreOrderSummary {
	id: string;
	fecha: string;
	resumen: string;
	total: number;
	estado: OrderStatus;
}

export interface StoreRecord {
	id: string;
	nombre: string;
	zona: string;
	direccion: string;
	contacto: string;
	telefono: string;
	horarioEntrega: string;
	saldoPendiente: number;
	limiteCredito: number;
	notasEntrega: string;
	activa: boolean;
	historialPedidos: StoreOrderSummary[];
}

export interface UserActivityEntry {
	id: string;
	hora: string;
	descripcion: string;
}

export interface UserRecord {
	id: string;
	nombre: string;
	email: string;
	telefono: string;
	rol: Role;
	vehiculoAsignado: string;
	estado: UserStatus;
	turno: string;
	ultimaActividad: string;
	historial: UserActivityEntry[];
}

export interface StockItem {
	id: string;
	nombre: string;
	categoria: 'materia_prima' | 'producto_terminado';
	cantidad: number;
	minimo: number;
	unidad: string;
	ubicacion: string;
	actualizadoHace: string;
}

export interface OrderRecord {
	id: string;
	codigo: string;
	tiendaId: string;
	tiendaNombre: string;
	zona: string;
	horaEntrega: string;
	itemsResumen: string;
	cajas: number;
	total: number;
	estado: OrderStatus;
	repartidorNombre: string;
	cobroContraEntrega: boolean;
}

export interface DeliveryStop {
	id: string;
	orden: number;
	tiendaId: string;
	tiendaNombre: string;
	direccion: string;
	zona: string;
	ventanaHoraria: string;
	cajas: number;
	importeCobrar: number;
	estado: 'pendiente' | 'en_camino' | 'entregado';
	distanciaKm: number;
}

export interface ProductionBatch {
	id: string;
	producto: string;
	lote: string;
	objetivoUnidades: number;
	completadasUnidades: number;
	horno: string;
	temperaturaC: number;
	minutosRestantes: number;
	estado: 'en_fermentacion' | 'horneando' | 'listo';
}

export interface ExpenseRecord {
	id: string;
	concepto: string;
	categoria: 'combustible' | 'peaje' | 'parking' | 'mantenimiento';
	importe: number;
	hora: string;
	vehiculo: string;
}

export interface NoticeRecord {
	id: string;
	titulo: string;
	mensaje: string;
	tipo: 'urgente' | 'stock' | 'ruta' | 'recordatorio';
	hora: string;
	leido: boolean;
	rolDestino: Role | 'todos';
}

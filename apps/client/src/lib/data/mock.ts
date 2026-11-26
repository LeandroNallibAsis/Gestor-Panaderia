import type {
	DeliveryStop,
	ExpenseRecord,
	NoticeRecord,
	OrderRecord,
	ProductionBatch,
	Role,
	StockItem,
	StoreRecord,
	UserRecord
} from '$lib/types';

export const ROLE_LABELS: Record<Role, string> = {
	jefe: 'Jefe / Encargado',
	repartidor: 'Repartidor',
	produccion: 'Producción'
};

export const ROLE_SHORT_LABELS: Record<Role, string> = {
	jefe: 'Jefe',
	repartidor: 'Repartidor',
	produccion: 'Obrador'
};

export const INITIAL_USERS: UserRecord[] = [
	{
		id: 'u-1',
		nombre: 'Carlos Mendoza',
		email: 'carlos@eltrigal.es',
		telefono: '612 340 891',
		rol: 'jefe',
		vehiculoAsignado: 'Sin vehículo (Central)',
		estado: 'activo',
		turno: '05:00 - 14:00',
		ultimaActividad: 'Hace 4 min · Aprobó pedido #PED-1048',
		historial: [
			{ id: 'h-1', hora: '07:40', descripcion: 'Revisó alertas de stock de harina T55' },
			{ id: 'h-2', hora: '06:55', descripcion: 'Registró cobro de 240,00 € en Hotel Mirador' },
			{ id: 'h-3', hora: '05:15', descripcion: 'Abrió jornada de obrador y rutas' }
		]
	},
	{
		id: 'u-2',
		nombre: 'Lucía Navarro',
		email: 'lucia@eltrigal.es',
		telefono: '645 118 204',
		rol: 'repartidor',
		vehiculoAsignado: 'Furgón Isotermo · 4821-KLP',
		estado: 'en_ruta',
		turno: '06:00 - 13:30',
		ultimaActividad: 'Hace 7 min · Entregó en Cafetería La Molienda',
		historial: [
			{ id: 'h-4', hora: '07:32', descripcion: 'Entrega completada: Cafetería La Molienda (6 cajas)' },
			{ id: 'h-5', hora: '06:48', descripcion: 'Cargó 28 cajas de bollería y barra rústica' },
			{ id: 'h-6', hora: '06:10', descripcion: 'Inició ruta Zona Centro - Norte' }
		]
	},
	{
		id: 'u-3',
		nombre: 'Mateo Ruiz',
		email: 'mateo@eltrigal.es',
		telefono: '678 902 331',
		rol: 'repartidor',
		vehiculoAsignado: 'Furgoneta Frío · 7104-MTR',
		estado: 'activo',
		turno: '06:30 - 14:00',
		ultimaActividad: 'Hace 19 min · Salió hacia Supermercado Barrio Sur',
		historial: [
			{ id: 'h-7', hora: '07:15', descripcion: 'Repostó diésel (45,00 €) en E.S. Avenida' },
			{ id: 'h-8', hora: '06:35', descripcion: 'Verificó albaranes de Zona Sur' }
		]
	},
	{
		id: 'u-4',
		nombre: 'Elena Gómez',
		email: 'elena@eltrigal.es',
		telefono: '629 551 088',
		rol: 'produccion',
		vehiculoAsignado: 'Obrador · Horno Rotativo 1',
		estado: 'obrador',
		turno: '03:00 - 11:00',
		ultimaActividad: 'Hace 2 min · Horneando Lote B-204 (Baguette)',
		historial: [
			{ id: 'h-9', hora: '07:45', descripcion: 'Reportó stock bajo de Harina Fuerza T55' },
			{ id: 'h-10', hora: '06:20', descripcion: 'Completó 180 u. de Croissant Mantequilla' },
			{ id: 'h-11', hora: '04:10', descripcion: 'Inició fermentación masa madre integral' }
		]
	},
	{
		id: 'u-5',
		nombre: 'Javier Torres',
		email: 'javier@eltrigal.es',
		telefono: '691 774 512',
		rol: 'produccion',
		vehiculoAsignado: 'Obrador · Mesa de Formado',
		estado: 'descanso',
		turno: '04:00 - 12:00',
		ultimaActividad: 'Hace 35 min · Finalizó empaquetado de pan de molde',
		historial: [
			{ id: 'h-12', hora: '07:10', descripcion: 'Completó lote de 90 panes de molde artesano' },
			{ id: 'h-13', hora: '05:00', descripcion: 'Preparó bandejas para turno de mañana' }
		]
	}
];

export const INITIAL_STORES: StoreRecord[] = [
	{
		id: 't-1',
		nombre: 'Cafetería La Molienda',
		zona: 'Centro',
		direccion: 'C/ Mayor 42, Bajo',
		contacto: 'Marta Serrano',
		telefono: '914 221 890',
		horarioEntrega: '07:00 - 07:45',
		saldoPendiente: 185.5,
		limiteCredito: 600,
		notasEntrega: 'Entrar por puerta lateral de servicio; dejar cajas junto a barra.',
		activa: true,
		historialPedidos: [
			{
				id: 'so-1',
				fecha: 'Hoy · 07:30',
				resumen: '60 croissants, 35 barras rústicas, 12 brioches',
				total: 142.5,
				estado: 'entregado'
			},
			{
				id: 'so-2',
				fecha: 'Ayer · 07:25',
				resumen: '55 croissants, 30 barras rústicas',
				total: 126.0,
				estado: 'entregado'
			}
		]
	},
	{
		id: 't-2',
		nombre: 'Supermercado Barrio Sur',
		zona: 'Sur',
		direccion: 'Av. Andalucía 118',
		contacto: 'Ricardo León',
		telefono: '915 883 412',
		horarioEntrega: '07:30 - 08:30',
		saldoPendiente: 420.0,
		limiteCredito: 1000,
		notasEntrega: 'Muelle de descarga 2. Pedir firma digital al encargado de panadería.',
		activa: true,
		historialPedidos: [
			{
				id: 'so-3',
				fecha: 'Hoy · 08:00',
				resumen: '120 baguettes, 40 panes de molde, 50 chapatas',
				total: 310.0,
				estado: 'en_reparto'
			},
			{
				id: 'so-4',
				fecha: 'Ayer · 07:55',
				resumen: '110 baguettes, 35 panes de molde',
				total: 278.4,
				estado: 'entregado'
			}
		]
	},
	{
		id: 't-3',
		nombre: 'Hotel Mirador Plaza',
		zona: 'Norte',
		direccion: 'Pº de la Estación 15',
		contacto: 'Sofía Domínguez',
		telefono: '917 094 210',
		horarioEntrega: '06:30 - 07:15',
		saldoPendiente: 0,
		limiteCredito: 1500,
		notasEntrega: 'Cocina planta -1. Código de portón: 4412#.',
		activa: true,
		historialPedidos: [
			{
				id: 'so-5',
				fecha: 'Hoy · 06:50',
				resumen: '90 mini croissants, 40 panecillos integrales, 20 hogazas',
				total: 248.0,
				estado: 'entregado'
			}
		]
	},
	{
		id: 't-4',
		nombre: 'Bodega El Rincón',
		zona: 'Este',
		direccion: 'C/ Toledo 77',
		contacto: 'Pablo Ibáñez',
		telefono: '913 662 145',
		horarioEntrega: '08:15 - 09:00',
		saldoPendiente: 94.2,
		limiteCredito: 400,
		notasEntrega: 'Cobro contra entrega si supera 100 €.',
		activa: true,
		historialPedidos: [
			{
				id: 'so-6',
				fecha: 'Hoy · 08:30',
				resumen: '45 barras de pueblo, 20 tortas de aceite',
				total: 94.2,
				estado: 'en_horno'
			}
		]
	},
	{
		id: 't-5',
		nombre: 'Colegio San Marcos',
		zona: 'Centro',
		direccion: 'C/ Cervantes 9',
		contacto: 'Ana Belén Cruz',
		telefono: '914 908 773',
		horarioEntrega: '08:00 - 08:45',
		saldoPendiente: 315.0,
		limiteCredito: 900,
		notasEntrega: 'Comedor escolar. Facturación mensual por transferencia.',
		activa: true,
		historialPedidos: [
			{
				id: 'so-7',
				fecha: 'Hoy · 08:20',
				resumen: '150 bocadillos blancos, 30 barras sin sal',
				total: 165.0,
				estado: 'pendiente'
			}
		]
	}
];

export const INITIAL_STOCK: StockItem[] = [
	{
		id: 's-1',
		nombre: 'Harina Fuerza T55 (Sacos 25 kg)',
		categoria: 'materia_prima',
		cantidad: 6,
		minimo: 10,
		unidad: 'sacos',
		ubicacion: 'Almacén A · Palé 1',
		actualizadoHace: 'Hace 5 min'
	},
	{
		id: 's-2',
		nombre: 'Mantequilla Francesa 82%',
		categoria: 'materia_prima',
		cantidad: 14,
		minimo: 18,
		unidad: 'kg',
		ubicacion: 'Cámara Frío 2',
		actualizadoHace: 'Hace 18 min'
	},
	{
		id: 's-3',
		nombre: 'Levadura Fresca Panadero',
		categoria: 'materia_prima',
		cantidad: 9,
		minimo: 8,
		unidad: 'kg',
		ubicacion: 'Cámara Frío 1',
		actualizadoHace: 'Hace 1 h'
	},
	{
		id: 's-4',
		nombre: 'Baguette Rústica',
		categoria: 'producto_terminado',
		cantidad: 210,
		minimo: 120,
		unidad: 'uds',
		ubicacion: 'Carros Expedición 1-3',
		actualizadoHace: 'Hace 3 min'
	},
	{
		id: 's-5',
		nombre: 'Croissant Mantequilla',
		categoria: 'producto_terminado',
		cantidad: 42,
		minimo: 60,
		unidad: 'uds',
		ubicacion: 'Vitrina / Bandeja B',
		actualizadoHace: 'Hace 12 min'
	},
	{
		id: 's-6',
		nombre: 'Hogaza Masa Madre 800g',
		categoria: 'producto_terminado',
		cantidad: 48,
		minimo: 30,
		unidad: 'uds',
		ubicacion: 'Estantería Horno 2',
		actualizadoHace: 'Hace 22 min'
	}
];

export const INITIAL_ORDERS: OrderRecord[] = [
	{
		id: 'o-1',
		codigo: '#PED-1048',
		tiendaId: 't-2',
		tiendaNombre: 'Supermercado Barrio Sur',
		zona: 'Sur',
		horaEntrega: '08:00',
		itemsResumen: '120 baguettes · 40 moldes · 50 chapatas',
		cajas: 11,
		total: 310.0,
		estado: 'en_reparto',
		repartidorNombre: 'Mateo Ruiz',
		cobroContraEntrega: false
	},
	{
		id: 'o-2',
		codigo: '#PED-1049',
		tiendaId: 't-4',
		tiendaNombre: 'Bodega El Rincón',
		zona: 'Este',
		horaEntrega: '08:30',
		itemsResumen: '45 barras pueblo · 20 tortas aceite',
		cajas: 4,
		total: 94.2,
		estado: 'en_horno',
		repartidorNombre: 'Lucía Navarro',
		cobroContraEntrega: true
	},
	{
		id: 'o-3',
		codigo: '#PED-1050',
		tiendaId: 't-5',
		tiendaNombre: 'Colegio San Marcos',
		zona: 'Centro',
		horaEntrega: '08:20',
		itemsResumen: '150 bocadillos · 30 barras sin sal',
		cajas: 6,
		total: 165.0,
		estado: 'pendiente',
		repartidorNombre: 'Lucía Navarro',
		cobroContraEntrega: false
	},
	{
		id: 'o-4',
		codigo: '#PED-1046',
		tiendaId: 't-1',
		tiendaNombre: 'Cafetería La Molienda',
		zona: 'Centro',
		horaEntrega: '07:30',
		itemsResumen: '60 croissants · 35 barras rústicas · 12 brioches',
		cajas: 6,
		total: 142.5,
		estado: 'entregado',
		repartidorNombre: 'Lucía Navarro',
		cobroContraEntrega: true
	},
	{
		id: 'o-5',
		codigo: '#PED-1045',
		tiendaId: 't-3',
		tiendaNombre: 'Hotel Mirador Plaza',
		zona: 'Norte',
		horaEntrega: '06:50',
		itemsResumen: '90 mini croissants · 40 integrales · 20 hogazas',
		cajas: 8,
		total: 248.0,
		estado: 'entregado',
		repartidorNombre: 'Lucía Navarro',
		cobroContraEntrega: false
	}
];

export const INITIAL_STOPS: DeliveryStop[] = [
	{
		id: 'r-1',
		orden: 1,
		tiendaId: 't-3',
		tiendaNombre: 'Hotel Mirador Plaza',
		direccion: 'Pº de la Estación 15',
		zona: 'Norte',
		ventanaHoraria: '06:30 - 07:15',
		cajas: 8,
		importeCobrar: 0,
		estado: 'entregado',
		distanciaKm: 2.4
	},
	{
		id: 'r-2',
		orden: 2,
		tiendaId: 't-1',
		tiendaNombre: 'Cafetería La Molienda',
		direccion: 'C/ Mayor 42, Bajo',
		zona: 'Centro',
		ventanaHoraria: '07:00 - 07:45',
		cajas: 6,
		importeCobrar: 142.5,
		estado: 'entregado',
		distanciaKm: 1.3
	},
	{
		id: 'r-3',
		orden: 3,
		tiendaId: 't-2',
		tiendaNombre: 'Supermercado Barrio Sur',
		direccion: 'Av. Andalucía 118',
		zona: 'Sur',
		ventanaHoraria: '07:30 - 08:30',
		cajas: 11,
		importeCobrar: 0,
		estado: 'en_camino',
		distanciaKm: 3.8
	},
	{
		id: 'r-4',
		orden: 4,
		tiendaId: 't-5',
		tiendaNombre: 'Colegio San Marcos',
		direccion: 'C/ Cervantes 9',
		zona: 'Centro',
		ventanaHoraria: '08:00 - 08:45',
		cajas: 6,
		importeCobrar: 0,
		estado: 'pendiente',
		distanciaKm: 1.9
	},
	{
		id: 'r-5',
		orden: 5,
		tiendaId: 't-4',
		tiendaNombre: 'Bodega El Rincón',
		direccion: 'C/ Toledo 77',
		zona: 'Este',
		ventanaHoraria: '08:15 - 09:00',
		cajas: 4,
		importeCobrar: 94.2,
		estado: 'pendiente',
		distanciaKm: 2.6
	}
];

export const INITIAL_BATCHES: ProductionBatch[] = [
	{
		id: 'b-1',
		producto: 'Baguette Rústica',
		lote: 'LOTE B-204',
		objetivoUnidades: 240,
		completadasUnidades: 210,
		horno: 'Horno Rotativo 1',
		temperaturaC: 235,
		minutosRestantes: 7,
		estado: 'horneando'
	},
	{
		id: 'b-2',
		producto: 'Bocadillo Blanco (Colegio)',
		lote: 'LOTE B-205',
		objetivoUnidades: 180,
		completadasUnidades: 60,
		horno: 'Horno Solera 2',
		temperaturaC: 220,
		minutosRestantes: 18,
		estado: 'en_fermentacion'
	},
	{
		id: 'b-3',
		producto: 'Croissant Mantequilla',
		lote: 'LOTE V-112',
		objetivoUnidades: 180,
		completadasUnidades: 180,
		horno: 'Horno Convección',
		temperaturaC: 195,
		minutosRestantes: 0,
		estado: 'listo'
	}
];

export const INITIAL_EXPENSES: ExpenseRecord[] = [
	{
		id: 'g-1',
		concepto: 'Diésel E.S. Avenida',
		categoria: 'combustible',
		importe: 45.0,
		hora: '07:15',
		vehiculo: '7104-MTR'
	},
	{
		id: 'g-2',
		concepto: 'Zona Carga y Descarga Centro',
		categoria: 'parking',
		importe: 3.4,
		hora: '07:28',
		vehiculo: '4821-KLP'
	}
];

export const INITIAL_NOTICES: NoticeRecord[] = [
	{
		id: 'n-1',
		titulo: 'Stock crítico: Harina Fuerza T55',
		mensaje: 'Quedan 6 sacos (mínimo recomendado: 10). Confirmar pedido con Harinera Castilla antes de las 11:00.',
		tipo: 'stock',
		hora: '07:45',
		leido: false,
		rolDestino: 'todos'
	},
	{
		id: 'n-2',
		titulo: 'Cobro pendiente en Bodega El Rincón',
		mensaje: 'Recoger 94,20 € en efectivo o TPV móvil al entregar #PED-1049.',
		tipo: 'ruta',
		hora: '07:30',
		leido: false,
		rolDestino: 'repartidor'
	},
	{
		id: 'n-3',
		titulo: 'Pedido extra Colegio San Marcos',
		mensaje: 'Añadidas 30 barras sin sal al lote de las 08:20.',
		tipo: 'urgente',
		hora: '07:12',
		leido: false,
		rolDestino: 'produccion'
	},
	{
		id: 'n-4',
		titulo: 'Revisión trimestral cámara frío 2',
		mensaje: 'Recordatorio técnico mañana a las 13:00.',
		tipo: 'recordatorio',
		hora: 'Ayer',
		leido: true,
		rolDestino: 'jefe'
	}
];

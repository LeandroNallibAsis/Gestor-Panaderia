import {
	INITIAL_BATCHES,
	INITIAL_EXPENSES,
	INITIAL_NOTICES,
	INITIAL_ORDERS,
	INITIAL_STOCK,
	INITIAL_STOPS,
	INITIAL_STORES,
	INITIAL_USERS
} from '$lib/data/mock';
import type {
	DeliveryStop,
	ExpenseRecord,
	NavTab,
	NoticeRecord,
	OrderRecord,
	OrderStatus,
	ProductionBatch,
	Role,
	StockItem,
	StoreRecord,
	UserRecord
} from '$lib/types';

export interface NavItem {
	id: NavTab;
	label: string;
	icon: string;
	badge?: number;
}

const DEFAULT_TAB_BY_ROLE: Record<Role, NavTab> = {
	jefe: 'panel',
	repartidor: 'ruta',
	produccion: 'produccion'
};

class AppStore {
	authenticated = $state(false);
	activeRole = $state<Role>('jefe');
	activeTab = $state<NavTab>('panel');
	currentUserName = $state('Carlos Mendoza');

	stores = $state<StoreRecord[]>(structuredClone(INITIAL_STORES));
	users = $state<UserRecord[]>(structuredClone(INITIAL_USERS));
	stock = $state<StockItem[]>(structuredClone(INITIAL_STOCK));
	orders = $state<OrderRecord[]>(structuredClone(INITIAL_ORDERS));
	stops = $state<DeliveryStop[]>(structuredClone(INITIAL_STOPS));
	batches = $state<ProductionBatch[]>(structuredClone(INITIAL_BATCHES));
	expenses = $state<ExpenseRecord[]>(structuredClone(INITIAL_EXPENSES));
	notices = $state<NoticeRecord[]>(structuredClone(INITIAL_NOTICES));

	selectedStoreId = $state<string | null>(null);
	selectedUserId = $state<string | null>(null);
	toastMessage = $state<string | null>(null);
	private toastTimer: ReturnType<typeof setTimeout> | null = null;

	lowStockItems = $derived(this.stock.filter((item) => item.cantidad < item.minimo));

	unreadNoticesCount = $derived(
		this.notices.filter(
			(n) => !n.leido && (n.rolDestino === 'todos' || n.rolDestino === this.activeRole)
		).length
	);

	pendingStopsCount = $derived(this.stops.filter((s) => s.estado !== 'entregado').length);

	activeOrdersCount = $derived(this.orders.filter((o) => o.estado !== 'entregado').length);

	navItems = $derived.by((): NavItem[] => {
		if (this.activeRole === 'jefe') {
			return [
				{ id: 'panel', label: 'Panel', icon: 'home' },
				{ id: 'stock', label: 'Stock', icon: 'boxes', badge: this.lowStockItems.length || undefined },
				{ id: 'mapa', label: 'Mapa', icon: 'map', badge: this.pendingStopsCount || undefined },
				{ id: 'pedidos', label: 'Pedidos', icon: 'receipt', badge: this.activeOrdersCount || undefined },
				{ id: 'mas', label: 'Más', icon: 'grid' }
			];
		}
		if (this.activeRole === 'repartidor') {
			return [
				{ id: 'ruta', label: 'Ruta', icon: 'truck', badge: this.pendingStopsCount || undefined },
				{ id: 'pedidos', label: 'Pedidos', icon: 'receipt', badge: this.activeOrdersCount || undefined },
				{ id: 'gastos', label: 'Gastos', icon: 'wallet' },
				{ id: 'avisos', label: 'Avisos', icon: 'bell', badge: this.unreadNoticesCount || undefined }
			];
		}
		return [
			{ id: 'produccion', label: 'Producción', icon: 'flame' },
			{ id: 'stock', label: 'Stock', icon: 'boxes', badge: this.lowStockItems.length || undefined },
			{ id: 'pedidos', label: 'Pedidos', icon: 'receipt', badge: this.activeOrdersCount || undefined },
			{ id: 'avisos', label: 'Avisos', icon: 'bell', badge: this.unreadNoticesCount || undefined }
		];
	});

	showToast(message: string) {
		this.toastMessage = message;
		if (this.toastTimer) clearTimeout(this.toastTimer);
		this.toastTimer = setTimeout(() => {
			this.toastMessage = null;
		}, 2800);
	}

	loginWithRole(role: Role, identifier?: string) {
		this.activeRole = role;
		this.activeTab = DEFAULT_TAB_BY_ROLE[role];
		const match = this.users.find((u) => u.rol === role);
		this.currentUserName = identifier?.trim()
			? identifier.trim()
			: (match?.nombre ?? 'Usuario Demo');
		this.authenticated = true;
		this.selectedStoreId = null;
		this.selectedUserId = null;
	}

	switchDemoRole(role: Role) {
		this.activeRole = role;
		this.activeTab = DEFAULT_TAB_BY_ROLE[role];
		const match = this.users.find((u) => u.rol === role);
		if (match) this.currentUserName = match.nombre;
		this.selectedStoreId = null;
		this.selectedUserId = null;
		const label =
			role === 'jefe' ? 'Jefe' : role === 'repartidor' ? 'Repartidor' : 'Producción';
		this.showToast(`Vista cambiada al rol ${label}`);
	}

	logout() {
		this.authenticated = false;
		this.selectedStoreId = null;
		this.selectedUserId = null;
	}

	navigate(tab: NavTab) {
		this.activeTab = tab;
	}

	openStoreDetail(storeId: string) {
		this.selectedStoreId = storeId;
		this.activeTab = 'tiendas';
	}

	openUserDetail(userId: string) {
		this.selectedUserId = userId;
		this.activeTab = 'usuarios';
	}

	adjustStock(itemId: string, delta: number) {
		const target = this.stock.find((s) => s.id === itemId);
		if (!target) return;
		target.cantidad = Math.max(0, target.cantidad + delta);
		target.actualizadoHace = 'Ahora mismo';
		this.showToast(`${target.nombre}: ${target.cantidad} ${target.unidad}`);
	}

	advanceOrderStatus(orderId: string) {
		const order = this.orders.find((o) => o.id === orderId);
		if (!order) return;
		const flow: Record<OrderStatus, OrderStatus> = {
			pendiente: 'en_horno',
			en_horno: 'en_reparto',
			en_reparto: 'entregado',
			entregado: 'entregado'
		};
		order.estado = flow[order.estado];
		this.showToast(`Pedido ${order.codigo} actualizado`);
	}

	createOrder(payload: {
		tiendaId: string;
		itemsResumen: string;
		cajas: number;
		total: number;
		horaEntrega: string;
		cobroContraEntrega: boolean;
	}) {
		const store = this.stores.find((s) => s.id === payload.tiendaId);
		if (!store) return;
		const nextNum = 1051 + this.orders.length;
		const newOrder: OrderRecord = {
			id: `o-${Date.now()}`,
			codigo: `#PED-${nextNum}`,
			tiendaId: store.id,
			tiendaNombre: store.nombre,
			zona: store.zona,
			horaEntrega: payload.horaEntrega,
			itemsResumen: payload.itemsResumen,
			cajas: payload.cajas,
			total: payload.total,
			estado: 'pendiente',
			repartidorNombre: 'Lucía Navarro',
			cobroContraEntrega: payload.cobroContraEntrega
		};
		this.orders = [newOrder, ...this.orders];
		store.historialPedidos = [
			{
				id: `so-${Date.now()}`,
				fecha: `Hoy · ${payload.horaEntrega}`,
				resumen: payload.itemsResumen,
				total: payload.total,
				estado: 'pendiente'
			},
			...store.historialPedidos
		];
		this.showToast(`Pedido ${newOrder.codigo} creado para ${store.nombre}`);
	}

	completeStop(stopId: string) {
		const stop = this.stops.find((s) => s.id === stopId);
		if (!stop) return;
		stop.estado = 'entregado';
		const nextPending = this.stops.find((s) => s.estado === 'pendiente');
		if (nextPending) nextPending.estado = 'en_camino';
		this.showToast(`Entrega completada en ${stop.tiendaNombre}`);
	}

	completeBatch(batchId: string) {
		const batch = this.batches.find((b) => b.id === batchId);
		if (!batch) return;
		batch.completadasUnidades = batch.objetivoUnidades;
		batch.minutosRestantes = 0;
		batch.estado = 'listo';
		this.showToast(`${batch.lote} marcado como listo`);
	}

	addExpense(payload: {
		concepto: string;
		categoria: ExpenseRecord['categoria'];
		importe: number;
		vehiculo: string;
	}) {
		const now = new Date();
		const hora = `${String(now.getHours()).padStart(2, '0')}:${String(now.getMinutes()).padStart(2, '0')}`;
		this.expenses = [
			{
				id: `g-${Date.now()}`,
				concepto: payload.concepto,
				categoria: payload.categoria,
				importe: payload.importe,
				hora,
				vehiculo: payload.vehiculo
			},
			...this.expenses
		];
		this.showToast(`Gasto registrado: ${payload.concepto}`);
	}

	markNoticeRead(noticeId: string) {
		const notice = this.notices.find((n) => n.id === noticeId);
		if (!notice) return;
		notice.leido = true;
		this.showToast('Aviso marcado como leído');
	}

	upsertStore(data: Omit<StoreRecord, 'id' | 'historialPedidos'>, existingId?: string) {
		if (existingId) {
			const idx = this.stores.findIndex((s) => s.id === existingId);
			if (idx !== -1) {
				this.stores[idx] = {
					...this.stores[idx],
					...data
				};
				this.showToast(`Ficha actualizada: ${data.nombre}`);
				return this.stores[idx].id;
			}
		}
		const newId = `t-${Date.now()}`;
		const created: StoreRecord = {
			id: newId,
			...data,
			historialPedidos: []
		};
		this.stores = [created, ...this.stores];
		this.showToast(`Tienda añadida: ${data.nombre}`);
		return newId;
	}

	registerStorePayment(storeId: string, amount: number) {
		const store = this.stores.find((s) => s.id === storeId);
		if (!store) return;
		store.saldoPendiente = Math.max(0, Number((store.saldoPendiente - amount).toFixed(2)));
		this.showToast(`Cobro registrado en ${store.nombre}`);
	}

	upsertUser(data: Omit<UserRecord, 'id' | 'historial' | 'ultimaActividad'>, existingId?: string) {
		if (existingId) {
			const idx = this.users.findIndex((u) => u.id === existingId);
			if (idx !== -1) {
				this.users[idx] = {
					...this.users[idx],
					...data,
					ultimaActividad: 'Ahora mismo · Ficha actualizada'
				};
				this.showToast(`Usuario actualizado: ${data.nombre}`);
				return this.users[idx].id;
			}
		}
		const newId = `u-${Date.now()}`;
		const created: UserRecord = {
			id: newId,
			...data,
			ultimaActividad: 'Ahora mismo · Alta en sistema',
			historial: [
				{
					id: `h-${Date.now()}`,
					hora: 'Ahora',
					descripcion: `Alta de usuario como ${data.rol}`
				}
			]
		};
		this.users = [created, ...this.users];
		this.showToast(`Usuario creado: ${data.nombre}`);
		return newId;
	}
}

export const appStore = new AppStore();

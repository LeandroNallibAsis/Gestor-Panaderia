import type { OrderStatus, Role, UserStatus } from '$lib/types';

export function formatCurrency(value: number): string {
	return new Intl.NumberFormat('es-ES', {
		style: 'currency',
		currency: 'EUR',
		minimumFractionDigits: 2
	}).format(value);
}

export function getOrderStatusMeta(status: OrderStatus): {
	label: string;
	badgeClass: string;
	nextLabel?: string;
} {
	switch (status) {
		case 'pendiente':
			return {
				label: 'Pendiente',
				badgeClass: 'bg-amber-500/15 text-amber-300 border-amber-500/40',
				nextLabel: 'Pasar a horno'
			};
		case 'en_horno':
			return {
				label: 'En horno',
				badgeClass: 'bg-orange-500/15 text-orange-300 border-orange-500/40',
				nextLabel: 'Enviar a reparto'
			};
		case 'en_reparto':
			return {
				label: 'En reparto',
				badgeClass: 'bg-sky-500/15 text-sky-300 border-sky-500/40',
				nextLabel: 'Marcar entregado'
			};
		case 'entregado':
			return {
				label: 'Entregado',
				badgeClass: 'bg-emerald-500/15 text-emerald-300 border-emerald-500/40'
			};
	}
}

export function getRoleMeta(role: Role): {
	label: string;
	shortLabel: string;
	badgeClass: string;
} {
	switch (role) {
		case 'jefe':
			return {
				label: 'Jefe / Encargado',
				shortLabel: 'Jefe',
				badgeClass: 'bg-amber-500/20 text-amber-300 border-amber-400/50'
			};
		case 'repartidor':
			return {
				label: 'Repartidor',
				shortLabel: 'Repartidor',
				badgeClass: 'bg-sky-500/20 text-sky-300 border-sky-400/50'
			};
		case 'produccion':
			return {
				label: 'Producción / Obrador',
				shortLabel: 'Producción',
				badgeClass: 'bg-orange-500/20 text-orange-300 border-orange-400/50'
			};
	}
}

export function getUserStatusMeta(status: UserStatus): {
	label: string;
	dotClass: string;
	badgeClass: string;
} {
	switch (status) {
		case 'activo':
			return {
				label: 'Activo',
				dotClass: 'bg-emerald-400',
				badgeClass: 'bg-emerald-500/15 text-emerald-300 border-emerald-500/40'
			};
		case 'en_ruta':
			return {
				label: 'En ruta',
				dotClass: 'bg-sky-400',
				badgeClass: 'bg-sky-500/15 text-sky-300 border-sky-500/40'
			};
		case 'obrador':
			return {
				label: 'En obrador',
				dotClass: 'bg-amber-400',
				badgeClass: 'bg-amber-500/15 text-amber-300 border-amber-500/40'
			};
		case 'descanso':
			return {
				label: 'Descanso',
				dotClass: 'bg-slate-400',
				badgeClass: 'bg-slate-500/20 text-slate-300 border-slate-500/40'
			};
	}
}

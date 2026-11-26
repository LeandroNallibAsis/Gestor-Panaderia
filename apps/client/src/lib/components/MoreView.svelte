<script lang="ts">
	import { appStore } from '$lib/stores/app.svelte';
	import { formatCurrency } from '$lib/utils';
	import Icon from './Icon.svelte';

	const totalCobrosPendientes = $derived(
		appStore.stores.reduce((acc, s) => acc + s.saldoPendiente, 0)
	);
	const totalGastosRuta = $derived(
		appStore.expenses.reduce((acc, e) => acc + e.importe, 0)
	);
	const totalPedidosHoy = $derived(
		appStore.orders.reduce((acc, o) => acc + o.total, 0)
	);
</script>

<div class="space-y-5">
	<div>
		<h1 class="text-2xl font-extrabold text-slate-50">Gestión y Fichas</h1>
		<p class="text-xs text-slate-400">Acceso directo a maestros, finanzas y recordatorios</p>
	</div>

	<div class="grid gap-3">
		<button
			type="button"
			onclick={() => appStore.navigate('tiendas')}
			class="flex min-h-[72px] items-center justify-between gap-3 rounded-2xl border border-slate-700 bg-[#14181F] p-4 text-left transition active:scale-[0.99] hover:border-amber-500/60"
		>
			<div class="flex items-center gap-3.5">
				<div class="flex h-12 w-12 items-center justify-center rounded-xl bg-amber-500/15 text-amber-400">
					<Icon name="store" class="h-6 w-6" />
				</div>
				<div>
					<p class="text-base font-extrabold text-slate-50">Fichas de Tiendas</p>
					<p class="text-xs text-slate-400">
						{appStore.stores.length} clientes · Saldos, horarios e historial
					</p>
				</div>
			</div>
			<Icon name="chevron-right" class="h-5 w-5 text-slate-400" />
		</button>

		<button
			type="button"
			onclick={() => appStore.navigate('usuarios')}
			class="flex min-h-[72px] items-center justify-between gap-3 rounded-2xl border border-slate-700 bg-[#14181F] p-4 text-left transition active:scale-[0.99] hover:border-amber-500/60"
		>
			<div class="flex items-center gap-3.5">
				<div class="flex h-12 w-12 items-center justify-center rounded-xl bg-sky-500/15 text-sky-400">
					<Icon name="users" class="h-6 w-6" />
				</div>
				<div>
					<p class="text-base font-extrabold text-slate-50">Fichas de Usuarios</p>
					<p class="text-xs text-slate-400">
						{appStore.users.length} empleados · Roles, vehículos y actividad
					</p>
				</div>
			</div>
			<Icon name="chevron-right" class="h-5 w-5 text-slate-400" />
		</button>

		<button
			type="button"
			onclick={() => appStore.navigate('avisos')}
			class="flex min-h-[72px] items-center justify-between gap-3 rounded-2xl border border-slate-700 bg-[#14181F] p-4 text-left transition active:scale-[0.99] hover:border-amber-500/60"
		>
			<div class="flex items-center gap-3.5">
				<div class="flex h-12 w-12 items-center justify-center rounded-xl bg-red-500/15 text-red-400">
					<Icon name="bell" class="h-6 w-6" />
				</div>
				<div>
					<p class="text-base font-extrabold text-slate-50">Recordatorios y Avisos</p>
					<p class="text-xs text-slate-400">
						{appStore.unreadNoticesCount} avisos sin leer
					</p>
				</div>
			</div>
			<Icon name="chevron-right" class="h-5 w-5 text-slate-400" />
		</button>
	</div>

	<!-- Resumen de Finanzas -->
	<section class="rounded-2xl border border-slate-800 bg-[#14181F] p-4 space-y-3">
		<div class="flex items-center gap-2">
			<Icon name="euro" class="h-5 w-5 text-emerald-400" />
			<h2 class="text-sm font-bold uppercase tracking-wider text-slate-200">
				Finanzas rápidas del día
			</h2>
		</div>

		<div class="grid grid-cols-3 gap-2 text-center">
			<div class="rounded-xl bg-[#0B0D10] p-3">
				<p class="text-[11px] text-slate-400">Ventas</p>
				<p class="mt-1 text-sm font-extrabold text-emerald-400 tabular-nums">
					{formatCurrency(totalPedidosHoy)}
				</p>
			</div>
			<div class="rounded-xl bg-[#0B0D10] p-3">
				<p class="text-[11px] text-slate-400">Por cobrar</p>
				<p class="mt-1 text-sm font-extrabold text-amber-400 tabular-nums">
					{formatCurrency(totalCobrosPendientes)}
				</p>
			</div>
			<div class="rounded-xl bg-[#0B0D10] p-3">
				<p class="text-[11px] text-slate-400">Gastos ruta</p>
				<p class="mt-1 text-sm font-extrabold text-red-400 tabular-nums">
					{formatCurrency(totalGastosRuta)}
				</p>
			</div>
		</div>
	</section>
</div>

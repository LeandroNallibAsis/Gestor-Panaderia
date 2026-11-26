<script lang="ts">
	import { appStore } from '$lib/stores/app.svelte';
	import { formatCurrency } from '$lib/utils';
	import Icon from './Icon.svelte';

	const totalPendingBalance = $derived(
		appStore.stores.reduce((acc, s) => acc + s.saldoPendiente, 0)
	);
	const totalSalesToday = $derived(
		appStore.orders.reduce((acc, o) => acc + o.total, 0)
	);
	const completedStops = $derived(
		appStore.stops.filter((s) => s.estado === 'entregado').length
	);
	const nextStop = $derived(
		appStore.stops.find((s) => s.estado === 'en_camino') ??
			appStore.stops.find((s) => s.estado === 'pendiente')
	);
	const activeBatches = $derived(
		appStore.batches.filter((b) => b.estado !== 'listo')
	);
</script>

<div class="space-y-5">
	<!-- Título de bienvenida según rol -->
	<section class="flex items-center justify-between gap-3">
		<div>
			<p class="text-xs font-bold uppercase tracking-wider text-amber-400">
				{#if appStore.activeRole === 'jefe'}
					Dirección de Obrador y Reparto
				{:else if appStore.activeRole === 'repartidor'}
					Turno de Reparto en Curso
				{:else}
					Control de Obrador y Hornos
				{/if}
			</p>
			<h1 class="mt-0.5 text-2xl font-extrabold tracking-tight text-slate-50">
				{#if appStore.activeRole === 'jefe'}
					Panel de Control
				{:else if appStore.activeRole === 'repartidor'}
					Resumen de Ruta
				{:else}
					Panel de Producción
				{/if}
			</h1>
		</div>
		<span
			class="rounded-xl border border-slate-700 bg-[#14181F] px-3 py-2 text-xs font-bold text-slate-200 tabular-nums"
		>
			Hoy · 07:50
		</span>
	</section>

	<!-- KPIs por rol -->
	<section class="grid grid-cols-2 gap-3">
		{#if appStore.activeRole === 'jefe'}
			<div class="rounded-2xl border border-slate-800 bg-[#14181F] p-4">
				<p class="text-xs font-semibold text-slate-400">Facturación del día</p>
				<p class="mt-1.5 text-2xl font-extrabold text-slate-50 tabular-nums">
					{formatCurrency(totalSalesToday)}
				</p>
				<p class="mt-1 text-xs font-medium text-emerald-400">+12% frente a ayer</p>
			</div>
			<div class="rounded-2xl border border-slate-800 bg-[#14181F] p-4">
				<p class="text-xs font-semibold text-slate-400">Pedidos activos</p>
				<p class="mt-1.5 text-2xl font-extrabold text-slate-50 tabular-nums">
					{appStore.activeOrdersCount}
					<span class="text-sm font-semibold text-slate-400">/ {appStore.orders.length}</span>
				</p>
				<p class="mt-1 text-xs font-medium text-sky-400">
					{appStore.pendingStopsCount} paradas pendientes
				</p>
			</div>
			<div class="rounded-2xl border border-slate-800 bg-[#14181F] p-4">
				<p class="text-xs font-semibold text-slate-400">Stock bajo mínimo</p>
				<p
					class="mt-1.5 text-2xl font-extrabold tabular-nums {appStore.lowStockItems.length > 0
						? 'text-red-400'
						: 'text-emerald-400'}"
				>
					{appStore.lowStockItems.length} artículos
				</p>
				<p class="mt-1 text-xs font-medium text-slate-400">Revisar almacén e ingredientes</p>
			</div>
			<div class="rounded-2xl border border-slate-800 bg-[#14181F] p-4">
				<p class="text-xs font-semibold text-slate-400">Saldo pendiente tiendas</p>
				<p class="mt-1.5 text-2xl font-extrabold text-amber-400 tabular-nums">
					{formatCurrency(totalPendingBalance)}
				</p>
				<p class="mt-1 text-xs font-medium text-slate-400">{appStore.stores.length} tiendas activas</p>
			</div>
		{:else if appStore.activeRole === 'repartidor'}
			<div class="rounded-2xl border border-slate-800 bg-[#14181F] p-4">
				<p class="text-xs font-semibold text-slate-400">Paradas completadas</p>
				<p class="mt-1.5 text-2xl font-extrabold text-emerald-400 tabular-nums">
					{completedStops}
					<span class="text-sm text-slate-400">/ {appStore.stops.length}</span>
				</p>
				<p class="mt-1 text-xs text-slate-400">Ruta Zona Centro - Sur</p>
			</div>
			<div class="rounded-2xl border border-slate-800 bg-[#14181F] p-4">
				<p class="text-xs font-semibold text-slate-400">Cajas por entregar</p>
				<p class="mt-1.5 text-2xl font-extrabold text-amber-400 tabular-nums">
					{appStore.stops
						.filter((s) => s.estado !== 'entregado')
						.reduce((a, b) => a + b.cajas, 0)} cajas
				</p>
				<p class="mt-1 text-xs text-slate-400">Furgón 4821-KLP</p>
			</div>
		{:else}
			<div class="rounded-2xl border border-slate-800 bg-[#14181F] p-4">
				<p class="text-xs font-semibold text-slate-400">Hornadas activas</p>
				<p class="mt-1.5 text-2xl font-extrabold text-amber-400 tabular-nums">
					{activeBatches.length} lotes
				</p>
				<p class="mt-1 text-xs text-slate-400">Rotativo 1 y Solera 2</p>
			</div>
			<div class="rounded-2xl border border-slate-800 bg-[#14181F] p-4">
				<p class="text-xs font-semibold text-slate-400">Alertas de ingrediente</p>
				<p class="mt-1.5 text-2xl font-extrabold text-red-400 tabular-nums">
					{appStore.lowStockItems.length} críticos
				</p>
				<p class="mt-1 text-xs text-slate-400">Harina T55 y Mantequilla</p>
			</div>
		{/if}
	</section>

	<!-- Próxima parada (repartidor) -->
	{#if appStore.activeRole === 'repartidor' && nextStop}
		<section class="rounded-2xl border border-sky-500/40 bg-sky-500/10 p-4">
			<div class="flex items-start justify-between gap-2">
				<div>
					<span class="text-xs font-bold uppercase tracking-wider text-sky-300">
						Siguiente entrega · Parada #{nextStop.orden}
					</span>
					<h2 class="mt-1 text-lg font-extrabold text-slate-50">{nextStop.tiendaNombre}</h2>
					<p class="text-sm text-slate-300">{nextStop.direccion} · {nextStop.ventanaHoraria}</p>
				</div>
				<span class="rounded-xl bg-sky-500/20 px-2.5 py-1 text-xs font-bold text-sky-200 tabular-nums">
					{nextStop.cajas} cajas
				</span>
			</div>
			<div class="mt-4 grid grid-cols-2 gap-2.5">
				<button
					type="button"
					onclick={() => appStore.completeStop(nextStop.id)}
					class="flex min-h-[48px] items-center justify-center gap-2 rounded-xl bg-emerald-500 px-4 text-sm font-extrabold text-slate-950 transition active:scale-95"
				>
					<Icon name="check" class="h-5 w-5" />
					<span>Marcar entregada</span>
				</button>
				<button
					type="button"
					onclick={() => appStore.openStoreDetail(nextStop.tiendaId)}
					class="flex min-h-[48px] items-center justify-center gap-2 rounded-xl border border-slate-700 bg-[#14181F] px-4 text-sm font-bold text-slate-100 transition active:scale-95"
				>
					<Icon name="store" class="h-5 w-5 text-amber-400" />
					<span>Ver ficha tienda</span>
				</button>
			</div>
		</section>
	{/if}

	<!-- Accesos rápidos en 1 toque -->
	<section class="space-y-2.5">
		<h2 class="text-sm font-bold uppercase tracking-wider text-slate-300">
			Accesos rápidos (1 toque)
		</h2>

		<div class="grid grid-cols-2 gap-2.5">
			{#if appStore.activeRole === 'jefe'}
				<button
					type="button"
					onclick={() => appStore.navigate('tiendas')}
					class="flex min-h-[56px] items-center gap-3 rounded-2xl border border-slate-700 bg-[#14181F] px-4 py-3 text-left transition active:scale-95 hover:border-amber-500/60"
				>
					<div class="flex h-10 w-10 shrink-0 items-center justify-center rounded-xl bg-amber-500/15 text-amber-400">
						<Icon name="store" class="h-5 w-5" />
					</div>
					<div>
						<p class="text-sm font-bold text-slate-50">Fichas Tiendas</p>
						<p class="text-xs text-slate-400">{appStore.stores.length} clientes</p>
					</div>
				</button>

				<button
					type="button"
					onclick={() => appStore.navigate('usuarios')}
					class="flex min-h-[56px] items-center gap-3 rounded-2xl border border-slate-700 bg-[#14181F] px-4 py-3 text-left transition active:scale-95 hover:border-amber-500/60"
				>
					<div class="flex h-10 w-10 shrink-0 items-center justify-center rounded-xl bg-sky-500/15 text-sky-400">
						<Icon name="users" class="h-5 w-5" />
					</div>
					<div>
						<p class="text-sm font-bold text-slate-50">Fichas Equipo</p>
						<p class="text-xs text-slate-400">{appStore.users.length} empleados</p>
					</div>
				</button>

				<button
					type="button"
					onclick={() => appStore.navigate('pedidos')}
					class="flex min-h-[56px] items-center gap-3 rounded-2xl border border-slate-700 bg-[#14181F] px-4 py-3 text-left transition active:scale-95 hover:border-amber-500/60"
				>
					<div class="flex h-10 w-10 shrink-0 items-center justify-center rounded-xl bg-emerald-500/15 text-emerald-400">
						<Icon name="receipt" class="h-5 w-5" />
					</div>
					<div>
						<p class="text-sm font-bold text-slate-50">Pedidos hoy</p>
						<p class="text-xs text-slate-400">{appStore.activeOrdersCount} activos</p>
					</div>
				</button>

				<button
					type="button"
					onclick={() => appStore.navigate('stock')}
					class="flex min-h-[56px] items-center gap-3 rounded-2xl border border-slate-700 bg-[#14181F] px-4 py-3 text-left transition active:scale-95 hover:border-amber-500/60"
				>
					<div class="flex h-10 w-10 shrink-0 items-center justify-center rounded-xl bg-red-500/15 text-red-400">
						<Icon name="boxes" class="h-5 w-5" />
					</div>
					<div>
						<p class="text-sm font-bold text-slate-50">Control Stock</p>
						<p class="text-xs text-slate-400">{appStore.lowStockItems.length} alertas</p>
					</div>
				</button>
			{:else if appStore.activeRole === 'repartidor'}
				<button
					type="button"
					onclick={() => appStore.navigate('pedidos')}
					class="flex min-h-[56px] items-center gap-3 rounded-2xl border border-slate-700 bg-[#14181F] px-4 py-3 text-left transition active:scale-95"
				>
					<Icon name="receipt" class="h-6 w-6 text-amber-400" />
					<div>
						<p class="text-sm font-bold text-slate-50">Albaranes</p>
						<p class="text-xs text-slate-400">{appStore.activeOrdersCount} pendientes</p>
					</div>
				</button>
				<button
					type="button"
					onclick={() => appStore.navigate('gastos')}
					class="flex min-h-[56px] items-center gap-3 rounded-2xl border border-slate-700 bg-[#14181F] px-4 py-3 text-left transition active:scale-95"
				>
					<Icon name="wallet" class="h-6 w-6 text-sky-400" />
					<div>
						<p class="text-sm font-bold text-slate-50">Añadir gasto</p>
						<p class="text-xs text-slate-400">Diésel o parking</p>
					</div>
				</button>
			{:else}
				<button
					type="button"
					onclick={() => appStore.navigate('stock')}
					class="flex min-h-[56px] items-center gap-3 rounded-2xl border border-slate-700 bg-[#14181F] px-4 py-3 text-left transition active:scale-95"
				>
					<Icon name="boxes" class="h-6 w-6 text-amber-400" />
					<div>
						<p class="text-sm font-bold text-slate-50">Ajustar Stock</p>
						<p class="text-xs text-slate-400">Harina y producto</p>
					</div>
				</button>
				<button
					type="button"
					onclick={() => appStore.navigate('pedidos')}
					class="flex min-h-[56px] items-center gap-3 rounded-2xl border border-slate-700 bg-[#14181F] px-4 py-3 text-left transition active:scale-95"
				>
					<Icon name="receipt" class="h-6 w-6 text-emerald-400" />
					<div>
						<p class="text-sm font-bold text-slate-50">Pedidos horno</p>
						<p class="text-xs text-slate-400">Ver cola salida</p>
					</div>
				</button>
			{/if}
		</div>
	</section>

	<!-- Alertas de stock bajo mínimo -->
	<section class="space-y-2.5">
		<div class="flex items-center justify-between">
			<h2 class="flex items-center gap-2 text-sm font-bold uppercase tracking-wider text-red-300">
				<Icon name="alert" class="h-4 w-4 text-red-400" />
				<span>Alertas de stock bajo mínimo</span>
			</h2>
			<button
				type="button"
				onclick={() => appStore.navigate('stock')}
				class="min-h-[40px] rounded-lg px-2 text-xs font-bold text-amber-400 hover:bg-slate-800"
			>
				Ver todo
			</button>
		</div>

		{#if appStore.lowStockItems.length === 0}
			<div class="rounded-2xl border border-emerald-500/30 bg-emerald-500/10 p-4 text-sm text-emerald-200">
				Todo el inventario está por encima del mínimo operativo.
			</div>
		{:else}
			<div class="space-y-2">
				{#each appStore.lowStockItems as item}
					<div
						class="flex items-center justify-between gap-3 rounded-2xl border border-red-500/35 bg-red-950/25 p-3.5"
					>
						<div class="min-w-0">
							<p class="truncate text-sm font-bold text-slate-50">{item.nombre}</p>
							<p class="text-xs text-red-300 tabular-nums">
								Actual: <strong>{item.cantidad} {item.unidad}</strong> · Mínimo: {item.minimo} {item.unidad}
							</p>
						</div>
						<button
							type="button"
							onclick={() => appStore.adjustStock(item.id, 5)}
							class="min-h-[48px] shrink-0 rounded-xl bg-amber-500 px-3.5 text-xs font-extrabold text-slate-950 transition active:scale-95"
						>
							+5 {item.unidad}
						</button>
					</div>
				{/each}
			</div>
		{/if}
	</section>
</div>

<script lang="ts">
	import { appStore } from '$lib/stores/app.svelte';
	import { formatCurrency } from '$lib/utils';
	import Icon from './Icon.svelte';

	const completedCount = $derived(appStore.stops.filter((s) => s.estado === 'entregado').length);
</script>

<div class="space-y-4">
	<div class="flex items-center justify-between gap-2">
		<div>
			<h1 class="text-2xl font-extrabold text-slate-50">
				{appStore.activeRole === 'repartidor' ? 'Mi Ruta de Hoy' : 'Mapa y Rutas'}
			</h1>
			<p class="text-xs text-slate-400">
				{completedCount} de {appStore.stops.length} paradas completadas · Furgón 4821-KLP
			</p>
		</div>
	</div>

	<!-- Esquema visual de ruta de reparto -->
	<section class="rounded-2xl border border-slate-800 bg-[#14181F] p-4">
		<div class="mb-2 flex items-center justify-between text-xs font-bold text-slate-300">
			<span>Recorrido Zona Norte → Centro → Sur → Este</span>
			<span class="text-amber-400 tabular-nums">12,0 km totales</span>
		</div>
		<div class="flex items-center gap-1.5 overflow-x-auto py-2">
			{#each appStore.stops as stop, idx}
				<div class="flex items-center gap-1.5 shrink-0">
					<div
						class="flex h-9 w-9 items-center justify-center rounded-full text-xs font-extrabold tabular-nums {stop.estado ===
						'entregado'
							? 'bg-emerald-500 text-slate-950'
							: stop.estado === 'en_camino'
								? 'bg-amber-500 text-slate-950 ring-4 ring-amber-500/25'
								: 'bg-slate-800 text-slate-300'}"
					>
						{stop.orden}
					</div>
					{#if idx < appStore.stops.length - 1}
						<div class="h-1 w-6 rounded-full {stop.estado === 'entregado' ? 'bg-emerald-500' : 'bg-slate-700'}"></div>
					{/if}
				</div>
			{/each}
		</div>
	</section>

	<!-- Lista de paradas -->
	<div class="space-y-3">
		{#each appStore.stops as stop}
			<div
				class="rounded-2xl border p-4 space-y-3 {stop.estado === 'en_camino'
					? 'border-amber-500/60 bg-amber-500/5'
					: 'border-slate-800 bg-[#14181F]'}"
			>
				<div class="flex items-start justify-between gap-2">
					<div class="flex items-start gap-3">
						<span
							class="flex h-9 w-9 shrink-0 items-center justify-center rounded-xl text-sm font-extrabold tabular-nums {stop.estado ===
							'entregado'
								? 'bg-emerald-500/20 text-emerald-300'
								: stop.estado === 'en_camino'
									? 'bg-amber-500 text-slate-950'
									: 'bg-slate-800 text-slate-300'}"
						>
							#{stop.orden}
						</span>
						<div>
							<h2 class="text-base font-extrabold text-slate-50">{stop.tiendaNombre}</h2>
							<p class="text-xs text-slate-300">{stop.direccion} · Zona {stop.zona}</p>
							<p class="mt-0.5 text-xs text-slate-400">
								Ventana: {stop.ventanaHoraria} · {stop.cajas} cajas · {stop.distanciaKm} km
							</p>
						</div>
					</div>

					{#if stop.importeCobrar > 0}
						<span class="shrink-0 rounded-lg border border-amber-500/40 bg-amber-500/15 px-2.5 py-1 text-xs font-extrabold text-amber-300 tabular-nums">
							Cobrar {formatCurrency(stop.importeCobrar)}
						</span>
					{/if}
				</div>

				<div class="grid grid-cols-2 gap-2">
					<button
						type="button"
						onclick={() => appStore.openStoreDetail(stop.tiendaId)}
						class="min-h-[48px] rounded-xl border border-slate-700 bg-[#0B0D10] px-3 text-xs font-bold text-slate-200 active:scale-95"
					>
						Ficha de tienda
					</button>
					{#if stop.estado !== 'entregado'}
						<button
							type="button"
							onclick={() => appStore.completeStop(stop.id)}
							class="flex min-h-[48px] items-center justify-center gap-1.5 rounded-xl bg-emerald-500 px-3 text-xs font-extrabold text-slate-950 active:scale-95"
						>
							<Icon name="check" class="h-4 w-4" />
							<span>Entregada</span>
						</button>
					{:else}
						<div class="flex min-h-[48px] items-center justify-center rounded-xl bg-emerald-500/15 text-xs font-bold text-emerald-300">
							Entrega confirmada
						</div>
					{/if}
				</div>
			</div>
		{/each}
	</div>
</div>

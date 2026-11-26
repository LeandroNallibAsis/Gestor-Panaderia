<script lang="ts">
	import { appStore } from '$lib/stores/app.svelte';
	import Icon from './Icon.svelte';

	let categoryFilter = $state<'todos' | 'materia_prima' | 'producto_terminado' | 'critico'>('todos');

	const filteredStock = $derived(
		appStore.stock.filter((item) => {
			if (categoryFilter === 'critico') return item.cantidad < item.minimo;
			if (categoryFilter === 'todos') return true;
			return item.categoria === categoryFilter;
		})
	);
</script>

<div class="space-y-4">
	<div class="flex items-center justify-between gap-2">
		<div>
			<h1 class="text-2xl font-extrabold text-slate-50">Inventario y Stock</h1>
			<p class="text-xs text-slate-400">
				{appStore.lowStockItems.length} artículos por debajo del mínimo
			</p>
		</div>
	</div>

	<div class="flex gap-2 overflow-x-auto pb-1">
		{#each [
			{ id: 'todos', label: 'Todo' },
			{ id: 'critico', label: `Alertas (${appStore.lowStockItems.length})` },
			{ id: 'materia_prima', label: 'Materias primas' },
			{ id: 'producto_terminado', label: 'Horneado' }
		] as f}
			<button
				type="button"
				onclick={() => (categoryFilter = f.id as typeof categoryFilter)}
				class="min-h-[44px] shrink-0 rounded-xl border px-3.5 text-xs font-bold transition {categoryFilter ===
				f.id
					? 'border-amber-500 bg-amber-500/15 text-amber-300'
					: 'border-slate-800 bg-[#14181F] text-slate-300'}"
			>
				{f.label}
			</button>
		{/each}
	</div>

	<div class="space-y-3">
		{#each filteredStock as item}
			{@const isLow = item.cantidad < item.minimo}
			<div
				class="rounded-2xl border p-4 {isLow
					? 'border-red-500/45 bg-red-950/20'
					: 'border-slate-800 bg-[#14181F]'}"
			>
				<div class="flex items-start justify-between gap-3">
					<div class="min-w-0">
						<div class="flex items-center gap-2">
							<p class="text-base font-extrabold text-slate-50">{item.nombre}</p>
						</div>
						<p class="mt-0.5 text-xs text-slate-400">
							{item.ubicacion} · {item.actualizadoHace}
						</p>
					</div>

					<span
						class="shrink-0 rounded-lg border px-2.5 py-1 text-xs font-bold {isLow
							? 'border-red-500/50 bg-red-500/15 text-red-300'
							: 'border-emerald-500/40 bg-emerald-500/15 text-emerald-300'}"
					>
						{isLow ? 'Bajo mínimo' : 'OK'}
					</span>
				</div>

				<div class="mt-4 flex items-center justify-between gap-3">
					<div>
						<p class="text-2xl font-extrabold text-slate-50 tabular-nums">
							{item.cantidad}
							<span class="text-sm font-semibold text-slate-400">{item.unidad}</span>
						</p>
						<p class="text-xs text-slate-400 tabular-nums">Mínimo: {item.minimo} {item.unidad}</p>
					</div>

					<div class="flex items-center gap-2">
						<button
							type="button"
							onclick={() => appStore.adjustStock(item.id, -1)}
							class="flex h-12 w-12 items-center justify-center rounded-xl border border-slate-700 bg-[#0B0D10] text-slate-100 active:scale-95"
							aria-label="Restar uno a {item.nombre}"
						>
							<Icon name="minus" class="h-5 w-5" />
						</button>
						<button
							type="button"
							onclick={() => appStore.adjustStock(item.id, 1)}
							class="flex h-12 w-12 items-center justify-center rounded-xl bg-amber-500 text-slate-950 active:scale-95"
							aria-label="Sumar uno a {item.nombre}"
						>
							<Icon name="plus" class="h-5 w-5" />
						</button>
					</div>
				</div>
			</div>
		{/each}
	</div>
</div>

<script lang="ts">
	import { appStore } from '$lib/stores/app.svelte';
	import Icon from './Icon.svelte';
</script>

<div class="space-y-4">
	<div>
		<h1 class="text-2xl font-extrabold text-slate-50">Producción y Hornadas</h1>
		<p class="text-xs text-slate-400">Control de lotes activos en obrador</p>
	</div>

	<div class="space-y-3">
		{#each appStore.batches as batch}
			{@const pct = Math.min(100, Math.round((batch.completadasUnidades / batch.objetivoUnidades) * 100))}
			<div class="rounded-2xl border border-slate-800 bg-[#14181F] p-4 space-y-3">
				<div class="flex items-start justify-between gap-2">
					<div>
						<span class="text-xs font-bold uppercase tracking-wider text-amber-400">
							{batch.lote} · {batch.horno}
						</span>
						<h2 class="mt-0.5 text-lg font-extrabold text-slate-50">{batch.producto}</h2>
					</div>
					<span
						class="rounded-lg border px-2.5 py-1 text-xs font-bold {batch.estado === 'listo'
							? 'border-emerald-500/40 bg-emerald-500/15 text-emerald-300'
							: batch.estado === 'horneando'
								? 'border-amber-500/40 bg-amber-500/15 text-amber-300'
								: 'border-sky-500/40 bg-sky-500/15 text-sky-300'}"
					>
						{batch.estado === 'listo'
							? 'Listo'
							: batch.estado === 'horneando'
								? `${batch.minutosRestantes} min · ${batch.temperaturaC} °C`
								: `Fermentando · ${batch.minutosRestantes} min`}
					</span>
				</div>

				<div>
					<div class="mb-1 flex justify-between text-xs font-semibold text-slate-300 tabular-nums">
						<span>Progreso de unidades</span>
						<span>{batch.completadasUnidades} / {batch.objetivoUnidades} uds ({pct}%)</span>
					</div>
					<div class="h-3 w-full overflow-hidden rounded-full bg-slate-800">
						<div
							class="h-full rounded-full transition-all {batch.estado === 'listo'
								? 'bg-emerald-500'
								: 'bg-amber-500'}"
							style="width: {pct}%"
						></div>
					</div>
				</div>

				{#if batch.estado !== 'listo'}
					<button
						type="button"
						onclick={() => appStore.completeBatch(batch.id)}
						class="flex min-h-[48px] w-full items-center justify-center gap-2 rounded-xl bg-amber-500 text-sm font-extrabold text-slate-950 active:scale-[0.98]"
					>
						<Icon name="check" class="h-5 w-5" />
						<span>Marcar hornada completada</span>
					</button>
				{/if}
			</div>
		{/each}
	</div>
</div>

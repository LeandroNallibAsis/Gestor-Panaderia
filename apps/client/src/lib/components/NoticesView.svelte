<script lang="ts">
	import { appStore } from '$lib/stores/app.svelte';
	import Icon from './Icon.svelte';

	const visibleNotices = $derived(
		appStore.notices.filter(
			(n) => n.rolDestino === 'todos' || n.rolDestino === appStore.activeRole || appStore.activeRole === 'jefe'
		)
	);
</script>

<div class="space-y-4">
	<div>
		<h1 class="text-2xl font-extrabold text-slate-50">Avisos y Recordatorios</h1>
		<p class="text-xs text-slate-400">
			{appStore.unreadNoticesCount} avisos pendientes de lectura
		</p>
	</div>

	<div class="space-y-3">
		{#each visibleNotices as notice}
			<div
				class="rounded-2xl border p-4 space-y-3 {notice.leido
					? 'border-slate-800 bg-[#14181F]/70 opacity-80'
					: 'border-amber-500/45 bg-[#14181F]'}"
			>
				<div class="flex items-start justify-between gap-2">
					<div class="flex items-center gap-2">
						<span
							class="rounded-lg px-2 py-0.5 text-[11px] font-bold uppercase {notice.tipo === 'urgente' || notice.tipo === 'stock'
								? 'bg-red-500/20 text-red-300'
								: 'bg-sky-500/20 text-sky-300'}"
						>
							{notice.tipo}
						</span>
						<span class="text-xs text-slate-400">{notice.hora}</span>
					</div>
					{#if !notice.leido}
						<span class="h-2.5 w-2.5 rounded-full bg-amber-400"></span>
					{/if}
				</div>

				<div>
					<h2 class="text-base font-extrabold text-slate-50">{notice.titulo}</h2>
					<p class="mt-1 text-sm text-slate-300">{notice.mensaje}</p>
				</div>

				{#if !notice.leido}
					<button
						type="button"
						onclick={() => appStore.markNoticeRead(notice.id)}
						class="flex min-h-[48px] w-full items-center justify-center gap-2 rounded-xl border border-slate-700 bg-[#0B0D10] text-xs font-bold text-slate-100 active:scale-95"
					>
						<Icon name="check" class="h-4 w-4 text-emerald-400" />
						<span>Marcar como leído</span>
					</button>
				{/if}
			</div>
		{/each}
	</div>
</div>

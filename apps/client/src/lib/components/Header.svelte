<script lang="ts">
	import { appStore } from '$lib/stores/app.svelte';
	import type { Role } from '$lib/types';
	import Icon from './Icon.svelte';

	const roles: { id: Role; label: string }[] = [
		{ id: 'jefe', label: 'Jefe' },
		{ id: 'repartidor', label: 'Repartidor' },
		{ id: 'produccion', label: 'Producción' }
	];
</script>

<header
	class="sticky top-0 z-30 border-b border-slate-800 bg-[#0B0D10]/95 px-4 pb-3 pt-safe backdrop-blur-md"
>
	<div class="mx-auto flex max-w-2xl items-center justify-between gap-3">
		<div class="min-w-0 flex items-center gap-3">
			<div
				class="flex h-11 w-11 shrink-0 items-center justify-center rounded-xl bg-amber-500 text-slate-950 shadow-lg shadow-amber-500/20"
			>
				<Icon name="flame" class="h-6 w-6" />
			</div>
			<div class="min-w-0">
				<p class="truncate text-sm font-bold tracking-tight text-slate-50">
					Panadería El Trigal
				</p>
				<p class="truncate text-xs text-slate-400">
					{appStore.currentUserName}
				</p>
			</div>
		</div>

		<button
			type="button"
			onclick={() => appStore.logout()}
			class="flex min-h-[48px] min-w-[48px] items-center justify-center gap-1.5 rounded-xl border border-slate-700 bg-slate-900 px-3 text-xs font-semibold text-slate-200 transition active:scale-95 hover:border-slate-500"
			title="Cerrar sesión"
			aria-label="Cerrar sesión"
		>
			<Icon name="logout" class="h-4 w-4 text-slate-300" />
			<span class="hidden sm:inline">Salir</span>
		</button>
	</div>

	<!-- Selector rápido de rol demo en cabecera (1 toque) -->
	<div class="mx-auto mt-3 max-w-2xl">
		<div
			class="grid grid-cols-3 gap-1.5 rounded-2xl border border-slate-800 bg-[#14181F] p-1"
			role="group"
			aria-label="Cambiar rol de demostración"
		>
			{#each roles as r}
				{@const active = appStore.activeRole === r.id}
				<button
					type="button"
					onclick={() => appStore.switchDemoRole(r.id)}
					class="min-h-[44px] rounded-xl px-2 text-xs font-bold transition active:scale-95 {active
						? 'bg-amber-500 text-slate-950 shadow-sm'
						: 'text-slate-300 hover:bg-slate-800/80'}"
				>
					{r.label}
				</button>
			{/each}
		</div>
	</div>
</header>

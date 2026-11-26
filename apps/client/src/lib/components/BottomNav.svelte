<script lang="ts">
	import { appStore } from '$lib/stores/app.svelte';
	import type { NavTab } from '$lib/types';
	import Icon from './Icon.svelte';

	function isTabActive(tabId: NavTab): boolean {
		if (appStore.activeTab === tabId) return true;
		if (
			tabId === 'mas' &&
			(appStore.activeTab === 'tiendas' ||
				appStore.activeTab === 'usuarios' ||
				appStore.activeTab === 'finanzas' ||
				appStore.activeTab === 'avisos')
		) {
			return true;
		}
		return false;
	}
</script>

<nav
	class="fixed inset-x-0 bottom-0 z-30 border-t border-slate-800 bg-[#0F131A]/95 pb-safe pt-1.5 backdrop-blur-md"
	aria-label="Navegación principal"
>
	<div
		class="mx-auto grid max-w-2xl gap-1 px-2"
		style="grid-template-columns: repeat({appStore.navItems.length}, minmax(0, 1fr));"
	>
		{#each appStore.navItems as item}
			{@const active = isTabActive(item.id)}
			<button
				type="button"
				onclick={() => appStore.navigate(item.id)}
				class="relative flex min-h-[54px] flex-col items-center justify-center gap-1 rounded-xl px-1 py-1.5 transition active:scale-95 {active
					? 'bg-amber-500/15 text-amber-400'
					: 'text-slate-400 hover:bg-slate-800/60 hover:text-slate-200'}"
				aria-current={active ? 'page' : undefined}
			>
				<div class="relative">
					<Icon name={item.icon} class="h-5 w-5" />
					{#if item.badge && item.badge > 0}
						<span
							class="absolute -right-2.5 -top-1.5 flex h-4 min-w-[16px] items-center justify-center rounded-full bg-red-500 px-1 text-[10px] font-extrabold text-white tabular-nums"
						>
							{item.badge}
						</span>
					{/if}
				</div>
				<span class="truncate text-[11px] font-semibold tracking-tight">{item.label}</span>
			</button>
		{/each}
	</div>
</nav>

<script lang="ts">
	import BottomNav from '$lib/components/BottomNav.svelte';
	import DashboardView from '$lib/components/DashboardView.svelte';
	import ExpensesView from '$lib/components/ExpensesView.svelte';
	import Header from '$lib/components/Header.svelte';
	import LoginView from '$lib/components/LoginView.svelte';
	import MoreView from '$lib/components/MoreView.svelte';
	import NoticesView from '$lib/components/NoticesView.svelte';
	import OrdersView from '$lib/components/OrdersView.svelte';
	import ProductionView from '$lib/components/ProductionView.svelte';
	import RouteMapView from '$lib/components/RouteMapView.svelte';
	import StockView from '$lib/components/StockView.svelte';
	import StoresView from '$lib/components/StoresView.svelte';
	import UsersView from '$lib/components/UsersView.svelte';
	import { appStore } from '$lib/stores/app.svelte';
</script>

{#if !appStore.authenticated}
	<LoginView />
{:else}
	<div class="min-h-dvh bg-[#0B0D10] text-slate-50">
		<Header />

		<main class="mx-auto max-w-2xl px-4 pb-28 pt-5">
			{#if appStore.activeTab === 'panel'}
				<DashboardView />
			{:else if appStore.activeTab === 'stock'}
				<StockView />
			{:else if appStore.activeTab === 'mapa' || appStore.activeTab === 'ruta'}
				<RouteMapView />
			{:else if appStore.activeTab === 'pedidos'}
				<OrdersView />
			{:else if appStore.activeTab === 'produccion'}
				<ProductionView />
			{:else if appStore.activeTab === 'gastos'}
				<ExpensesView />
			{:else if appStore.activeTab === 'avisos'}
				<NoticesView />
			{:else if appStore.activeTab === 'mas' || appStore.activeTab === 'finanzas'}
				<MoreView />
			{:else if appStore.activeTab === 'tiendas'}
				<StoresView />
			{:else if appStore.activeTab === 'usuarios'}
				<UsersView />
			{/if}
		</main>

		<BottomNav />
	</div>
{/if}

{#if appStore.toastMessage}
	<div
		role="status"
		aria-live="polite"
		class="fixed inset-x-4 bottom-24 z-50 mx-auto max-w-md rounded-2xl border border-amber-500/50 bg-[#1B202A] px-4 py-3 text-center text-sm font-bold text-amber-200 shadow-2xl"
	>
		{appStore.toastMessage}
	</div>
{/if}

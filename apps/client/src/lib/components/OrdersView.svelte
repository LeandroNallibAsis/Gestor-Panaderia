<script lang="ts">
	import { appStore } from '$lib/stores/app.svelte';
	import type { OrderStatus } from '$lib/types';
	import { formatCurrency, getOrderStatusMeta } from '$lib/utils';
	import Icon from './Icon.svelte';

	let statusFilter = $state<'todos' | OrderStatus>('todos');
	let modalOpen = $state(false);
	let tiendaId = $state(appStore.stores[0]?.id ?? '');
	let itemsResumen = $state('40 baguettes · 20 croissants');
	let cajas = $state('4');
	let total = $state('88.50');
	let horaEntrega = $state('09:00');
	let cobroContraEntrega = $state(false);

	const filteredOrders = $derived(
		appStore.orders.filter((o) => (statusFilter === 'todos' ? true : o.estado === statusFilter))
	);

	function handleCreate(event: SubmitEvent) {
		event.preventDefault();
		if (!tiendaId || !itemsResumen.trim()) return;
		appStore.createOrder({
			tiendaId,
			itemsResumen: itemsResumen.trim(),
			cajas: Math.max(1, Number(cajas) || 1),
			total: Math.max(1, Number(total) || 10),
			horaEntrega: horaEntrega || '09:00',
			cobroContraEntrega
		});
		modalOpen = false;
	}
</script>

<div class="space-y-4">
	<div class="flex items-center justify-between gap-2">
		<div>
			<h1 class="text-2xl font-extrabold text-slate-50">Pedidos del Día</h1>
			<p class="text-xs text-slate-400">{appStore.activeOrdersCount} en curso hoy</p>
		</div>
		<button
			type="button"
			onclick={() => (modalOpen = true)}
			class="flex min-h-[48px] items-center gap-1.5 rounded-xl bg-amber-500 px-4 text-sm font-extrabold text-slate-950 active:scale-95"
		>
			<Icon name="plus" class="h-5 w-5" />
			<span>Nuevo</span>
		</button>
	</div>

	<div class="flex gap-2 overflow-x-auto pb-1">
		{#each [
			{ id: 'todos', label: 'Todos' },
			{ id: 'pendiente', label: 'Pendiente' },
			{ id: 'en_horno', label: 'En horno' },
			{ id: 'en_reparto', label: 'En reparto' },
			{ id: 'entregado', label: 'Entregados' }
		] as f}
			<button
				type="button"
				onclick={() => (statusFilter = f.id as typeof statusFilter)}
				class="min-h-[44px] shrink-0 rounded-xl border px-3.5 text-xs font-bold transition {statusFilter ===
				f.id
					? 'border-amber-500 bg-amber-500/15 text-amber-300'
					: 'border-slate-800 bg-[#14181F] text-slate-300'}"
			>
				{f.label}
			</button>
		{/each}
	</div>

	<div class="space-y-3">
		{#each filteredOrders as order}
			{@const meta = getOrderStatusMeta(order.estado)}
			<div class="rounded-2xl border border-slate-800 bg-[#14181F] p-4 space-y-3">
				<div class="flex items-start justify-between gap-2">
					<div>
						<div class="flex items-center gap-2">
							<span class="text-xs font-extrabold text-amber-400 tabular-nums">{order.codigo}</span>
							<span class="text-xs text-slate-400">· {order.horaEntrega}</span>
						</div>
						<h2 class="mt-0.5 text-base font-extrabold text-slate-50">{order.tiendaNombre}</h2>
					</div>
					<span class="rounded-lg border px-2.5 py-1 text-[11px] font-bold {meta.badgeClass}">
						{meta.label}
					</span>
				</div>

				<p class="text-sm text-slate-200">{order.itemsResumen}</p>

				<div class="flex items-center justify-between text-xs text-slate-400">
					<span>{order.cajas} cajas · {order.repartidorNombre}</span>
					<span class="text-base font-extrabold text-slate-50 tabular-nums">
						{formatCurrency(order.total)}
					</span>
				</div>

				<div class="grid grid-cols-2 gap-2 pt-1">
					<button
						type="button"
						onclick={() => appStore.openStoreDetail(order.tiendaId)}
						class="min-h-[48px] rounded-xl border border-slate-700 bg-[#0B0D10] px-3 text-xs font-bold text-slate-200 active:scale-95"
					>
						Ver tienda
					</button>
					{#if meta.nextLabel}
						<button
							type="button"
							onclick={() => appStore.advanceOrderStatus(order.id)}
							class="min-h-[48px] rounded-xl bg-amber-500 px-3 text-xs font-extrabold text-slate-950 active:scale-95"
						>
							{meta.nextLabel}
						</button>
					{:else}
						<div
							class="flex min-h-[48px] items-center justify-center rounded-xl border border-emerald-500/30 bg-emerald-500/10 text-xs font-bold text-emerald-300"
						>
							Completado
						</div>
					{/if}
				</div>
			</div>
		{/each}
	</div>
</div>

{#if modalOpen}
	<div class="fixed inset-0 z-50 flex items-end justify-center bg-black/75 sm:items-center sm:p-4">
		<div class="w-full max-w-lg rounded-t-3xl border border-slate-700 bg-[#14181F] p-5 sm:rounded-3xl">
			<div class="mb-4 flex items-center justify-between">
				<h2 class="text-lg font-extrabold text-slate-50">Nuevo pedido rápido</h2>
				<button
					type="button"
					onclick={() => (modalOpen = false)}
					class="flex h-11 w-11 items-center justify-center rounded-xl bg-slate-800 text-slate-300"
				>
					<Icon name="x" class="h-5 w-5" />
				</button>
			</div>

			<form onsubmit={handleCreate} class="space-y-3.5">
				<div>
					<label for="ord-tienda" class="mb-1 block text-xs font-bold text-slate-300">Tienda cliente</label>
					<select
						id="ord-tienda"
						bind:value={tiendaId}
						class="min-h-[48px] w-full rounded-xl border border-slate-700 bg-[#0B0D10] px-3 text-sm text-slate-50"
					>
						{#each appStore.stores as s}
							<option value={s.id}>{s.nombre} ({s.zona})</option>
						{/each}
					</select>
				</div>

				<div>
					<label for="ord-items" class="mb-1 block text-xs font-bold text-slate-300">Detalle de productos</label>
					<input
						id="ord-items"
						type="text"
						bind:value={itemsResumen}
						class="min-h-[48px] w-full rounded-xl border border-slate-700 bg-[#0B0D10] px-3.5 text-sm text-slate-50"
					/>
				</div>

				<div class="grid grid-cols-3 gap-2.5">
					<div>
						<label for="ord-cajas" class="mb-1 block text-xs font-bold text-slate-300">Cajas</label>
						<input
							id="ord-cajas"
							type="number"
							bind:value={cajas}
							class="min-h-[48px] w-full rounded-xl border border-slate-700 bg-[#0B0D10] px-3 text-sm text-slate-50"
						/>
					</div>
					<div>
						<label for="ord-total" class="mb-1 block text-xs font-bold text-slate-300">Total (€)</label>
						<input
							id="ord-total"
							type="number"
							step="0.01"
							bind:value={total}
							class="min-h-[48px] w-full rounded-xl border border-slate-700 bg-[#0B0D10] px-3 text-sm text-slate-50"
						/>
					</div>
					<div>
						<label for="ord-hora" class="mb-1 block text-xs font-bold text-slate-300">Hora</label>
						<input
							id="ord-hora"
							type="text"
							bind:value={horaEntrega}
							class="min-h-[48px] w-full rounded-xl border border-slate-700 bg-[#0B0D10] px-3 text-sm text-slate-50"
						/>
					</div>
				</div>

				<label class="flex min-h-[48px] items-center gap-3 rounded-xl border border-slate-800 bg-[#0B0D10] px-3.5 text-sm text-slate-200">
					<input type="checkbox" bind:checked={cobroContraEntrega} class="h-5 w-5 accent-amber-500" />
					<span>Cobro contra entrega en efectivo / TPV</span>
				</label>

				<button
					type="submit"
					class="min-h-[52px] w-full rounded-xl bg-amber-500 text-sm font-extrabold text-slate-950 active:scale-[0.98]"
				>
					Registrar pedido
				</button>
			</form>
		</div>
	</div>
{/if}

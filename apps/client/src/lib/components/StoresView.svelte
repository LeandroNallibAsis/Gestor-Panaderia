<script lang="ts">
	import { appStore } from '$lib/stores/app.svelte';
	import type { StoreRecord } from '$lib/types';
	import { formatCurrency, getOrderStatusMeta } from '$lib/utils';
	import Icon from './Icon.svelte';

	let searchQuery = $state('');
	let filterMode = $state<'todas' | 'con_saldo'>('todas');
	let modalOpen = $state(false);
	let editingStoreId = $state<string | null>(null);

	let formNombre = $state('');
	let formZona = $state('Centro');
	let formDireccion = $state('');
	let formContacto = $state('');
	let formTelefono = $state('');
	let formHorario = $state('07:30 - 08:15');
	let formSaldo = $state('0');
	let formLimite = $state('600');
	let formNotas = $state('');
	let formError = $state('');

	const filteredStores = $derived(
		appStore.stores.filter((store) => {
			if (filterMode === 'con_saldo' && store.saldoPendiente <= 0) return false;
			const q = searchQuery.trim().toLowerCase();
			if (!q) return true;
			return (
				store.nombre.toLowerCase().includes(q) ||
				store.zona.toLowerCase().includes(q) ||
				store.contacto.toLowerCase().includes(q) ||
				store.direccion.toLowerCase().includes(q)
			);
		})
	);

	const selectedStore = $derived(
		appStore.stores.find((s) => s.id === appStore.selectedStoreId) ?? null
	);

	function openCreateModal() {
		editingStoreId = null;
		formNombre = '';
		formZona = 'Centro';
		formDireccion = '';
		formContacto = '';
		formTelefono = '';
		formHorario = '07:30 - 08:15';
		formSaldo = '0';
		formLimite = '600';
		formNotas = '';
		formError = '';
		modalOpen = true;
	}

	function openEditModal(store: StoreRecord) {
		editingStoreId = store.id;
		formNombre = store.nombre;
		formZona = store.zona;
		formDireccion = store.direccion;
		formContacto = store.contacto;
		formTelefono = store.telefono;
		formHorario = store.horarioEntrega;
		formSaldo = String(store.saldoPendiente);
		formLimite = String(store.limiteCredito);
		formNotas = store.notasEntrega;
		formError = '';
		modalOpen = true;
	}

	function saveStore(event: SubmitEvent) {
		event.preventDefault();
		if (!formNombre.trim() || !formDireccion.trim() || !formContacto.trim()) {
			formError = 'Nombre, dirección y persona de contacto son obligatorios.';
			return;
		}
		const savedId = appStore.upsertStore(
			{
				nombre: formNombre.trim(),
				zona: formZona.trim() || 'Centro',
				direccion: formDireccion.trim(),
				contacto: formContacto.trim(),
				telefono: formTelefono.trim() || 'Sin teléfono',
				horarioEntrega: formHorario.trim() || '07:00 - 08:00',
				saldoPendiente: Math.max(0, Number(formSaldo) || 0),
				limiteCredito: Math.max(100, Number(formLimite) || 600),
				notasEntrega: formNotas.trim() || 'Sin observaciones especiales.',
				activa: true
			},
			editingStoreId ?? undefined
		);
		modalOpen = false;
		appStore.selectedStoreId = savedId;
	}
</script>

{#if selectedStore}
	<!-- Detalle de ficha de tienda -->
	<div class="space-y-4">
		<div class="flex items-center justify-between gap-2">
			<button
				type="button"
				onclick={() => (appStore.selectedStoreId = null)}
				class="flex min-h-[48px] items-center gap-1.5 rounded-xl border border-slate-700 bg-[#14181F] px-3.5 text-sm font-bold text-slate-100 active:scale-95"
			>
				<Icon name="chevron-left" class="h-5 w-5" />
				<span>Tiendas</span>
			</button>

			<button
				type="button"
				onclick={() => openEditModal(selectedStore)}
				class="flex min-h-[48px] items-center gap-2 rounded-xl bg-amber-500 px-4 text-sm font-extrabold text-slate-950 active:scale-95"
			>
				<Icon name="edit" class="h-4 w-4" />
				<span>Editar ficha</span>
			</button>
		</div>

		<section class="rounded-2xl border border-slate-800 bg-[#14181F] p-4 space-y-4">
			<div class="flex items-start justify-between gap-3">
				<div>
					<span class="rounded-lg border border-amber-500/30 bg-amber-500/10 px-2.5 py-0.5 text-xs font-bold text-amber-300">
						Zona {selectedStore.zona}
					</span>
					<h1 class="mt-2 text-xl font-extrabold text-slate-50">{selectedStore.nombre}</h1>
				</div>
				<div class="text-right">
					<p class="text-xs text-slate-400">Saldo pendiente</p>
					<p
						class="text-xl font-extrabold tabular-nums {selectedStore.saldoPendiente > 0
							? 'text-amber-400'
							: 'text-emerald-400'}"
					>
						{formatCurrency(selectedStore.saldoPendiente)}
					</p>
				</div>
			</div>

			<div class="grid gap-2.5 text-sm">
				<div class="flex items-start gap-2.5 rounded-xl bg-[#0B0D10] p-3">
					<Icon name="pin" class="mt-0.5 h-4 w-4 shrink-0 text-amber-400" />
					<div>
						<p class="text-xs text-slate-400">Dirección de entrega</p>
						<p class="font-semibold text-slate-100">{selectedStore.direccion}</p>
					</div>
				</div>

				<div class="flex items-start gap-2.5 rounded-xl bg-[#0B0D10] p-3">
					<Icon name="clock" class="mt-0.5 h-4 w-4 shrink-0 text-sky-400" />
					<div>
						<p class="text-xs text-slate-400">Horario preferente</p>
						<p class="font-semibold text-slate-100">{selectedStore.horarioEntrega}</p>
					</div>
				</div>

				<div class="flex items-center justify-between gap-2 rounded-xl bg-[#0B0D10] p-3">
					<div class="flex items-start gap-2.5">
						<Icon name="phone" class="mt-0.5 h-4 w-4 shrink-0 text-emerald-400" />
						<div>
							<p class="text-xs text-slate-400">Contacto: {selectedStore.contacto}</p>
							<p class="font-semibold text-slate-100 tabular-nums">{selectedStore.telefono}</p>
						</div>
					</div>
					<a
						href="tel:{selectedStore.telefono.replace(/\s+/g, '')}"
						class="flex min-h-[44px] items-center gap-1.5 rounded-xl bg-emerald-500/20 px-3.5 text-xs font-bold text-emerald-300 border border-emerald-500/40"
					>
						<Icon name="phone" class="h-4 w-4" />
						<span>Llamar</span>
					</a>
				</div>
			</div>

			<div class="rounded-xl border border-slate-800 bg-[#0B0D10] p-3">
				<p class="text-xs font-bold uppercase tracking-wider text-slate-400">Notas de reparto</p>
				<p class="mt-1 text-sm text-slate-200">{selectedStore.notasEntrega}</p>
			</div>

			{#if selectedStore.saldoPendiente > 0}
				<button
					type="button"
					onclick={() => appStore.registerStorePayment(selectedStore.id, selectedStore.saldoPendiente)}
					class="flex min-h-[52px] w-full items-center justify-center gap-2 rounded-xl bg-emerald-500 px-4 text-sm font-extrabold text-slate-950 active:scale-[0.98]"
				>
					<Icon name="check" class="h-5 w-5" />
					<span>Liquidar saldo pendiente ({formatCurrency(selectedStore.saldoPendiente)})</span>
				</button>
			{/if}
		</section>

		<!-- Historial de pedidos de la tienda -->
		<section class="space-y-2.5">
			<h2 class="text-sm font-bold uppercase tracking-wider text-slate-300">
				Historial reciente de pedidos
			</h2>

			{#if selectedStore.historialPedidos.length === 0}
				<div class="rounded-2xl border border-slate-800 bg-[#14181F] p-4 text-sm text-slate-400">
					Aún no hay pedidos registrados para esta tienda.
				</div>
			{:else}
				{#each selectedStore.historialPedidos as h}
					{@const meta = getOrderStatusMeta(h.estado)}
					<div class="rounded-2xl border border-slate-800 bg-[#14181F] p-3.5">
						<div class="flex items-center justify-between gap-2">
							<span class="text-xs font-bold text-slate-400">{h.fecha}</span>
							<span class="rounded-lg border px-2 py-0.5 text-[11px] font-bold {meta.badgeClass}">
								{meta.label}
							</span>
						</div>
						<p class="mt-1.5 text-sm font-medium text-slate-100">{h.resumen}</p>
						<p class="mt-1 text-sm font-extrabold text-amber-400 tabular-nums">
							{formatCurrency(h.total)}
						</p>
					</div>
				{/each}
			{/if}
		</section>
	</div>
{:else}
	<!-- Listado de Tiendas -->
	<div class="space-y-4">
		<div class="flex items-center justify-between gap-3">
			<div>
				<h1 class="text-2xl font-extrabold text-slate-50">Fichas de Tiendas</h1>
				<p class="text-xs text-slate-400">{appStore.stores.length} clientes registrados</p>
			</div>
			<button
				type="button"
				onclick={openCreateModal}
				class="flex min-h-[48px] items-center gap-1.5 rounded-xl bg-amber-500 px-4 text-sm font-extrabold text-slate-950 active:scale-95"
			>
				<Icon name="plus" class="h-5 w-5" />
				<span>Nueva</span>
			</button>
		</div>

		<!-- Buscador -->
		<div class="relative">
			<span class="pointer-events-none absolute left-3.5 top-1/2 -translate-y-1/2 text-slate-400">
				<Icon name="search" class="h-5 w-5" />
			</span>
			<input
				type="search"
				bind:value={searchQuery}
				placeholder="Buscar tienda, contacto o dirección..."
				class="min-h-[50px] w-full rounded-xl border border-slate-700 bg-[#14181F] pl-11 pr-4 text-sm text-slate-50 placeholder:text-slate-500 focus:border-amber-500 focus:outline-none"
			/>
		</div>

		<!-- Filtros -->
		<div class="flex gap-2">
			<button
				type="button"
				onclick={() => (filterMode = 'todas')}
				class="min-h-[44px] rounded-xl border px-4 text-xs font-bold transition {filterMode === 'todas'
					? 'border-amber-500 bg-amber-500/15 text-amber-300'
					: 'border-slate-800 bg-[#14181F] text-slate-300'}"
			>
				Todas ({appStore.stores.length})
			</button>
			<button
				type="button"
				onclick={() => (filterMode = 'con_saldo')}
				class="min-h-[44px] rounded-xl border px-4 text-xs font-bold transition {filterMode === 'con_saldo'
					? 'border-amber-500 bg-amber-500/15 text-amber-300'
					: 'border-slate-800 bg-[#14181F] text-slate-300'}"
			>
				Con saldo pendiente ({appStore.stores.filter((s) => s.saldoPendiente > 0).length})
			</button>
		</div>

		<!-- Tarjetas de tienda -->
		<div class="space-y-2.5">
			{#each filteredStores as store}
				<button
					type="button"
					onclick={() => (appStore.selectedStoreId = store.id)}
					class="flex w-full items-center justify-between gap-3 rounded-2xl border border-slate-800 bg-[#14181F] p-4 text-left transition active:scale-[0.99] hover:border-slate-600"
				>
					<div class="min-w-0 space-y-1">
						<div class="flex items-center gap-2">
							<p class="truncate text-base font-extrabold text-slate-50">{store.nombre}</p>
							<span class="shrink-0 rounded-md bg-slate-800 px-2 py-0.5 text-[11px] font-bold text-slate-300">
								{store.zona}
							</span>
						</div>
						<p class="truncate text-xs text-slate-400">
							{store.direccion} · {store.horarioEntrega}
						</p>
						<p class="text-xs text-slate-300">Contacto: {store.contacto}</p>
					</div>

					<div class="flex shrink-0 items-center gap-2">
						<div class="text-right">
							<p class="text-[11px] text-slate-400">Saldo</p>
							<p
								class="text-sm font-extrabold tabular-nums {store.saldoPendiente > 0
									? 'text-amber-400'
									: 'text-emerald-400'}"
							>
								{formatCurrency(store.saldoPendiente)}
							</p>
						</div>
						<Icon name="chevron-right" class="h-5 w-5 text-slate-500" />
					</div>
				</button>
			{:else}
				<div class="rounded-2xl border border-slate-800 bg-[#14181F] p-6 text-center text-sm text-slate-400">
					No se encontraron tiendas con ese filtro.
				</div>
			{/each}
		</div>
	</div>
{/if}

<!-- Modal Alta / Edición de Tienda -->
{#if modalOpen}
	<div class="fixed inset-0 z-50 flex items-end justify-center bg-black/75 p-0 sm:items-center sm:p-4">
		<div
			class="max-h-[90dvh] w-full max-w-lg overflow-y-auto rounded-t-3xl border border-slate-700 bg-[#14181F] p-5 sm:rounded-3xl"
		>
			<div class="mb-4 flex items-center justify-between">
				<h2 class="text-lg font-extrabold text-slate-50">
					{editingStoreId ? 'Editar ficha de tienda' : 'Nueva tienda cliente'}
				</h2>
				<button
					type="button"
					onclick={() => (modalOpen = false)}
					class="flex h-11 w-11 items-center justify-center rounded-xl bg-slate-800 text-slate-300"
					aria-label="Cerrar modal"
				>
					<Icon name="x" class="h-5 w-5" />
				</button>
			</div>

			<form onsubmit={saveStore} class="space-y-3.5">
				{#if formError}
					<p class="rounded-xl border border-red-500/40 bg-red-500/15 p-3 text-xs font-semibold text-red-200">
						{formError}
					</p>
				{/if}

				<div>
					<label for="st-nombre" class="mb-1 block text-xs font-bold text-slate-300">Nombre comercial *</label>
					<input
						id="st-nombre"
						type="text"
						bind:value={formNombre}
						placeholder="Ej. Panadería Plaza Mayor"
						class="min-h-[48px] w-full rounded-xl border border-slate-700 bg-[#0B0D10] px-3.5 text-sm text-slate-50"
					/>
				</div>

				<div class="grid grid-cols-2 gap-3">
					<div>
						<label for="st-zona" class="mb-1 block text-xs font-bold text-slate-300">Zona</label>
						<input
							id="st-zona"
							type="text"
							bind:value={formZona}
							class="min-h-[48px] w-full rounded-xl border border-slate-700 bg-[#0B0D10] px-3.5 text-sm text-slate-50"
						/>
					</div>
					<div>
						<label for="st-horario" class="mb-1 block text-xs font-bold text-slate-300">Horario entrega</label>
						<input
							id="st-horario"
							type="text"
							bind:value={formHorario}
							class="min-h-[48px] w-full rounded-xl border border-slate-700 bg-[#0B0D10] px-3.5 text-sm text-slate-50"
						/>
					</div>
				</div>

				<div>
					<label for="st-dir" class="mb-1 block text-xs font-bold text-slate-300">Dirección *</label>
					<input
						id="st-dir"
						type="text"
						bind:value={formDireccion}
						placeholder="Calle, número y local"
						class="min-h-[48px] w-full rounded-xl border border-slate-700 bg-[#0B0D10] px-3.5 text-sm text-slate-50"
					/>
				</div>

				<div class="grid grid-cols-2 gap-3">
					<div>
						<label for="st-contacto" class="mb-1 block text-xs font-bold text-slate-300">Contacto *</label>
						<input
							id="st-contacto"
							type="text"
							bind:value={formContacto}
							class="min-h-[48px] w-full rounded-xl border border-slate-700 bg-[#0B0D10] px-3.5 text-sm text-slate-50"
						/>
					</div>
					<div>
						<label for="st-tel" class="mb-1 block text-xs font-bold text-slate-300">Teléfono</label>
						<input
							id="st-tel"
							type="tel"
							bind:value={formTelefono}
							class="min-h-[48px] w-full rounded-xl border border-slate-700 bg-[#0B0D10] px-3.5 text-sm text-slate-50"
						/>
					</div>
				</div>

				<div class="grid grid-cols-2 gap-3">
					<div>
						<label for="st-saldo" class="mb-1 block text-xs font-bold text-slate-300">Saldo pendiente (€)</label>
						<input
							id="st-saldo"
							type="number"
							step="0.01"
							bind:value={formSaldo}
							class="min-h-[48px] w-full rounded-xl border border-slate-700 bg-[#0B0D10] px-3.5 text-sm text-slate-50"
						/>
					</div>
					<div>
						<label for="st-lim" class="mb-1 block text-xs font-bold text-slate-300">Límite crédito (€)</label>
						<input
							id="st-lim"
							type="number"
							step="10"
							bind:value={formLimite}
							class="min-h-[48px] w-full rounded-xl border border-slate-700 bg-[#0B0D10] px-3.5 text-sm text-slate-50"
						/>
					</div>
				</div>

				<div>
					<label for="st-notas" class="mb-1 block text-xs font-bold text-slate-300">Notas de entrega</label>
					<textarea
						id="st-notas"
						rows="2"
						bind:value={formNotas}
						class="w-full rounded-xl border border-slate-700 bg-[#0B0D10] p-3 text-sm text-slate-50"
					></textarea>
				</div>

				<button
					type="submit"
					class="min-h-[52px] w-full rounded-xl bg-amber-500 text-sm font-extrabold text-slate-950 active:scale-[0.98]"
				>
					{editingStoreId ? 'Guardar cambios' : 'Crear ficha de tienda'}
				</button>
			</form>
		</div>
	</div>
{/if}

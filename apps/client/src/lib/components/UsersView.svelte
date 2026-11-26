<script lang="ts">
	import { appStore } from '$lib/stores/app.svelte';
	import type { Role, UserRecord, UserStatus } from '$lib/types';
	import { getRoleMeta, getUserStatusMeta } from '$lib/utils';
	import Icon from './Icon.svelte';

	let roleFilter = $state<'todos' | Role>('todos');
	let modalOpen = $state(false);
	let editingUserId = $state<string | null>(null);

	let formNombre = $state('');
	let formEmail = $state('');
	let formTelefono = $state('');
	let formRol = $state<Role>('repartidor');
	let formVehiculo = $state('');
	let formEstado = $state<UserStatus>('activo');
	let formTurno = $state('06:00 - 14:00');
	let formError = $state('');

	const filteredUsers = $derived(
		appStore.users.filter((u) => (roleFilter === 'todos' ? true : u.rol === roleFilter))
	);

	const selectedUser = $derived(
		appStore.users.find((u) => u.id === appStore.selectedUserId) ?? null
	);

	function openCreateModal() {
		editingUserId = null;
		formNombre = '';
		formEmail = '';
		formTelefono = '';
		formRol = 'repartidor';
		formVehiculo = 'Furgoneta Reparto';
		formEstado = 'activo';
		formTurno = '06:00 - 14:00';
		formError = '';
		modalOpen = true;
	}

	function openEditModal(user: UserRecord) {
		editingUserId = user.id;
		formNombre = user.nombre;
		formEmail = user.email;
		formTelefono = user.telefono;
		formRol = user.rol;
		formVehiculo = user.vehiculoAsignado;
		formEstado = user.estado;
		formTurno = user.turno;
		formError = '';
		modalOpen = true;
	}

	function saveUser(event: SubmitEvent) {
		event.preventDefault();
		if (!formNombre.trim() || !formTelefono.trim()) {
			formError = 'El nombre y el teléfono son obligatorios.';
			return;
		}
		const savedId = appStore.upsertUser(
			{
				nombre: formNombre.trim(),
				email: formEmail.trim() || 'usuario@eltrigal.es',
				telefono: formTelefono.trim(),
				rol: formRol,
				vehiculoAsignado: formVehiculo.trim() || 'Sin vehículo asignado',
				estado: formEstado,
				turno: formTurno.trim() || '06:00 - 14:00'
			},
			editingUserId ?? undefined
		);
		modalOpen = false;
		appStore.selectedUserId = savedId;
	}
</script>

{#if selectedUser}
	{@const roleMeta = getRoleMeta(selectedUser.rol)}
	{@const statusMeta = getUserStatusMeta(selectedUser.estado)}

	<div class="space-y-4">
		<div class="flex items-center justify-between gap-2">
			<button
				type="button"
				onclick={() => (appStore.selectedUserId = null)}
				class="flex min-h-[48px] items-center gap-1.5 rounded-xl border border-slate-700 bg-[#14181F] px-3.5 text-sm font-bold text-slate-100 active:scale-95"
			>
				<Icon name="chevron-left" class="h-5 w-5" />
				<span>Equipo</span>
			</button>

			<button
				type="button"
				onclick={() => openEditModal(selectedUser)}
				class="flex min-h-[48px] items-center gap-2 rounded-xl bg-amber-500 px-4 text-sm font-extrabold text-slate-950 active:scale-95"
			>
				<Icon name="edit" class="h-4 w-4" />
				<span>Editar ficha</span>
			</button>
		</div>

		<section class="rounded-2xl border border-slate-800 bg-[#14181F] p-4 space-y-4">
			<div class="flex items-start justify-between gap-3">
				<div>
					<div class="flex flex-wrap items-center gap-2">
						<span class="rounded-lg border px-2.5 py-0.5 text-xs font-bold {roleMeta.badgeClass}">
							{roleMeta.label}
						</span>
						<span class="flex items-center gap-1.5 rounded-lg border px-2.5 py-0.5 text-xs font-bold {statusMeta.badgeClass}">
							<span class="h-2 w-2 rounded-full {statusMeta.dotClass}"></span>
							{statusMeta.label}
						</span>
					</div>
					<h1 class="mt-2 text-xl font-extrabold text-slate-50">{selectedUser.nombre}</h1>
					<p class="text-xs text-slate-400">{selectedUser.email}</p>
				</div>
			</div>

			<div class="grid gap-2.5 text-sm">
				<div class="flex items-center justify-between rounded-xl bg-[#0B0D10] p-3">
					<div class="flex items-start gap-2.5">
						<Icon name="phone" class="mt-0.5 h-4 w-4 text-emerald-400" />
						<div>
							<p class="text-xs text-slate-400">Teléfono</p>
							<p class="font-semibold text-slate-100 tabular-nums">{selectedUser.telefono}</p>
						</div>
					</div>
					<a
						href="tel:{selectedUser.telefono.replace(/\s+/g, '')}"
						class="flex min-h-[44px] items-center gap-1.5 rounded-xl border border-emerald-500/40 bg-emerald-500/20 px-3.5 text-xs font-bold text-emerald-300"
					>
						<Icon name="phone" class="h-4 w-4" />
						<span>Llamar</span>
					</a>
				</div>

				<div class="flex items-start gap-2.5 rounded-xl bg-[#0B0D10] p-3">
					<Icon name="truck" class="mt-0.5 h-4 w-4 text-sky-400" />
					<div>
						<p class="text-xs text-slate-400">Vehículo / Puesto asignado</p>
						<p class="font-semibold text-slate-100">{selectedUser.vehiculoAsignado}</p>
					</div>
				</div>

				<div class="flex items-start gap-2.5 rounded-xl bg-[#0B0D10] p-3">
					<Icon name="clock" class="mt-0.5 h-4 w-4 text-amber-400" />
					<div>
						<p class="text-xs text-slate-400">Turno habitual</p>
						<p class="font-semibold text-slate-100">{selectedUser.turno}</p>
					</div>
				</div>
			</div>
		</section>

		<section class="space-y-2.5">
			<h2 class="text-sm font-bold uppercase tracking-wider text-slate-300">
				Historial reciente de actividad
			</h2>
			{#each selectedUser.historial as entry}
				<div class="flex items-start gap-3 rounded-2xl border border-slate-800 bg-[#14181F] p-3.5">
					<span class="shrink-0 rounded-lg bg-slate-800 px-2 py-1 text-xs font-bold text-amber-300 tabular-nums">
						{entry.hora}
					</span>
					<p class="text-sm text-slate-200">{entry.descripcion}</p>
				</div>
			{/each}
		</section>
	</div>
{:else}
	<div class="space-y-4">
		<div class="flex items-center justify-between gap-3">
			<div>
				<h1 class="text-2xl font-extrabold text-slate-50">Fichas de Usuarios</h1>
				<p class="text-xs text-slate-400">{appStore.users.length} miembros en plantilla</p>
			</div>
			<button
				type="button"
				onclick={openCreateModal}
				class="flex min-h-[48px] items-center gap-1.5 rounded-xl bg-amber-500 px-4 text-sm font-extrabold text-slate-950 active:scale-95"
			>
				<Icon name="plus" class="h-5 w-5" />
				<span>Nuevo</span>
			</button>
		</div>

		<!-- Filtros por rol -->
		<div class="flex gap-2 overflow-x-auto pb-1">
			{#each [
				{ id: 'todos', label: 'Todos' },
				{ id: 'jefe', label: 'Jefe' },
				{ id: 'repartidor', label: 'Repartidor' },
				{ id: 'produccion', label: 'Producción' }
			] as tab}
				<button
					type="button"
					onclick={() => (roleFilter = tab.id as 'todos' | Role)}
					class="min-h-[44px] shrink-0 rounded-xl border px-4 text-xs font-bold transition {roleFilter === tab.id
						? 'border-amber-500 bg-amber-500/15 text-amber-300'
						: 'border-slate-800 bg-[#14181F] text-slate-300'}"
				>
					{tab.label}
				</button>
			{/each}
		</div>

		<div class="space-y-2.5">
			{#each filteredUsers as user}
				{@const roleMeta = getRoleMeta(user.rol)}
				{@const statusMeta = getUserStatusMeta(user.estado)}
				<button
					type="button"
					onclick={() => (appStore.selectedUserId = user.id)}
					class="flex w-full items-center justify-between gap-3 rounded-2xl border border-slate-800 bg-[#14181F] p-4 text-left transition active:scale-[0.99] hover:border-slate-600"
				>
					<div class="min-w-0 space-y-1.5">
						<div class="flex flex-wrap items-center gap-2">
							<p class="text-base font-extrabold text-slate-50">{user.nombre}</p>
							<span class="rounded-md border px-2 py-0.5 text-[11px] font-bold {roleMeta.badgeClass}">
								{roleMeta.shortLabel}
							</span>
						</div>
						<p class="truncate text-xs text-slate-300">{user.vehiculoAsignado}</p>
						<p class="truncate text-xs text-slate-400">{user.ultimaActividad}</p>
					</div>

					<div class="flex shrink-0 items-center gap-2">
						<span class="flex items-center gap-1.5 rounded-lg border px-2.5 py-1 text-[11px] font-bold {statusMeta.badgeClass}">
							<span class="h-2 w-2 rounded-full {statusMeta.dotClass}"></span>
							{statusMeta.label}
						</span>
						<Icon name="chevron-right" class="h-5 w-5 text-slate-500" />
					</div>
				</button>
			{/each}
		</div>
	</div>
{/if}

{#if modalOpen}
	<div class="fixed inset-0 z-50 flex items-end justify-center bg-black/75 p-0 sm:items-center sm:p-4">
		<div
			class="max-h-[90dvh] w-full max-w-lg overflow-y-auto rounded-t-3xl border border-slate-700 bg-[#14181F] p-5 sm:rounded-3xl"
		>
			<div class="mb-4 flex items-center justify-between">
				<h2 class="text-lg font-extrabold text-slate-50">
					{editingUserId ? 'Editar ficha de usuario' : 'Nuevo empleado'}
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

			<form onsubmit={saveUser} class="space-y-3.5">
				{#if formError}
					<p class="rounded-xl border border-red-500/40 bg-red-500/15 p-3 text-xs font-semibold text-red-200">
						{formError}
					</p>
				{/if}

				<div>
					<label for="usr-nombre" class="mb-1 block text-xs font-bold text-slate-300">Nombre completo *</label>
					<input
						id="usr-nombre"
						type="text"
						bind:value={formNombre}
						placeholder="Ej. Laura Martín"
						class="min-h-[48px] w-full rounded-xl border border-slate-700 bg-[#0B0D10] px-3.5 text-sm text-slate-50"
					/>
				</div>

				<div class="grid grid-cols-2 gap-3">
					<div>
						<label for="usr-tel" class="mb-1 block text-xs font-bold text-slate-300">Teléfono *</label>
						<input
							id="usr-tel"
							type="tel"
							bind:value={formTelefono}
							placeholder="600 123 456"
							class="min-h-[48px] w-full rounded-xl border border-slate-700 bg-[#0B0D10] px-3.5 text-sm text-slate-50"
						/>
					</div>
					<div>
						<label for="usr-email" class="mb-1 block text-xs font-bold text-slate-300">Correo</label>
						<input
							id="usr-email"
							type="email"
							bind:value={formEmail}
							placeholder="laura@eltrigal.es"
							class="min-h-[48px] w-full rounded-xl border border-slate-700 bg-[#0B0D10] px-3.5 text-sm text-slate-50"
						/>
					</div>
				</div>

				<div class="grid grid-cols-2 gap-3">
					<div>
						<label for="usr-rol" class="mb-1 block text-xs font-bold text-slate-300">Rol operativo</label>
						<select
							id="usr-rol"
							bind:value={formRol}
							class="min-h-[48px] w-full rounded-xl border border-slate-700 bg-[#0B0D10] px-3 text-sm text-slate-50"
						>
							<option value="jefe">Jefe</option>
							<option value="repartidor">Repartidor</option>
							<option value="produccion">Producción</option>
						</select>
					</div>
					<div>
						<label for="usr-estado" class="mb-1 block text-xs font-bold text-slate-300">Estado actual</label>
						<select
							id="usr-estado"
							bind:value={formEstado}
							class="min-h-[48px] w-full rounded-xl border border-slate-700 bg-[#0B0D10] px-3 text-sm text-slate-50"
						>
							<option value="activo">Activo</option>
							<option value="en_ruta">En ruta</option>
							<option value="obrador">En obrador</option>
							<option value="descanso">Descanso</option>
						</select>
					</div>
				</div>

				<div>
					<label for="usr-vehiculo" class="mb-1 block text-xs font-bold text-slate-300">Vehículo o puesto</label>
					<input
						id="usr-vehiculo"
						type="text"
						bind:value={formVehiculo}
						placeholder="Ej. Furgón Isotermo · 3912-LMN"
						class="min-h-[48px] w-full rounded-xl border border-slate-700 bg-[#0B0D10] px-3.5 text-sm text-slate-50"
					/>
				</div>

				<div>
					<label for="usr-turno" class="mb-1 block text-xs font-bold text-slate-300">Turno</label>
					<input
						id="usr-turno"
						type="text"
						bind:value={formTurno}
						class="min-h-[48px] w-full rounded-xl border border-slate-700 bg-[#0B0D10] px-3.5 text-sm text-slate-50"
					/>
				</div>

				<button
					type="submit"
					class="min-h-[52px] w-full rounded-xl bg-amber-500 text-sm font-extrabold text-slate-950 active:scale-[0.98]"
				>
					{editingUserId ? 'Guardar cambios' : 'Dar de alta'}
				</button>
			</form>
		</div>
	</div>
{/if}

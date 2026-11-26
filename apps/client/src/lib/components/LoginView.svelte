<script lang="ts">
	import { appStore } from '$lib/stores/app.svelte';
	import type { Role } from '$lib/types';
	import Icon from './Icon.svelte';

	let identifier = $state('carlos@eltrigal.es');
	let password = $state('••••••••');
	let selectedRole = $state<Role>('jefe');
	let showPassword = $state(false);
	let errorMsg = $state('');

	const demoProfiles: {
		role: Role;
		title: string;
		subtitle: string;
		email: string;
		icon: string;
	}[] = [
		{
			role: 'jefe',
			title: 'Jefe / Encargado',
			subtitle: 'Control total, stock, fichas y finanzas',
			email: 'carlos@eltrigal.es',
			icon: 'home'
		},
		{
			role: 'repartidor',
			title: 'Repartidor',
			subtitle: 'Ruta diaria, entregas, cobros y gastos',
			email: 'lucia@eltrigal.es',
			icon: 'truck'
		},
		{
			role: 'produccion',
			title: 'Producción',
			subtitle: 'Hornadas, stock obrador y pedidos',
			email: 'elena@eltrigal.es',
			icon: 'flame'
		}
	];

	function selectDemo(profile: (typeof demoProfiles)[number]) {
		selectedRole = profile.role;
		identifier = profile.email;
		password = '••••••••';
		errorMsg = '';
	}

	function handleSubmit(event: SubmitEvent) {
		event.preventDefault();
		if (!identifier.trim() || !password.trim()) {
			errorMsg = 'Introduce tu correo o teléfono y contraseña.';
			return;
		}
		appStore.loginWithRole(selectedRole, identifier);
	}
</script>

<main class="flex min-h-dvh flex-col justify-between bg-[#0B0D10] px-4 py-6">
	<div class="mx-auto w-full max-w-md space-y-6">
		<!-- Cabecera de marca -->
		<div class="flex items-center gap-3.5 pt-2">
			<div
				class="flex h-14 w-14 items-center justify-center rounded-2xl bg-amber-500 text-slate-950 shadow-xl shadow-amber-500/20"
			>
				<Icon name="flame" class="h-8 w-8" />
			</div>
			<div>
				<span
					class="inline-block rounded-md border border-amber-500/30 bg-amber-500/10 px-2 py-0.5 text-[11px] font-bold uppercase tracking-wider text-amber-300"
				>
					Fase 1 · Demo Operativa
				</span>
				<h1 class="mt-1 text-2xl font-extrabold tracking-tight text-slate-50">
					Panadería Control
				</h1>
			</div>
		</div>

		<!-- Selector rápido de rol demo (1 toque) -->
		<section class="rounded-2xl border border-slate-800 bg-[#14181F] p-4">
			<div class="mb-3 flex items-center justify-between">
				<h2 class="text-sm font-bold uppercase tracking-wider text-slate-300">
					Acceso rápido por rol
				</h2>
				<span class="text-xs text-amber-400">1 toque para entrar</span>
			</div>

			<div class="space-y-2.5">
				{#each demoProfiles as p}
					{@const isSelected = selectedRole === p.role}
					<div
						class="flex items-center gap-2 rounded-xl border p-2 transition {isSelected
							? 'border-amber-500 bg-amber-500/10'
							: 'border-slate-800 bg-[#0F1319]'}"
					>
						<button
							type="button"
							onclick={() => selectDemo(p)}
							class="flex min-h-[48px] flex-1 items-center gap-3 px-2 text-left"
						>
							<div
								class="flex h-10 w-10 shrink-0 items-center justify-center rounded-xl {isSelected
									? 'bg-amber-500 text-slate-950'
									: 'bg-slate-800 text-slate-200'}"
							>
								<Icon name={p.icon} class="h-5 w-5" />
							</div>
							<div class="min-w-0">
								<p class="text-sm font-bold text-slate-50">{p.title}</p>
								<p class="truncate text-xs text-slate-400">{p.subtitle}</p>
							</div>
						</button>
						<button
							type="button"
							onclick={() => appStore.loginWithRole(p.role)}
							class="min-h-[48px] shrink-0 rounded-xl bg-amber-500 px-3.5 text-xs font-extrabold text-slate-950 transition active:scale-95 hover:bg-amber-400"
						>
							Entrar
						</button>
					</div>
				{/each}
			</div>
		</section>

		<!-- Formulario estándar -->
		<form
			onsubmit={handleSubmit}
			class="space-y-4 rounded-2xl border border-slate-800 bg-[#14181F] p-4"
		>
			<h2 class="text-sm font-bold uppercase tracking-wider text-slate-300">
				Iniciar sesión con credenciales
			</h2>

			{#if errorMsg}
				<div
					class="rounded-xl border border-red-500/40 bg-red-500/15 px-3.5 py-2.5 text-sm font-medium text-red-200"
				>
					{errorMsg}
				</div>
			{/if}

			<div>
				<label for="login-id" class="mb-1.5 block text-sm font-semibold text-slate-200">
					Correo electrónico o teléfono
				</label>
				<input
					id="login-id"
					type="text"
					bind:value={identifier}
					placeholder="ej. lucia@eltrigal.es o 645118204"
					class="min-h-[52px] w-full rounded-xl border border-slate-700 bg-[#0B0D10] px-4 text-base text-slate-50 placeholder:text-slate-500 focus:border-amber-500 focus:outline-none"
				/>
			</div>

			<div>
				<label for="login-pass" class="mb-1.5 block text-sm font-semibold text-slate-200">
					Contraseña
				</label>
				<div class="relative flex items-center">
					<input
						id="login-pass"
						type={showPassword ? 'text' : 'password'}
						bind:value={password}
						placeholder="Tu contraseña"
						class="min-h-[52px] w-full rounded-xl border border-slate-700 bg-[#0B0D10] pl-4 pr-24 text-base text-slate-50 placeholder:text-slate-500 focus:border-amber-500 focus:outline-none"
					/>
					<button
						type="button"
						onclick={() => (showPassword = !showPassword)}
						class="absolute right-2 min-h-[40px] rounded-lg px-3 text-xs font-bold text-amber-400 hover:bg-slate-800"
					>
						{showPassword ? 'Ocultar' : 'Mostrar'}
					</button>
				</div>
			</div>

			<button
				type="submit"
				class="flex min-h-[54px] w-full items-center justify-center gap-2 rounded-xl bg-amber-500 px-5 text-base font-extrabold text-slate-950 shadow-lg shadow-amber-500/20 transition active:scale-[0.98] hover:bg-amber-400"
			>
				<span>Acceder como {demoProfiles.find((d) => d.role === selectedRole)?.title}</span>
				<Icon name="chevron-right" class="h-5 w-5" />
			</button>
		</form>
	</div>

	<footer class="mx-auto mt-6 w-full max-w-md text-center text-xs text-slate-500">
		Obrador Central · Panadería Artesanal El Trigal · Tauri 2
	</footer>
</main>

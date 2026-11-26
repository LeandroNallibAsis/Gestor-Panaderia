<script lang="ts">
	import { appStore } from '$lib/stores/app.svelte';
	import type { ExpenseRecord } from '$lib/types';
	import { formatCurrency } from '$lib/utils';
	import Icon from './Icon.svelte';

	let concepto = $state('');
	let categoria = $state<ExpenseRecord['categoria']>('combustible');
	let importe = $state('');
	let vehiculo = $state('4821-KLP');

	const totalExpenses = $derived(appStore.expenses.reduce((acc, e) => acc + e.importe, 0));

	function submitExpense(event: SubmitEvent) {
		event.preventDefault();
		const amount = Number(importe);
		if (!concepto.trim() || !amount || amount <= 0) return;
		appStore.addExpense({
			concepto: concepto.trim(),
			categoria,
			importe: amount,
			vehiculo
		});
		concepto = '';
		importe = '';
	}
</script>

<div class="space-y-4">
	<div class="flex items-center justify-between gap-2">
		<div>
			<h1 class="text-2xl font-extrabold text-slate-50">Gastos en Ruta</h1>
			<p class="text-xs text-slate-400">Registro rápido para ticket de reparto</p>
		</div>
		<div class="rounded-xl border border-slate-800 bg-[#14181F] px-3.5 py-2 text-right">
			<p class="text-[11px] text-slate-400">Total hoy</p>
			<p class="text-base font-extrabold text-amber-400 tabular-nums">{formatCurrency(totalExpenses)}</p>
		</div>
	</div>

	<form onsubmit={submitExpense} class="space-y-3 rounded-2xl border border-slate-800 bg-[#14181F] p-4">
		<h2 class="text-sm font-bold uppercase tracking-wider text-slate-300">Añadir gasto (1 paso)</h2>
		<input
			type="text"
			bind:value={concepto}
			placeholder="Concepto (ej. Diésel, Peaje M-40, Parking)"
			class="min-h-[48px] w-full rounded-xl border border-slate-700 bg-[#0B0D10] px-3.5 text-sm text-slate-50"
		/>
		<div class="grid grid-cols-2 gap-2.5">
			<select
				bind:value={categoria}
				class="min-h-[48px] rounded-xl border border-slate-700 bg-[#0B0D10] px-3 text-sm text-slate-50"
			>
				<option value="combustible">Combustible</option>
				<option value="parking">Parking</option>
				<option value="peaje">Peaje</option>
				<option value="mantenimiento">Mantenimiento</option>
			</select>
			<input
				type="number"
				step="0.01"
				bind:value={importe}
				placeholder="Importe (€)"
				class="min-h-[48px] rounded-xl border border-slate-700 bg-[#0B0D10] px-3.5 text-sm text-slate-50"
			/>
		</div>
		<button
			type="submit"
			class="flex min-h-[50px] w-full items-center justify-center gap-2 rounded-xl bg-amber-500 text-sm font-extrabold text-slate-950 active:scale-[0.98]"
		>
			<Icon name="plus" class="h-5 w-5" />
			<span>Registrar gasto</span>
		</button>
	</form>

	<div class="space-y-2.5">
		{#each appStore.expenses as exp}
			<div class="flex items-center justify-between gap-3 rounded-2xl border border-slate-800 bg-[#14181F] p-3.5">
				<div>
					<p class="text-sm font-bold text-slate-50">{exp.concepto}</p>
					<p class="text-xs text-slate-400">
						{exp.categoria} · {exp.vehiculo} · {exp.hora}
					</p>
				</div>
				<span class="text-base font-extrabold text-amber-400 tabular-nums">
					{formatCurrency(exp.importe)}
				</span>
			</div>
		{/each}
	</div>
</div>

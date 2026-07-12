<!-- Reusable toggle switch component -->
<script lang="ts">
	type Props = {
		checked: boolean;
		onchange?: (v: boolean) => void;
		id?: string;
		label?: string;
		description?: string;
		disabled?: boolean;
	};
	let { checked, onchange, id, label, description, disabled = false }: Props = $props();
</script>

{#if label}
	<div class="flex items-center justify-between border border-border rounded-lg px-4 py-3 bg-secondary/40">
		<div class="min-w-0 pr-4">
			<p class="text-sm font-medium">{label}</p>
			{#if description}<p class="text-xs text-muted-foreground mt-0.5">{description}</p>{/if}
		</div>
		<button
			type="button"
			{id}
			role="switch"
			aria-checked={checked}
			aria-label={label}
			{disabled}
			onclick={() => !disabled && onchange?.(!checked)}
			class="relative inline-flex h-5 w-9 shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out focus:outline-none focus:ring-2 focus:ring-ring focus:ring-offset-1 disabled:opacity-50 {checked ? 'bg-primary' : 'bg-muted'}"
		>
			<span class="pointer-events-none inline-block h-4 w-4 rounded-full bg-white shadow-sm ring-0 transition duration-200 ease-in-out {checked ? 'translate-x-4' : 'translate-x-0'}"></span>
		</button>
	</div>
{:else}
	<button
		type="button"
		{id}
		role="switch"
		aria-checked={checked}
		aria-label="Toggle"
		{disabled}
		onclick={() => !disabled && onchange?.(!checked)}
		class="relative inline-flex h-5 w-9 shrink-0 cursor-pointer rounded-full border-2 border-transparent transition-colors duration-200 ease-in-out focus:outline-none focus:ring-2 focus:ring-ring disabled:opacity-50 {checked ? 'bg-primary' : 'bg-muted'}"
	>
		<span class="pointer-events-none inline-block h-4 w-4 rounded-full bg-white shadow-sm ring-0 transition duration-200 ease-in-out {checked ? 'translate-x-4' : 'translate-x-0'}"></span>
	</button>
{/if}

<script lang="ts">
	import { Layers, ChevronDown, Check, Plus } from '@lucide/svelte';
	import type { EnvironmentResponseDto } from '$lib/client/types.gen';

	type Props = {
		envs: EnvironmentResponseDto[];
		selectedId: number | null;
		onSelect: (id: number) => void;
		onCreateNew: () => void;
	};

	let { envs, selectedId, onSelect, onCreateNew }: Props = $props();

	let open = $state(false);
	const selected = $derived(envs.find((e) => e.id === selectedId));
</script>

<div class="relative">
	<button
		class="flex items-center gap-1.5 font-medium text-foreground hover:text-primary transition-colors text-sm"
		onclick={() => (open = !open)}
	>
		<Layers class="w-3.5 h-3.5" />
		{selected?.name ?? '...'}
		<ChevronDown class="w-3.5 h-3.5 text-muted-foreground" />
	</button>

	{#if open}
		<div
			class="fixed inset-0 z-10"
			role="button"
			tabindex="-1"
			aria-label="Close"
			onclick={() => (open = false)}
			onkeydown={() => {}}
		></div>
		<div
			class="absolute top-full left-0 mt-1 z-20 bg-popover border border-border rounded-lg shadow-xl min-w-[180px] overflow-hidden"
		>
			<div
				class="px-3 py-2 text-[10px] uppercase tracking-widest text-muted-foreground border-b border-border"
			>
				Environments
			</div>

			{#each envs as env}
				<button
					class="w-full flex items-center gap-2 px-3 py-2 text-sm hover:bg-accent transition-colors text-left"
					onclick={() => { onSelect(env.id); open = false; }}
				>
					{#if selectedId === env.id}
						<Check class="w-3.5 h-3.5 text-primary shrink-0" />
					{:else}
						<span class="w-3.5"></span>
					{/if}
					<span class="truncate flex-1">{env.name}</span>
					{#if env.is_default}
						<span class="text-[10px] px-1.5 py-0.5 rounded bg-muted text-muted-foreground">default</span>
					{/if}
				</button>
			{/each}

			<div class="border-t border-border">
				<button
					class="w-full flex items-center gap-2 px-3 py-2 text-sm text-muted-foreground hover:bg-accent hover:text-foreground transition-colors"
					onclick={() => { open = false; onCreateNew(); }}
				>
					<Plus class="w-3.5 h-3.5" />
					New environment
				</button>
			</div>
		</div>
	{/if}
</div>

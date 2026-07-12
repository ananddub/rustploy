<script lang="ts">
	import { Plus, Globe, Database, Layers, FileText, Bot, Download } from '@lucide/svelte';

	type ServiceType = 'application' | 'database' | 'compose' | 'template' | 'ai' | 'import';
	type Props = { onSelect: (type: ServiceType) => void };
	let { onSelect }: Props = $props();

	let open = $state(false);

	const options: { type: ServiceType; label: string; icon: any }[] = [
		{ type: 'application', label: 'Application', icon: Globe },
		{ type: 'database', label: 'Database', icon: Database },
		{ type: 'compose', label: 'Compose', icon: Layers },
		{ type: 'template', label: 'Template', icon: FileText },
		{ type: 'ai', label: 'AI Assistant', icon: Bot },
		{ type: 'import', label: 'Import', icon: Download }
	];
</script>

<div class="relative">
	<button
		class="inline-flex items-center gap-1.5 px-3 py-1.5 rounded-md bg-primary text-primary-foreground text-sm font-medium hover:bg-primary/90 transition-colors"
		onclick={() => (open = !open)}
	>
		<Plus class="w-4 h-4" />
		Create Service
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
			class="absolute top-full right-0 mt-1 z-20 bg-popover border border-border rounded-lg shadow-xl w-48 overflow-hidden"
		>
			<p class="px-3 py-2 text-[10px] uppercase tracking-widest text-muted-foreground border-b border-border">
				Actions
			</p>
			{#each options as opt}
				<button
					class="w-full flex items-center gap-2.5 px-3 py-2 text-sm text-muted-foreground hover:bg-accent hover:text-foreground transition-colors text-left"
					onclick={() => { open = false; onSelect(opt.type); }}
				>
					<opt.icon class="w-4 h-4 shrink-0" />
					{opt.label}
				</button>
			{/each}
		</div>
	{/if}
</div>

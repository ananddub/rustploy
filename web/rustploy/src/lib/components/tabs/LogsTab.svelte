<script lang="ts">
	import { FileText, RefreshCw } from '@lucide/svelte';

	type Props = { serviceLabel?: string };
	let { serviceLabel }: Props = $props();
	const label = $derived(serviceLabel ?? 'service');

	let lines = $state('100');
</script>

<div class="bg-card border border-border rounded-lg p-6 flex flex-col gap-4">
	<div class="flex items-center justify-between">
		<div>
			<h2 class="text-base font-semibold">Logs</h2>
			<p class="text-sm text-muted-foreground mt-1">Live logs for this {label}.</p>
		</div>
		<div class="flex items-center gap-2">
			<select
				class="h-8 rounded-md border border-input bg-transparent px-2 text-sm focus:outline-none focus:ring-1 focus:ring-ring"
				bind:value={lines}
			>
				<option value="50">50 lines</option>
				<option value="100">100 lines</option>
				<option value="200">200 lines</option>
				<option value="500">500 lines</option>
			</select>
			<button class="inline-flex items-center gap-1.5 px-2.5 py-1.5 rounded-md text-sm text-muted-foreground hover:text-foreground hover:bg-accent transition-colors">
				<RefreshCw size={14} /> Refresh
			</button>
		</div>
	</div>

	<div class="rounded-md bg-[#0d0d0d] border border-border p-4 font-mono text-xs text-muted-foreground/60 min-h-64 flex items-center justify-center">
		<div class="flex flex-col items-center gap-2 text-muted-foreground/30">
			<FileText size={32} />
			<p>No logs available. Deploy your {label} first.</p>
		</div>
	</div>
</div>

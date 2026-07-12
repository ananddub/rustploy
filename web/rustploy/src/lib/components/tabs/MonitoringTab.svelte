<script lang="ts">
	import { ChartLine } from '@lucide/svelte';
	type Props = { serviceLabel?: string };
	let { serviceLabel }: Props = $props();
</script>

<div class="bg-card border border-border rounded-lg p-6">
	<h2 class="text-base font-semibold mb-1">Monitoring</h2>
	<p class="text-sm text-muted-foreground">Resource usage for this {serviceLabel ?? 'service'}.</p>
	<div class="flex items-center justify-center py-16 text-muted-foreground/30">
		<div class="flex flex-col items-center gap-2">
			<ChartLine size={40} />
			<p class="text-sm">No monitoring data yet.</p>
		</div>
	</div>
</div>

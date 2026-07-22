<script lang="ts">
	import { goto } from '$app/navigation';
	import { Box } from '@lucide/svelte';
	import type { ApplicationResponseDto } from '$lib/client/types.gen';
	import { appStatusDot } from '$lib/helpers';

	type Props = { app: ApplicationResponseDto; projectId: number };
	let { app, projectId }: Props = $props();

	function formatDate(ts: number) {
		return new Date(ts * 1000).toLocaleDateString('en-IN', {
			day: '2-digit', month: 'short', year: 'numeric'
		});
	}
</script>

<div
	role="button"
	tabindex="0"
	class="w-56 bg-card border border-border rounded-lg p-4 flex flex-col gap-3 hover:border-foreground/20 transition-colors cursor-pointer group shrink-0"
	onclick={() => goto(`/projects/${projectId}/app/${app.id}`)}
	onkeydown={(e) => e.key === 'Enter' && goto(`/projects/${projectId}/app/${app.id}`)}
>
	<div class="flex items-center gap-3">
		<div class="w-9 h-9 rounded-lg bg-primary/10 flex items-center justify-center shrink-0 relative">
			<Box class="w-4 h-4 text-primary" />
			<span class="absolute -top-0.5 -right-0.5 w-2.5 h-2.5 rounded-full border-2 border-card {appStatusDot(app.app_status)}"></span>
		</div>
		<div class="min-w-0">
			<p class="font-medium text-sm truncate group-hover:text-primary transition-colors">{app.app_name}</p>
			<p class="text-sm text-muted-foreground">{app.build_type}</p>
		</div>
	</div>

	<div class="flex items-center justify-between border-t border-border pt-2">
		<span class="flex items-center gap-1.5 text-xs text-muted-foreground">
			<span class="w-1.5 h-1.5 rounded-full {appStatusDot(app.app_status)}"></span>
			{app.app_status ?? 'unknown'}
		</span>
		<span class="text-xs text-muted-foreground/50">{formatDate(app.created_at)}</span>
	</div>
</div>

<script lang="ts">
	import { FileText, RefreshCw, Download } from '@lucide/svelte';
	import Switch from '$lib/components/Switch.svelte';
	import { deploymentControllerDockerServiceLogs, deploymentControllerDockerComposeLogs } from '$lib/client/sdk.gen';

	type Props = {
		serviceLabel?: string;
		serviceId?: number;
		appName?: string;
		serviceType?: 'application' | 'compose';
	};
	let { serviceLabel, serviceId, appName, serviceType = 'application' }: Props = $props();
	const label = $derived(serviceLabel ?? 'service');

	let lines = $state('100');
	let timestamps = $state(false);
	let logs = $state<string[]>([]);
	let loading = $state(false);
	let error = $state('');

	async function loadLogs() {
		if (!appName) return;
		loading = true; error = ''; logs = [];
		try {
			let res: any;
			if (serviceType === 'compose') {
				res = await deploymentControllerDockerComposeLogs({
					query: {
						project_name: appName,
						tail: parseInt(lines),
						timestamps
					} as any
				});
			} else {
				res = await deploymentControllerDockerServiceLogs({
					path: { target: appName },
					query: { tail: parseInt(lines), timestamps } as any
				});
			}
			// Response may be a string blob of log lines
			const raw = res.data as any;
			if (typeof raw === 'string') {
				logs = raw.split('\n').filter(Boolean);
			} else if (Array.isArray(raw)) {
				logs = raw;
			}
		} catch (e: any) {
			error = e?.message ?? 'Failed to load logs';
		} finally {
			loading = false;
		}
	}

	function downloadLogs() {
		const blob = new Blob([logs.join('\n')], { type: 'text/plain' });
		const url = URL.createObjectURL(blob);
		const a = document.createElement('a');
		a.href = url; a.download = `${appName ?? 'logs'}.txt`; a.click();
		URL.revokeObjectURL(url);
	}

	function ansiToHtml(line: string): string {
		// Strip ANSI escape codes for display
		return line.replace(/\x1B\[[0-9;]*[mGKHF]/g, '');
	}
</script>

<div class="bg-card border border-border rounded-lg p-6 flex flex-col gap-4 animate-fade-up">
	<div class="flex items-center justify-between">
		<div>
			<h2 class="text-base font-semibold">Logs</h2>
			<p class="text-sm text-muted-foreground mt-1">Container logs for this {label}.</p>
		</div>
		<div class="flex items-center gap-2">
			<div class="flex items-center gap-1.5 text-xs text-muted-foreground">
				<Switch checked={timestamps} onchange={(v) => (timestamps = v)} />
				Timestamps
			</div>
			<select
				class="h-8 rounded-md border border-input bg-secondary px-2 text-sm focus:outline-none focus:ring-1 focus:ring-ring"
				bind:value={lines}
			>
				<option value="50">50 lines</option>
				<option value="100">100 lines</option>
				<option value="200">200 lines</option>
				<option value="500">500 lines</option>
				<option value="1000">1000 lines</option>
			</select>
			{#if logs.length > 0}
				<button onclick={downloadLogs} class="inline-flex items-center gap-1.5 px-2.5 py-1.5 rounded-md text-sm text-muted-foreground hover:text-foreground hover:bg-accent transition-colors" title="Download logs">
					<Download size={14} />
				</button>
			{/if}
			<button
				onclick={loadLogs}
				disabled={loading || !appName}
				class="inline-flex items-center gap-1.5 px-2.5 py-1.5 rounded-md text-sm text-muted-foreground hover:text-foreground hover:bg-accent disabled:opacity-50 transition-colors"
			>
				<RefreshCw size={14} class={loading ? 'animate-spin' : ''} /> Fetch Logs
			</button>
		</div>
	</div>

	{#if !appName}
		<div class="rounded-md bg-[#0d0d0d] border border-border p-4 font-mono text-xs min-h-64 flex items-center justify-center">
			<div class="flex flex-col items-center gap-2 text-muted-foreground/30">
				<FileText size={32} />
				<p>Deploy your {label} first to see logs.</p>
			</div>
		</div>
	{:else if error}
		<div class="rounded-md bg-destructive/10 border border-destructive/30 px-3 py-2 text-sm text-destructive">{error}</div>
	{:else if logs.length === 0 && !loading}
		<div class="rounded-md bg-[#0d0d0d] border border-border p-4 font-mono text-xs min-h-64 flex items-center justify-center">
			<div class="flex flex-col items-center gap-2 text-muted-foreground/30">
				<FileText size={32} />
				<p>Click "Fetch Logs" to load container output.</p>
			</div>
		</div>
	{:else}
		<div class="rounded-md bg-[#0d0d0d] border border-border p-4 font-mono text-xs min-h-64 max-h-[600px] overflow-y-auto">
			{#if loading}
				<div class="flex items-center justify-center py-10 text-muted-foreground/30">
					<div class="w-5 h-5 border-2 border-muted-foreground/20 border-t-muted-foreground/60 rounded-full animate-spin mr-2"></div>
					Loading logs…
				</div>
			{:else}
				{#each logs as line, i}
					<div class="py-px text-green-400/80 leading-relaxed whitespace-pre-wrap break-all">
						{ansiToHtml(line)}
					</div>
				{/each}
			{/if}
		</div>
	{/if}
</div>

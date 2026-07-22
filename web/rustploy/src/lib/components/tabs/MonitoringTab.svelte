<script lang="ts">
	import { ChartLine, RefreshCw, Cpu, MemoryStick } from '@lucide/svelte';
	import {
		deploymentControllerApplicationStats,
		deploymentControllerComposeStats
	} from '$lib/client/sdk.gen';
	import type { DockerStatsQuery } from '$lib/client/types.gen';

	type Props = {
		serviceLabel?: string;
		serviceId?: number;
		appName?: string;
		serviceType?: 'application' | 'compose';
	};
	let { serviceLabel, serviceId, serviceType = 'application' }: Props = $props();

	type Stats = {
		cpu_percent?: number;
		memory_usage?: number;
		memory_limit?: number;
		memory_percent?: number;
		network_rx?: number;
		network_tx?: number;
		block_read?: number;
		block_write?: number;
	};

	let stats = $state<Stats | null>(null);
	let loading = $state(false);
	let error = $state('');

	async function loadStats() {
		if (!serviceId) return;
		loading = true; error = '';
		try {
			const statsQuery = { query: {} as DockerStatsQuery };
			const res = serviceType === 'compose'
				? await deploymentControllerComposeStats({ path: { id: serviceId }, query: statsQuery })
				: await deploymentControllerApplicationStats({ path: { id: serviceId }, query: statsQuery });
			stats = (res.data as Stats) ?? null;
		} catch (e: any) {
			error = e?.message ?? 'Failed to load stats';
		} finally {
			loading = false;
		}
	}

	function formatBytes(bytes?: number): string {
		if (!bytes) return '0 B';
		const units = ['B', 'KB', 'MB', 'GB'];
		let i = 0;
		let v = bytes;
		while (v >= 1024 && i < units.length - 1) { v /= 1024; i++; }
		return `${v.toFixed(1)} ${units[i]}`;
	}

	function fmtPercent(v?: number): string {
		return v != null ? `${v.toFixed(2)}%` : '—';
	}
</script>

<div class="flex flex-col gap-6 animate-fade-up">
	<section class="bg-card border border-border rounded-lg p-6">
		<div class="flex items-center justify-between mb-4">
			<div>
				<h2 class="text-base font-semibold">Monitoring</h2>
				<p class="text-sm text-muted-foreground mt-1">Real-time resource usage for this {serviceLabel ?? 'service'}.</p>
			</div>
			<button
				onclick={loadStats}
				disabled={loading || !serviceId}
				class="inline-flex items-center gap-1.5 px-2.5 py-1.5 rounded-md text-sm text-muted-foreground hover:text-foreground hover:bg-accent disabled:opacity-50 transition-colors"
			>
				<RefreshCw size={14} class={loading ? 'animate-spin' : ''} /> Refresh
			</button>
		</div>

		{#if error}
			<div class="rounded-md bg-destructive/10 border border-destructive/30 px-3 py-2 text-sm text-destructive">{error}</div>
		{:else if !stats && !loading}
			<div class="flex flex-col items-center justify-center py-12 text-muted-foreground/30">
				<ChartLine size={40} class="mb-3" />
				<p class="text-sm">Click Refresh to load current stats.</p>
			</div>
		{:else if loading}
			<div class="flex justify-center py-12"><div class="w-6 h-6 border-2 border-muted-foreground/30 border-t-foreground rounded-full animate-spin"></div></div>
		{:else if stats}
			<div class="grid grid-cols-2 md:grid-cols-4 gap-4">
				<div class="bg-secondary rounded-lg p-4">
					<div class="flex items-center gap-2 text-muted-foreground text-xs uppercase tracking-wide mb-2">
						<Cpu size={12} /> CPU
					</div>
					<p class="text-2xl font-semibold">{fmtPercent(stats.cpu_percent)}</p>
				</div>
				<div class="bg-secondary rounded-lg p-4">
					<div class="flex items-center gap-2 text-muted-foreground text-xs uppercase tracking-wide mb-2">
						Memory
					</div>
					<p class="text-2xl font-semibold">{fmtPercent(stats.memory_percent)}</p>
					<p class="text-sm text-muted-foreground mt-1">{formatBytes(stats.memory_usage)} / {formatBytes(stats.memory_limit)}</p>
				</div>
				<div class="bg-secondary rounded-lg p-4">
					<div class="flex items-center gap-2 text-muted-foreground text-xs uppercase tracking-wide mb-2">
						Network In
					</div>
					<p class="text-2xl font-semibold">{formatBytes(stats.network_rx)}</p>
				</div>
				<div class="bg-secondary rounded-lg p-4">
					<div class="flex items-center gap-2 text-muted-foreground text-xs uppercase tracking-wide mb-2">
						Network Out
					</div>
					<p class="text-2xl font-semibold">{formatBytes(stats.network_tx)}</p>
				</div>
				<div class="bg-secondary rounded-lg p-4">
					<div class="text-muted-foreground text-xs uppercase tracking-wide mb-2">Block Read</div>
					<p class="text-2xl font-semibold">{formatBytes(stats.block_read)}</p>
				</div>
				<div class="bg-secondary rounded-lg p-4">
					<div class="text-muted-foreground text-xs uppercase tracking-wide mb-2">Block Write</div>
					<p class="text-2xl font-semibold">{formatBytes(stats.block_write)}</p>
				</div>
			</div>
		{/if}
	</section>
</div>

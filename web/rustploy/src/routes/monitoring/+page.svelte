<script lang="ts">
	import { onMount, onDestroy } from 'svelte';
	import { goto } from '$app/navigation';
	import { ChartLine, RefreshCw, Terminal, Activity } from '@lucide/svelte';
	import PageLayout from '$lib/components/PageLayout.svelte';
	import { getAuthSession } from '$lib/auth';
	import * as Card from '$lib/components/ui/card';
	import * as Chart from '$lib/components/ui/chart/index.js';
	import { Button } from '$lib/components/ui/button';
	import { LineChart, AreaChart, Area } from 'layerchart';
	import { scaleUtc } from 'd3-scale';
	import { curveNatural } from 'd3-shape';

	const session = getAuthSession();
	if (!session) goto('/auth', { replaceState: true });

	const token = session?.tokens?.access_token || '';

	let cpuSeries = $state<{ date: Date; value: number }[]>([]);
	let memorySeries = $state<{ date: Date; usedGB: number }[]>([]);
	let networkSeries = $state<{ date: Date; inMB: number; outMB: number }[]>([]);
	let memTotalGB = $state(16);

	let logs = $state<string[]>([
		'[SYSTEM] Rustploy Container Monitor Connected',
		'[INFO] Listening on /api/monitoring/stream/logs for stdout/stderr...'
	]);

	let sseContainer: EventSource | null = null;
	let sseLog: EventSource | null = null;
	let isLive = $state(false);

	function parseDate(ts: any): Date {
		if (typeof ts === 'number') return new Date(ts > 1e11 ? ts : ts * 1000);
		if (typeof ts === 'string') {
			const d = new Date(ts);
			if (!isNaN(d.getTime())) return d;
		}
		return new Date();
	}

	async function loadHistoricalMetrics(activeToken: string) {
		try {
			const res = await fetch('/api/monitoring/server/1', {
				headers: { Authorization: `Bearer ${activeToken}` }
			});
			if (!res.ok) return;
			const data = await res.json();
			if (Array.isArray(data) && data.length > 0) {
				const sorted = [...data].reverse();
				cpuSeries = sorted.map((m: any) => ({
					date: parseDate(m.timestamp),
					value: Math.round(m.cpu || 0)
				}));
				memorySeries = sorted.map((m: any) => ({
					date: parseDate(m.timestamp),
					usedGB: +(m.mem_used_gb || (m.mem_used ? m.mem_used / 1024 : 0)).toFixed(2)
				}));
				networkSeries = sorted.map((m: any) => ({
					date: parseDate(m.timestamp),
					inMB: +(m.network_in || 0).toFixed(1),
					outMB: +(m.network_out || 0).toFixed(1)
				}));
				if (sorted[0].mem_total) {
					memTotalGB = Math.round(sorted[0].mem_total);
				}
			}
		} catch (err) {
			console.error('Failed to load REST historical metrics:', err);
		}
	}

	onMount(() => {
		const activeToken = getAuthSession()?.tokens?.access_token || token;
		loadHistoricalMetrics(activeToken);

		try {
			const sseUrl = `/api/monitoring/stream/containers?token=${encodeURIComponent(activeToken)}`;
			sseContainer = new EventSource(sseUrl);
			sseContainer.addEventListener('container-metric', (e) => {
				const data = JSON.parse(e.data);
				const pointTime = new Date(data.timestamp * 1000);
				isLive = true;

				cpuSeries = [
					...cpuSeries.slice(-19),
					{ date: pointTime, value: Math.round(data.cpu_percent) }
				];

				memorySeries = [
					...memorySeries.slice(-19),
					{ date: pointTime, usedGB: +(data.memory_used_mb / 1024).toFixed(2) }
				];

				networkSeries = [
					...networkSeries.slice(-19),
					{
						date: pointTime,
						inMB: +(data.net_rx_kbps / 1024).toFixed(1),
						outMB: +(data.net_tx_kbps / 1024).toFixed(1)
					}
				];
			});

			const logUrl = `/api/monitoring/stream/logs?token=${encodeURIComponent(activeToken)}`;
			sseLog = new EventSource(logUrl);
			sseLog.addEventListener('container-log', (e) => {
				const data = JSON.parse(e.data);
				const line = `[${data.container_id || 'CONTAINER'}] ${data.log_line}`;
				logs = [...logs.slice(-200), line];
			});
		} catch (err) {
			console.error('SSE initialization error:', err);
		}
	});

	onDestroy(() => {
		sseContainer?.close();
		sseLog?.close();
	});

	const currentCpu = $derived(cpuSeries.at(-1)?.value ?? 0);
	const currentMem = $derived(memorySeries.at(-1)?.usedGB ?? 0);
	const currentNet = $derived(networkSeries.at(-1) ?? { inMB: 0, outMB: 0 });

	const cpuConfig: Chart.ChartConfig = { value: { label: 'CPU %', color: 'var(--color-chart-1)' } };
	const memConfig: Chart.ChartConfig = {
		usedGB: { label: 'Memory GB', color: 'var(--color-chart-2)' }
	};
	const netConfig: Chart.ChartConfig = {
		inMB: { label: 'In MB', color: 'var(--color-chart-1)' },
		outMB: { label: 'Out MB', color: 'var(--color-chart-2)' }
	};

	const C = [
		'var(--chart-1)',
		'var(--chart-2)',
		'var(--chart-3)',
		'var(--chart-4)',
		'var(--chart-5)'
	];

	const xFmt = (v: Date) =>
		v.toLocaleTimeString('en-US', {
			hour: '2-digit',
			minute: '2-digit',
			second: '2-digit',
			hour12: false
		});

	let refreshing = $state(false);
	function refresh() {
		refreshing = true;
		const activeToken = getAuthSession()?.tokens?.access_token || token;
		loadHistoricalMetrics(activeToken).finally(() => {
			setTimeout(() => (refreshing = false), 600);
		});
	}
</script>

<PageLayout>
	<header class="flex items-center justify-between px-6 py-3 border-b border-border">
		<div class="flex items-center gap-2 text-sm">
			<ChartLine class="w-4 h-4 text-primary" />
			<span class="font-medium">Real-Time Monitoring</span>
			{#if isLive}
				<span class="flex items-center gap-1 text-xs text-emerald-500 font-medium px-2 py-0.5 rounded-full bg-emerald-500/10 border border-emerald-500/20">
					<Activity class="w-3 h-3 animate-pulse" /> SSE Connected
				</span>
			{/if}
		</div>
		<Button variant="outline" size="sm" class="h-7 text-xs gap-1.5" onclick={refresh}>
			<RefreshCw class="w-3 h-3 {refreshing ? 'animate-spin' : ''}" />
			Refresh
		</Button>
	</header>

	<main class="flex-1 p-6 overflow-y-auto space-y-6">
		<div class="space-y-1">
			<h1 class="text-2xl font-semibold tracking-tight">System & Container Monitoring</h1>
			<p class="text-sm text-muted-foreground">Real-time Server-Sent Events (SSE) telemetry stream & Docker logs</p>
		</div>

		<div class="grid gap-6 lg:grid-cols-2">
			<!-- 1. CPU -->
			<Card.Root class="bg-background">
				<Card.Header class="flex flex-row items-center justify-between space-y-0 pb-2">
					<Card.Title class="text-sm font-medium">CPU Usage</Card.Title>
					<span class="text-2xl font-bold" style="color:{C[0]}">{currentCpu}%</span>
				</Card.Header>
				<Card.Content>
					<div class="flex flex-col gap-2 w-full">
						<span class="text-sm text-muted-foreground">Used: {currentCpu}% of total capacity</span>
						<div class="w-full h-2 rounded-full bg-muted overflow-hidden">
							<div
								class="h-full rounded-full transition-all duration-500"
								style="width:{currentCpu}%; background:{C[0]}"
							></div>
						</div>
						<Chart.Container
							config={cpuConfig}
							class="mt-2 w-full ![aspect-ratio:unset]"
							style="height:160px"
						>
							{#if cpuSeries.length > 0}
								<LineChart
									data={cpuSeries}
									x="date"
									xScale={scaleUtc()}
									axis="x"
									yDomain={[0, 100]}
									series={[{ key: 'value', label: 'CPU %', color: cpuConfig.value.color }]}
									props={{
										spline: { curve: curveNatural, motion: 'tween', strokeWidth: 2 },
										xAxis: { format: xFmt },
										highlight: { points: { r: 4 } }
									}}
								>
									{#snippet tooltip()}<Chart.Tooltip hideLabel />{/snippet}
								</LineChart>
							{:else}
								<div class="flex items-center justify-center h-full text-xs text-muted-foreground">
									Loading real-time CPU metrics...
								</div>
							{/if}
						</Chart.Container>
					</div>
				</Card.Content>
			</Card.Root>

			<!-- 2. Memory -->
			<Card.Root class="bg-background">
				<Card.Header class="flex flex-row items-center justify-between space-y-0 pb-2">
					<Card.Title class="text-sm font-medium">Memory Usage</Card.Title>
					<span class="text-2xl font-bold" style="color:{C[1]}">{currentMem} GB</span>
				</Card.Header>
				<Card.Content>
					<div class="flex flex-col gap-2 w-full">
						<span class="text-sm text-muted-foreground">Used: {currentMem} GB / Limit: {memTotalGB} GB</span>
						<div class="w-full h-2 rounded-full bg-muted overflow-hidden">
							<div
								class="h-full rounded-full transition-all duration-500"
								style="width:{(currentMem / memTotalGB) * 100}%; background:{C[1]}"
							></div>
						</div>
						<Chart.Container
							config={memConfig}
							class="mt-2 w-full"
							style="height:160px;aspect-ratio:unset"
						>
							{#if memorySeries.length > 0}
								<LineChart
									data={memorySeries}
									x="date"
									xScale={scaleUtc()}
									axis="x"
									yDomain={[0, memTotalGB]}
									series={[{ key: 'usedGB', label: 'Memory GB', color: memConfig.usedGB.color }]}
									props={{
										spline: { curve: curveNatural, motion: 'tween', strokeWidth: 2 },
										xAxis: { format: xFmt },
										highlight: { points: { r: 4 } }
									}}
								>
									{#snippet tooltip()}<Chart.Tooltip hideLabel />{/snippet}
								</LineChart>
							{:else}
								<div class="flex items-center justify-center h-full text-xs text-muted-foreground">
									Loading real-time Memory metrics...
								</div>
							{/if}
						</Chart.Container>
					</div>
				</Card.Content>
			</Card.Root>
		</div>

		<!-- 3. Network & Live Log Terminal -->
		<div class="grid gap-6 lg:grid-cols-2">
			<!-- Network Chart -->
			<Card.Root class="bg-background flex flex-col">
				<Card.Header class="flex flex-row items-center justify-between space-y-0 pb-2">
					<Card.Title class="text-sm font-medium">Network I/O</Card.Title>
					<div class="flex items-center gap-4 text-xs font-medium">
						<div class="flex items-center gap-1.5">
							<span class="inline-block w-2.5 h-2.5 rounded-[2px] shrink-0" style="background:{C[0]}"></span>
							In: <strong class="text-foreground ml-0.5">{currentNet.inMB} MB/s</strong>
						</div>
						<div class="flex items-center gap-1.5">
							<span class="inline-block w-2.5 h-2.5 rounded-[2px] shrink-0" style="background:{C[1]}"></span>
							Out: <strong class="text-foreground ml-0.5">{currentNet.outMB} MB/s</strong>
						</div>
					</div>
				</Card.Header>
				<Card.Content class="flex-1 min-h-0 px-4 pb-4 pt-0">
					<Chart.Container config={netConfig} class="![aspect-ratio:unset] w-full h-[180px]">
						{#if networkSeries.length > 0}
							<AreaChart
								data={networkSeries}
								x="date"
								xScale={scaleUtc()}
								legend={false}
								series={[
									{ key: 'inMB', label: 'In MB', color: netConfig.inMB.color },
									{ key: 'outMB', label: 'Out MB', color: netConfig.outMB.color }
								]}
								props={{ xAxis: { format: xFmt }, yAxis: { format: () => '' } }}
							>
								{#snippet marks({ context })}
									<defs>
										<linearGradient id="fillIn" x1="0" y1="0" x2="0" y2="1">
											<stop offset="5%" stop-color="var(--color-inMB)" stop-opacity={1.0} />
											<stop offset="95%" stop-color="var(--color-inMB)" stop-opacity={0.1} />
										</linearGradient>
										<linearGradient id="fillOut" x1="0" y1="0" x2="0" y2="1">
											<stop offset="5%" stop-color="var(--color-outMB)" stop-opacity={0.8} />
											<stop offset="95%" stop-color="var(--color-outMB)" stop-opacity={0.1} />
										</linearGradient>
									</defs>
									{#each context.series.visibleSeries as s (s.key)}
										<Area
											seriesKey={s.key}
											curve={curveNatural}
											fillOpacity={0.4}
											line={{ class: 'stroke-2' }}
											motion="tween"
											{...s.props}
											fill={s.key === 'inMB' ? 'url(#fillIn)' : 'url(#fillOut)'}
										/>
									{/each}
								{/snippet}
								{#snippet tooltip()}
									<Chart.Tooltip
										labelFormatter={(v: Date) => v.toLocaleTimeString()}
										indicator="line"
									/>
								{/snippet}
							</AreaChart>
						{:else}
							<div class="flex items-center justify-center h-full text-xs text-muted-foreground">
								Loading real-time Network metrics...
							</div>
						{/if}
					</Chart.Container>
				</Card.Content>
			</Card.Root>

			<!-- Live Log Terminal Output -->
			<Card.Root class="bg-black text-emerald-400 font-mono text-xs border border-border flex flex-col h-[280px]">
				<Card.Header class="flex flex-row items-center justify-between py-2 px-4 border-b border-zinc-800 bg-zinc-950">
					<div class="flex items-center gap-2">
						<Terminal class="w-4 h-4 text-emerald-400" />
						<span class="font-medium text-zinc-300 text-xs">Live Docker Container Stream</span>
					</div>
					<span class="text-[10px] text-zinc-500">Auto-tailing</span>
				</Card.Header>
				<Card.Content class="p-3 overflow-y-auto flex-1 font-mono space-y-1">
					{#each logs as log}
						<div class="leading-relaxed break-all hover:bg-zinc-900/50 px-1 rounded transition-colors">
							{log}
						</div>
					{/each}
				</Card.Content>
			</Card.Root>
		</div>
	</main>
</PageLayout>

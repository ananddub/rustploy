<script lang="ts">
	import { goto } from '$app/navigation';
	import {
		ArrowDownUp, Search, ChevronDown, Download, Copy,
		CheckCircle2, TrendingUp, Globe, Server as ServerIcon,
		Info, AlertCircle, RefreshCw, X
	} from '@lucide/svelte';
	import PageLayout from '$lib/components/PageLayout.svelte';
	import { getAuthSession } from '$lib/auth';
	import * as Card from '$lib/components/ui/card';
	import * as Table from '$lib/components/ui/table';
	import * as Chart from '$lib/components/ui/chart/index.js';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import * as Select from '$lib/components/ui/select';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { Badge } from '$lib/components/ui/badge';
	import { Label } from '$lib/components/ui/label';
	import { AreaChart } from 'layerchart';
	import { scaleUtc } from 'd3-scale';
	import { toastSuccess, toastError } from '$lib/toast';

	const session = getAuthSession();
	if (!session) goto('/auth', { replaceState: true });

	// ─── Types ────────────────────────────────────────────────────────────────────
	type LogEntry = {
		id: number;
		level: string;
		RequestMethod: string;
		RequestAddr: string;
		RequestPath: string;
		DownstreamStatus: number;
		Duration: number; // nanoseconds
		ClientAddr: string;
		StartUTC: string;
		RouterName?: string;
		ServiceName?: string;
	};

	// ─── Static data ──────────────────────────────────────────────────────────────
	const now = Date.now();
	const allLogs: LogEntry[] = [
		{ id:1,  level:'info',    RequestMethod:'GET',    RequestAddr:'api.example.com', RequestPath:'/api/v1/projects',         DownstreamStatus:200, Duration:45_000_000,  ClientAddr:'10.0.0.1:52100', StartUTC: new Date(now - 60000).toISOString(),  RouterName:'api-router',    ServiceName:'api-service' },
		{ id:2,  level:'info',    RequestMethod:'POST',   RequestAddr:'api.example.com', RequestPath:'/api/v1/deployments',      DownstreamStatus:201, Duration:120_000_000, ClientAddr:'10.0.0.2:43210', StartUTC: new Date(now - 75000).toISOString(),  RouterName:'api-router',    ServiceName:'api-service' },
		{ id:3,  level:'info',    RequestMethod:'GET',    RequestAddr:'api.example.com', RequestPath:'/api/v1/monitoring/stats', DownstreamStatus:200, Duration:32_000_000,  ClientAddr:'10.0.0.1:52101', StartUTC: new Date(now - 90000).toISOString(),  RouterName:'api-router',    ServiceName:'api-service' },
		{ id:4,  level:'warning', RequestMethod:'DELETE', RequestAddr:'api.example.com', RequestPath:'/api/v1/applications/5',  DownstreamStatus:404, Duration:12_000_000,  ClientAddr:'10.0.0.3:61000', StartUTC: new Date(now - 110000).toISOString(), RouterName:'api-router',    ServiceName:'api-service' },
		{ id:5,  level:'info',    RequestMethod:'GET',    RequestAddr:'app.example.com', RequestPath:'/health',                 DownstreamStatus:200, Duration:5_000_000,   ClientAddr:'10.0.0.1:52102', StartUTC: new Date(now - 180000).toISOString(), RouterName:'app-router',    ServiceName:'app-service' },
		{ id:6,  level:'warning', RequestMethod:'POST',   RequestAddr:'api.example.com', RequestPath:'/api/v1/auth/login',      DownstreamStatus:401, Duration:89_000_000,  ClientAddr:'10.0.0.4:55000', StartUTC: new Date(now - 270000).toISOString(), RouterName:'api-router',    ServiceName:'api-service' },
		{ id:7,  level:'error',   RequestMethod:'GET',    RequestAddr:'api.example.com', RequestPath:'/api/v1/users/99',        DownstreamStatus:500, Duration:203_000_000, ClientAddr:'10.0.0.2:43211', StartUTC: new Date(now - 300000).toISOString(), RouterName:'api-router',    ServiceName:'api-service' },
		{ id:8,  level:'info',    RequestMethod:'PUT',    RequestAddr:'api.example.com', RequestPath:'/api/v1/projects/3',      DownstreamStatus:200, Duration:67_000_000,  ClientAddr:'10.0.0.1:52103', StartUTC: new Date(now - 360000).toISOString(), RouterName:'api-router',    ServiceName:'api-service' },
		{ id:9,  level:'info',    RequestMethod:'GET',    RequestAddr:'app.example.com', RequestPath:'/',                       DownstreamStatus:200, Duration:18_000_000,  ClientAddr:'10.0.0.5:60000', StartUTC: new Date(now - 420000).toISOString(), RouterName:'app-router',    ServiceName:'app-service' },
		{ id:10, level:'info',    RequestMethod:'PATCH',  RequestAddr:'api.example.com', RequestPath:'/api/v1/settings',        DownstreamStatus:200, Duration:55_000_000,  ClientAddr:'10.0.0.1:52104', StartUTC: new Date(now - 500000).toISOString(), RouterName:'api-router',    ServiceName:'api-service' },
		{ id:11, level:'warning', RequestMethod:'GET',    RequestAddr:'api.example.com', RequestPath:'/api/v1/old-endpoint',    DownstreamStatus:301, Duration:8_000_000,   ClientAddr:'10.0.0.2:43212', StartUTC: new Date(now - 560000).toISOString(), RouterName:'api-router',    ServiceName:'api-service' },
		{ id:12, level:'error',   RequestMethod:'POST',   RequestAddr:'api.example.com', RequestPath:'/api/v1/deploy',          DownstreamStatus:503, Duration:5001_000_000,ClientAddr:'10.0.0.3:61001', StartUTC: new Date(now - 620000).toISOString(), RouterName:'api-router',    ServiceName:'api-service' },
	];

	// Chart data: hourly request counts (last 12 hours)
	const chartData = Array.from({ length: 12 }, (_, i) => ({
		date: new Date(now - (11 - i) * 3600_000),
		count: Math.floor(Math.random() * 80) + 10,
	}));

	const chartConfig: Chart.ChartConfig = {
		count: { label: 'Requests', color: 'var(--color-chart-1)' }
	};

	// ─── State ────────────────────────────────────────────────────────────────────
	let isActive       = $state(true);
	let cronExpression = $state('0 0 * * *');
	let search         = $state('');
	let statusFilter   = $state<string[]>([]);
	let selectedRow    = $state<LogEntry | null>(null);
	let page           = $state(1);
	let pageSize       = $state(10);
	let sortKey        = $state<keyof LogEntry>('StartUTC');
	let sortDir        = $state<'asc'|'desc'>('desc');

	// Column visibility
	let visibleCols = $state({ level: true, message: true, time: true });

	// ─── Helpers ──────────────────────────────────────────────────────────────────
	function fmtDuration(ns: number) {
		const ms = ns / 1_000_000;
		if (ms < 1)    return `${(ns / 1000).toFixed(2)} µs`;
		if (ms < 1000) return `${ms.toFixed(2)} ms`;
		return `${(ms / 1000).toFixed(2)} s`;
	}

	function fmtTime(iso: string) {
		return new Date(iso).toLocaleString(undefined, { month:'short', day:'numeric', hour:'2-digit', minute:'2-digit', second:'2-digit' });
	}

	function statusVariant(s: number): 'default' | 'secondary' | 'outline' | 'destructive' {
		if (s === 0)             return 'secondary';
		if (s >= 200 && s < 300) return 'default';
		if (s >= 300 && s < 400) return 'outline';
		if (s >= 400)            return 'destructive';
		return 'outline';
	}

	function methodColor(m: string) {
		const map: Record<string,string> = { GET:'text-green-400', POST:'text-blue-400', PUT:'text-yellow-400', PATCH:'text-purple-400', DELETE:'text-red-400' };
		return map[m] ?? 'text-foreground';
	}

	function statusCategory(s: number) {
		if (s >= 100 && s < 200) return 'info';
		if (s >= 200 && s < 300) return 'success';
		if (s >= 300 && s < 400) return 'redirect';
		if (s >= 400 && s < 500) return 'client';
		if (s >= 500)            return 'server';
		return '';
	}

	const statusOptions = [
		{ label: '1xx Info',     value: 'info',     icon: Info        },
		{ label: '2xx Success',  value: 'success',  icon: CheckCircle2 },
		{ label: '3xx Redirect', value: 'redirect', icon: TrendingUp  },
		{ label: '4xx Client',   value: 'client',   icon: Globe       },
		{ label: '5xx Server',   value: 'server',   icon: ServerIcon  },
	];

	// ─── Derived ──────────────────────────────────────────────────────────────────
	const filtered = $derived.by(() => {
		let list = allLogs.filter(r => {
			const q = search.toLowerCase();
			const matchSearch = !search || r.RequestPath.toLowerCase().includes(q) || r.RequestAddr.toLowerCase().includes(q) || r.ClientAddr.toLowerCase().includes(q);
			const matchStatus = statusFilter.length === 0 || statusFilter.includes(statusCategory(r.DownstreamStatus));
			return matchSearch && matchStatus;
		});
		list = [...list].sort((a, b) => {
			const av = String(a[sortKey] ?? '');
			const bv = String(b[sortKey] ?? '');
			return sortDir === 'asc' ? av.localeCompare(bv) : bv.localeCompare(av);
		});
		return list;
	});

	const totalPages = $derived(Math.max(1, Math.ceil(filtered.length / pageSize)));
	const paginated  = $derived(filtered.slice((page-1)*pageSize, page*pageSize));
	const showFrom   = $derived(filtered.length === 0 ? 0 : (page-1)*pageSize + 1);
	const showTo     = $derived(Math.min(page*pageSize, filtered.length));

	$effect(() => { search; statusFilter; pageSize; page = 1; });

	function downloadRow(row: LogEntry) {
		const blob = new Blob([JSON.stringify(row, null, 2)], { type: 'application/json' });
		const url  = URL.createObjectURL(blob);
		const a    = document.createElement('a');
		a.href = url; a.download = 'log.json'; a.click();
		URL.revokeObjectURL(url);
	}

	function copyAddr(addr: string) {
		navigator.clipboard.writeText(addr);
		toastSuccess('Copied to clipboard');
	}

	function toggleStatus(val: string) {
		statusFilter = statusFilter.includes(val)
			? statusFilter.filter(v => v !== val)
			: [...statusFilter, val];
	}
</script>

<PageLayout>
	<header class="flex items-center gap-2 px-6 py-3 border-b border-border text-sm">
		<ArrowDownUp class="w-4 h-4 text-muted-foreground" />
		<span class="font-medium">Requests</span>
	</header>

	<main class="flex-1 p-6 overflow-y-auto">
		<div class="rounded-xl border border-border bg-card">

			<!-- Header -->
			<div class="px-6 pt-5 pb-4 border-b border-border">
				<h1 class="text-xl font-bold flex items-center gap-2">
					<ArrowDownUp class="w-5 h-5" /> Requests
				</h1>
				<p class="text-sm text-muted-foreground mt-0.5">
					See all the incoming requests that pass through Traefik
				</p>
			</div>

			<div class="px-6 py-5 space-y-5">

				<!-- Controls row: cron + activate -->
				<div class="flex flex-wrap items-end gap-4 justify-between">
					<div class="flex items-center gap-3 flex-wrap">
						<div class="flex flex-col gap-1">
							<Label for="cron" class="text-sm text-muted-foreground flex items-center gap-1.5">
								Log Cleanup Schedule
								<span title="At the scheduled time, the cleanup job will keep only the last 1000 entries in the access log file." class="cursor-help opacity-50">ⓘ</span>
							</Label>
							<div class="flex items-center gap-2">
								<Input id="cron" bind:value={cronExpression} placeholder="0 0 * * *" class="h-8 w-36 font-mono text-xs" />
								<Button variant="outline" size="sm" class="h-8 text-xs"
									onclick={() => toastSuccess('Log cleanup schedule updated')}>
									Update Schedule
								</Button>
							</div>
						</div>
					</div>
					<Button
						variant={isActive ? 'destructive' : 'default'}
						size="sm"
						class="h-8 text-xs"
						onclick={() => { isActive = !isActive; toastSuccess(`Requests ${isActive ? 'activated' : 'deactivated'}`); }}
					>
						{isActive ? 'Deactivate' : 'Activate'}
					</Button>
				</div>

				{#if !isActive}
					<!-- Inactive state -->
					<div class="flex flex-col items-center justify-center py-20 gap-4 text-muted-foreground">
						<AlertCircle class="w-12 h-12 opacity-40" />
						<div class="text-center space-y-2">
							<h3 class="text-lg font-medium">Requests are not activated</h3>
							<p class="text-sm max-w-md opacity-70">
								Activate requests to see incoming traffic statistics and monitor your application's usage.
								After activation, you'll need to reload Traefik for the changes to take effect.
							</p>
						</div>
					</div>
				{:else}
					<!-- Chart -->
					<div>
						<Chart.Container config={chartConfig} class="w-full" style="height:200px;aspect-ratio:unset">
							<AreaChart
								data={chartData}
								x="date"
								xScale={scaleUtc()}
								legend={false}
								series={[{ key:'count', label:'Requests', color: chartConfig.count.color }]}
								props={{ xAxis:{ format:(v: Date) => v.toLocaleTimeString('en-US',{hour:'2-digit',minute:'2-digit',hour12:false}) }, yAxis:{} }}
							/>
						</Chart.Container>
					</div>

					<!-- Filters toolbar -->
					<div class="flex items-center gap-2 flex-wrap">
						<div class="relative flex-1 min-w-[200px] max-w-xs">
							<Search class="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-muted-foreground" />
							<Input bind:value={search} placeholder="Filter by hostname, path, IP…" class="pl-9 h-9" />
						</div>

						<!-- Status faceted filter -->
						<DropdownMenu.Root>
							<DropdownMenu.Trigger>
								<Button variant="outline" class="h-9 gap-1.5 text-sm">
									Status
									{#if statusFilter.length > 0}
										<span class="ml-1 rounded bg-primary text-primary-foreground text-[10px] px-1.5 py-0.5">{statusFilter.length}</span>
									{/if}
									<ChevronDown class="w-4 h-4 ml-1" />
								</Button>
							</DropdownMenu.Trigger>
							<DropdownMenu.Content align="start" class="w-44">
								{#each statusOptions as opt (opt.value)}
									<DropdownMenu.CheckboxItem
										checked={statusFilter.includes(opt.value)}
										onCheckedChange={() => toggleStatus(opt.value)}
										class="gap-2 text-xs"
									>
										<opt.icon class="w-3.5 h-3.5 text-muted-foreground" />
										{opt.label}
									</DropdownMenu.CheckboxItem>
								{/each}
								{#if statusFilter.length > 0}
									<DropdownMenu.Separator />
									<DropdownMenu.Item class="text-sm justify-center text-muted-foreground" onclick={() => (statusFilter = [])}>
										Clear filters
									</DropdownMenu.Item>
								{/if}
							</DropdownMenu.Content>
						</DropdownMenu.Root>

						<!-- Column visibility -->
						<DropdownMenu.Root>
							<DropdownMenu.Trigger>
								<Button variant="outline" class="h-9 gap-1.5 text-sm sm:ml-auto">
									Columns <ChevronDown class="w-4 h-4" />
								</Button>
							</DropdownMenu.Trigger>
							<DropdownMenu.Content align="end">
								<DropdownMenu.CheckboxItem checked={visibleCols.level}   onCheckedChange={(v) => (visibleCols.level   = !!v)} class="text-xs">Level</DropdownMenu.CheckboxItem>
								<DropdownMenu.CheckboxItem checked={visibleCols.message} onCheckedChange={(v) => (visibleCols.message = !!v)} class="text-xs">Message</DropdownMenu.CheckboxItem>
								<DropdownMenu.CheckboxItem checked={visibleCols.time}    onCheckedChange={(v) => (visibleCols.time    = !!v)} class="text-xs">Time</DropdownMenu.CheckboxItem>
							</DropdownMenu.Content>
						</DropdownMenu.Root>
					</div>

					<!-- Table -->
					<div class="rounded-md border border-border overflow-x-auto">
						{#if filtered.length === 0}
							<div class="flex flex-col items-center justify-center py-20 text-muted-foreground gap-2">
								<ArrowDownUp class="w-10 h-10 opacity-20" />
								<p class="text-base font-medium">No results</p>
							</div>
						{:else}
							<Table.Root>
								<Table.Header>
									<Table.Row>
										{#if visibleCols.level}
											<Table.Head class="text-sm w-20">Level</Table.Head>
										{/if}
										{#if visibleCols.message}
											<Table.Head class="text-sm">Message</Table.Head>
										{/if}
										{#if visibleCols.time}
											<Table.Head class="text-sm w-44">
												<button class="flex items-center gap-1 hover:text-foreground transition-colors"
													onclick={() => { sortKey = 'StartUTC'; sortDir = sortDir === 'asc' ? 'desc' : 'asc'; }}>
													Time <span class="opacity-40 text-[10px]">{sortKey === 'StartUTC' ? (sortDir === 'asc' ? '↑' : '↓') : '↕'}</span>
												</button>
											</Table.Head>
										{/if}
									</Table.Row>
								</Table.Header>
								<Table.Body>
									{#each paginated as row (row.id)}
										<Table.Row
											class="cursor-pointer hover:bg-muted/30 transition-colors"
											onclick={() => (selectedRow = row)}
										>
											{#if visibleCols.level}
												<Table.Cell>
													<Badge variant="secondary" class="text-[10px] capitalize">{row.level}</Badge>
												</Table.Cell>
											{/if}
											{#if visibleCols.message}
												<Table.Cell>
													<div class="flex flex-col gap-1.5">
														<div class="flex items-center gap-2 flex-wrap text-sm">
															<span class="font-mono font-bold text-xs {methodColor(row.RequestMethod)}">{row.RequestMethod}</span>
															<span class="bg-muted px-2 py-0.5 rounded text-xs font-mono text-muted-foreground">{row.RequestAddr}</span>
															<span class="text-xs font-mono break-all">{row.RequestPath}</span>
														</div>
														<div class="flex items-center gap-2 flex-wrap">
															<Badge variant={statusVariant(row.DownstreamStatus)} class="text-[10px]">
																Status: {row.DownstreamStatus}
															</Badge>
															<Badge variant="secondary" class="text-[10px]">Exec Time: {fmtDuration(row.Duration)}</Badge>
															<Badge variant="secondary" class="text-[10px]">IP: {row.ClientAddr}</Badge>
														</div>
													</div>
												</Table.Cell>
											{/if}
											{#if visibleCols.time}
												<Table.Cell class="text-sm text-muted-foreground whitespace-nowrap">{fmtTime(row.StartUTC)}</Table.Cell>
											{/if}
										</Table.Row>
									{/each}
								</Table.Body>
							</Table.Root>
						{/if}
					</div>

					<!-- Pagination -->
					{#if filtered.length > 0}
						<div class="flex flex-col gap-3 sm:flex-row sm:items-center sm:justify-between">
							<span class="text-sm text-muted-foreground">
								Showing {showFrom}–{showTo} of {filtered.length} entries
							</span>
							<div class="flex items-center gap-2">
								<Button variant="outline" size="sm" class="h-8" disabled={page <= 1} onclick={() => page--}>Previous</Button>
								<span class="text-xs text-muted-foreground tabular-nums">{page} / {totalPages}</span>
								<Button variant="outline" size="sm" class="h-8" disabled={page >= totalPages} onclick={() => page++}>Next</Button>
							</div>
						</div>
					{/if}
				{/if}
			</div>
		</div>
	</main>
</PageLayout>

<!-- ─── Detail side sheet ─────────────────────────────────────────────────── -->
{#if selectedRow}
	<div class="fixed inset-0 bg-black/40 z-40" role="button" tabindex="-1"
		onclick={() => (selectedRow = null)} onkeydown={() => {}}></div>
	<div class="fixed inset-y-0 right-0 z-50 w-full max-w-[740px] bg-card border-l border-border flex flex-col shadow-2xl">
		<!-- Sheet header -->
		<div class="flex items-start justify-between px-6 py-4 border-b border-border">
			<div>
				<h2 class="text-base font-semibold">Request log</h2>
				<p class="text-sm text-muted-foreground mt-0.5">Details of the request log entry</p>
			</div>
			<button onclick={() => (selectedRow = null)}
				class="text-muted-foreground hover:text-foreground hover:bg-accent p-1.5 rounded transition-colors">
				<X class="w-4 h-4" />
			</button>
		</div>

		<!-- Sheet body -->
		<div class="flex-1 overflow-y-auto px-6 py-4">
			<div class="rounded-md border border-border overflow-hidden">
				<Table.Root>
					<Table.Body>
						{#each Object.entries(selectedRow) as [key, value] (key)}
							<Table.Row>
								<Table.Cell class="font-medium text-sm w-40 text-muted-foreground">{key}</Table.Cell>
								<Table.Cell class="text-sm font-mono break-all whitespace-pre-wrap">
									{#if key === 'RequestAddr' || key === 'ClientAddr'}
										<div class="flex items-center gap-2 bg-muted px-2 py-1 rounded">
											<span>{value}</span>
											<button onclick={() => copyAddr(String(value))} class="shrink-0 text-muted-foreground hover:text-foreground">
												<Copy class="w-3.5 h-3.5" />
											</button>
										</div>
									{:else if key === 'DownstreamStatus'}
										<Badge variant={statusVariant(Number(value))} class="text-[10px]">{value}</Badge>
									{:else if key === 'Duration'}
										<Badge variant="secondary" class="text-[10px]">{fmtDuration(Number(value))}</Badge>
									{:else if key === 'RequestMethod'}
										<Badge variant="outline" class="text-[10px] {methodColor(String(value))}">{value}</Badge>
									{:else if key === 'level'}
										<Badge variant="secondary" class="text-[10px] capitalize">{value}</Badge>
									{:else if key === 'StartUTC'}
										{fmtTime(String(value))}
									{:else}
										{value}
									{/if}
								</Table.Cell>
							</Table.Row>
						{/each}
					</Table.Body>
				</Table.Root>
			</div>
		</div>

		<!-- Sheet footer -->
		<div class="px-6 py-4 border-t border-border">
			<Button variant="outline" class="w-full gap-2" onclick={() => downloadRow(selectedRow!)}>
				<Download class="w-4 h-4" /> Download as JSON
			</Button>
		</div>
	</div>
{/if}

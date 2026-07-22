<script lang="ts">
	import { goto } from '$app/navigation';
	import {
		Rocket, Boxes, Search, ArrowUpDown, ChevronLeft, ChevronRight,
		ExternalLink, Loader2, ListTodo, Server, XCircle, RefreshCw
	} from '@lucide/svelte';
	import PageLayout from '$lib/components/PageLayout.svelte';
	import { getAuthSession } from '$lib/auth';
	import * as Card from '$lib/components/ui/card';
	import * as Table from '$lib/components/ui/table';
	import * as Tabs from '$lib/components/ui/tabs';
	import * as Select from '$lib/components/ui/select';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { Badge } from '$lib/components/ui/badge';

	const session = getAuthSession();
	if (!session) goto('/auth', { replaceState: true });

	// ─── Types ────────────────────────────────────────────────────────────────────
	type DeployStatus = 'running' | 'done' | 'error' | 'cancelled';
	type ServiceType = 'Application' | 'Compose';

	type Deployment = {
		id: string;
		service: string;
		serviceType: ServiceType;
		project: string;
		environment: string;
		server: string;
		title: string;
		status: DeployStatus;
		createdAt: string;
		href: string;
	};

	type QueueJob = {
		id: string;
		label: string;
		type: string;
		state: 'active' | 'waiting' | 'completed' | 'failed' | 'delayed' | 'paused';
		added: string;
		processed: string;
		finished: string;
		error: string;
	};

	// ─── Static data ──────────────────────────────────────────────────────────────
	const allDeployments: Deployment[] = [
		{ id: 'd-001', service: 'rustploy-app',   serviceType: 'Application', project: 'main-app',    environment: 'production',  server: 'VPS-1',    title: 'Deploy v1.4.2',         status: 'done',      createdAt: '2026-07-13 13:45:00', href: '/projects/1/app/1' },
		{ id: 'd-002', service: 'api-service',    serviceType: 'Application', project: 'main-app',    environment: 'production',  server: 'VPS-1',    title: 'Hotfix: auth bug',      status: 'done',      createdAt: '2026-07-13 12:30:00', href: '/projects/1/app/2' },
		{ id: 'd-003', service: 'worker-service', serviceType: 'Application', project: 'main-app',    environment: 'staging',     server: 'VPS-2',    title: 'Add queue processor',   status: 'running',   createdAt: '2026-07-13 14:00:00', href: '/projects/1/app/3' },
		{ id: 'd-004', service: 'full-stack',     serviceType: 'Compose',     project: 'ecommerce',   environment: 'production',  server: 'VPS-1',    title: 'Stack update',          status: 'done',      createdAt: '2026-07-13 11:00:00', href: '/projects/2/compose/1' },
		{ id: 'd-005', service: 'data-pipeline',  serviceType: 'Compose',     project: 'analytics',   environment: 'production',  server: 'VPS-3',    title: 'New pipeline config',   status: 'error',     createdAt: '2026-07-13 10:15:00', href: '/projects/3/compose/2' },
		{ id: 'd-006', service: 'frontend',       serviceType: 'Application', project: 'ecommerce',   environment: 'staging',     server: 'VPS-2',    title: 'UI redesign',           status: 'cancelled', createdAt: '2026-07-13 09:00:00', href: '/projects/2/app/4' },
		{ id: 'd-007', service: 'backend',        serviceType: 'Application', project: 'ecommerce',   environment: 'staging',     server: 'VPS-2',    title: 'API v2 migration',      status: 'done',      createdAt: '2026-07-13 08:30:00', href: '/projects/2/app/5' },
		{ id: 'd-008', service: 'monitoring',     serviceType: 'Compose',     project: 'infra',       environment: 'production',  server: 'VPS-1',    title: 'Prometheus + Grafana',  status: 'done',      createdAt: '2026-07-12 22:00:00', href: '/projects/4/compose/3' },
		{ id: 'd-009', service: 'auth-service',   serviceType: 'Application', project: 'main-app',    environment: 'production',  server: 'VPS-1',    title: 'JWT refresh fix',       status: 'error',     createdAt: '2026-07-12 20:45:00', href: '/projects/1/app/6' },
		{ id: 'd-010', service: 'notifications',  serviceType: 'Application', project: 'main-app',    environment: 'staging',     server: 'VPS-2',    title: 'Email templates update',status: 'done',      createdAt: '2026-07-12 18:00:00', href: '/projects/1/app/7' },
		{ id: 'd-011', service: 'cdn-proxy',      serviceType: 'Application', project: 'infra',       environment: 'production',  server: 'VPS-3',    title: 'Cache invalidation',    status: 'running',   createdAt: '2026-07-13 14:02:00', href: '/projects/4/app/8' },
		{ id: 'd-012', service: 'db-cluster',     serviceType: 'Compose',     project: 'analytics',   environment: 'production',  server: 'VPS-3',    title: 'Replica set config',    status: 'done',      createdAt: '2026-07-12 16:00:00', href: '/projects/3/compose/4' },
	];

	const queueJobs: QueueJob[] = [
		{ id: 'q-8821', label: 'Deploy worker-service',  type: 'application', state: 'active',    added: '2026-07-13 14:00:00', processed: '2026-07-13 14:00:05', finished: '—',                    error: '' },
		{ id: 'q-8820', label: 'Deploy cdn-proxy',       type: 'application', state: 'waiting',   added: '2026-07-13 14:02:00', processed: '—',                   finished: '—',                    error: '' },
		{ id: 'q-8819', label: 'Deploy auth-service',    type: 'application', state: 'failed',    added: '2026-07-12 20:45:00', processed: '2026-07-12 20:45:10', finished: '2026-07-12 20:46:00', error: 'Build failed: cannot find module' },
		{ id: 'q-8818', label: 'Stack update full-stack',type: 'compose',     state: 'completed', added: '2026-07-13 11:00:00', processed: '2026-07-13 11:00:08', finished: '2026-07-13 11:03:22', error: '' },
		{ id: 'q-8817', label: 'New pipeline config',    type: 'compose',     state: 'delayed',   added: '2026-07-13 10:15:00', processed: '—',                   finished: '—',                    error: '' },
	];

	// ─── State ────────────────────────────────────────────────────────────────────
	let activeTab    = $state('deployments');
	let search       = $state('');
	let statusFilter = $state('all');
	let typeFilter   = $state('all');
	let sortKey      = $state<keyof Deployment>('createdAt');
	let sortDir      = $state<'asc'|'desc'>('desc');
	let page         = $state(1);
	let pageSize     = $state(10);
	let refreshing   = $state(false);

	// ─── Derived ──────────────────────────────────────────────────────────────────
	const filtered = $derived.by(() => {
		let list = allDeployments.filter(d => {
			const q = search.toLowerCase();
			const matchSearch = !search || [d.service, d.project, d.environment, d.server, d.title].some(v => v.toLowerCase().includes(q));
			const matchStatus = statusFilter === 'all' || d.status === statusFilter;
			const matchType   = typeFilter   === 'all' || d.serviceType.toLowerCase() === typeFilter;
			return matchSearch && matchStatus && matchType;
		});
		list = [...list].sort((a, b) => {
			const av = a[sortKey] ?? '';
			const bv = b[sortKey] ?? '';
			return sortDir === 'asc' ? av.localeCompare(bv) : bv.localeCompare(av);
		});
		return list;
	});

	const totalPages = $derived(Math.max(1, Math.ceil(filtered.length / pageSize)));
	const paginated  = $derived(filtered.slice((page-1)*pageSize, page*pageSize));
	const showFrom   = $derived(filtered.length === 0 ? 0 : (page-1)*pageSize + 1);
	const showTo     = $derived(Math.min(page*pageSize, filtered.length));

	$effect(() => { search; statusFilter; typeFilter; pageSize; page = 1; });

	function toggleSort(key: keyof Deployment) {
		if (sortKey === key) sortDir = sortDir === 'asc' ? 'desc' : 'asc';
		else { sortKey = key; sortDir = 'asc'; }
	}

	function refresh() { refreshing = true; setTimeout(() => (refreshing = false), 800); }

	// ─── Badge helpers ────────────────────────────────────────────────────────────
	function deployBadge(status: string) {
		if (status === 'done')      return 'bg-green-500/15 text-green-500 border-green-500/20';
		if (status === 'running')   return 'bg-yellow-500/15 text-yellow-500 border-yellow-500/20';
		if (status === 'error')     return 'bg-red-500/15 text-red-500 border-red-500/20';
		return 'bg-muted text-muted-foreground border-border';
	}

	function queueBadge(state: string) {
		if (state === 'active')    return 'bg-yellow-500/15 text-yellow-500 border-yellow-500/20';
		if (state === 'completed') return 'bg-green-500/15 text-green-500 border-green-500/20';
		if (state === 'failed')    return 'bg-red-500/15 text-red-500 border-red-500/20';
		return 'bg-muted text-muted-foreground border-border';
	}
</script>

<PageLayout>
	<header class="flex items-center gap-2 px-6 py-3 border-b border-border text-sm">
		<Rocket class="w-4 h-4 text-muted-foreground" />
		<span class="font-medium">Deployments</span>
	</header>

	<main class="flex-1 p-6 overflow-y-auto">
		<div class="rounded-xl border border-border bg-card">

			<!-- Header -->
			<div class="flex items-start justify-between gap-3 flex-wrap px-6 pt-6 pb-4 border-b border-border">
				<div>
					<h1 class="text-xl font-bold flex items-center gap-2">
						<Rocket class="w-5 h-5" /> Deployments
					</h1>
					<p class="text-sm text-muted-foreground mt-0.5">All application and compose deployments in one place</p>
				</div>
				<Button variant="outline" size="sm" class="gap-1.5 h-8" onclick={refresh}>
					<RefreshCw class="w-3.5 h-3.5 {refreshing ? 'animate-spin' : ''}" />
					Refresh
				</Button>
			</div>

			<!-- Tabs -->
			<div class="px-6 pt-4">
				<Tabs.Root value={activeTab} onValueChange={(v) => (activeTab = v)}>
					<Tabs.List>
						<Tabs.Trigger value="deployments">Deployments</Tabs.Trigger>
						<Tabs.Trigger value="queue">Queue</Tabs.Trigger>
					</Tabs.List>

					<!-- ── Deployments Tab ── -->
					<Tabs.Content value="deployments" class="pt-4 pb-6 space-y-4">

						<!-- Filters -->
						<div class="flex items-center gap-2 flex-wrap">
							<div class="relative flex-1 min-w-[200px] max-w-xs">
								<Search class="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-muted-foreground" />
								<Input bind:value={search} placeholder="Search by name, project, server…" class="pl-9 h-9" />
							</div>
							<Select.Root type="single" value={statusFilter} onValueChange={(v) => (statusFilter = v ?? 'all')}>
								<Select.Trigger class="w-36 h-9">
									<span class="text-sm capitalize">{statusFilter === 'all' ? 'All statuses' : statusFilter}</span>
								</Select.Trigger>
								<Select.Content>
									<Select.Item value="all">All statuses</Select.Item>
									<Select.Item value="running">Running</Select.Item>
									<Select.Item value="done">Done</Select.Item>
									<Select.Item value="error">Error</Select.Item>
									<Select.Item value="cancelled">Cancelled</Select.Item>
								</Select.Content>
							</Select.Root>
							<Select.Root type="single" value={typeFilter} onValueChange={(v) => (typeFilter = v ?? 'all')}>
								<Select.Trigger class="w-36 h-9">
									<span class="text-sm capitalize">{typeFilter === 'all' ? 'All types' : typeFilter}</span>
								</Select.Trigger>
								<Select.Content>
									<Select.Item value="all">All types</Select.Item>
									<Select.Item value="application">Application</Select.Item>
									<Select.Item value="compose">Compose</Select.Item>
								</Select.Content>
							</Select.Root>
						</div>

						<!-- Table -->
						<div class="rounded-md border border-border overflow-x-auto">
							{#if filtered.length === 0}
								<div class="flex flex-col items-center justify-center py-20 text-muted-foreground gap-2">
									<Rocket class="w-10 h-10 opacity-20" />
									<p class="font-medium">No deployments found</p>
									<p class="text-sm opacity-60">Deployments from applications and compose will appear here</p>
								</div>
							{:else}
								<Table.Root>
									<Table.Header>
										<Table.Row>
											{#each [
												{ key: 'service',     label: 'Service'     },
												{ key: 'project',     label: 'Project'     },
												{ key: 'environment', label: 'Environment' },
												{ key: 'server',      label: 'Server'      },
												{ key: 'title',       label: 'Title'       },
												{ key: 'status',      label: 'Status'      },
												{ key: 'createdAt',   label: 'Created'     },
											] as col (col.key)}
												<Table.Head>
													<button
														class="flex items-center gap-1.5 text-xs font-medium hover:text-foreground transition-colors whitespace-nowrap"
														onclick={() => toggleSort(col.key as keyof Deployment)}
													>
														{col.label}
														<ArrowUpDown class="w-3 h-3 opacity-40" />
													</button>
												</Table.Head>
											{/each}
											<Table.Head class="w-20"></Table.Head>
										</Table.Row>
									</Table.Header>
									<Table.Body>
										{#each paginated as d (d.id)}
											<Table.Row class="hover:bg-muted/30 transition-colors">
												<Table.Cell>
													<div class="flex items-center gap-2.5">
														<div class="w-8 h-8 rounded-md bg-muted flex items-center justify-center shrink-0">
															{#if d.serviceType === 'Application'}
																<Rocket class="w-4 h-4 text-muted-foreground" />
															{:else}
																<Boxes class="w-4 h-4 text-muted-foreground" />
															{/if}
														</div>
														<div class="flex flex-col min-w-0">
															<span class="text-sm font-medium truncate">{d.service}</span>
															<Badge variant="outline" class="w-fit text-[10px] px-1.5 py-0 h-4 mt-0.5">{d.serviceType}</Badge>
														</div>
													</div>
												</Table.Cell>
												<Table.Cell class="text-sm text-muted-foreground">{d.project}</Table.Cell>
												<Table.Cell class="text-sm text-muted-foreground">{d.environment}</Table.Cell>
												<Table.Cell>
													<div class="flex items-center gap-1.5 text-sm text-muted-foreground">
														<Server class="w-3.5 h-3.5 shrink-0" />{d.server}
													</div>
												</Table.Cell>
												<Table.Cell>
													<span class="text-sm truncate max-w-[180px] block">{d.title || '—'}</span>
												</Table.Cell>
												<Table.Cell>
													<div class="flex items-center gap-1.5">
														{#if d.status === 'running'}
															<Loader2 class="w-3 h-3 animate-spin text-yellow-500 shrink-0" />
														{/if}
														<span class="inline-flex items-center px-2 py-0.5 rounded-full text-[10px] font-medium border capitalize {deployBadge(d.status)}">
															{d.status}
														</span>
													</div>
												</Table.Cell>
												<Table.Cell class="text-sm text-muted-foreground whitespace-nowrap">{d.createdAt}</Table.Cell>
												<Table.Cell>
													<Button variant="ghost" size="sm" class="h-7 gap-1 text-xs" onclick={() => goto(d.href)}>
														<ExternalLink class="w-3.5 h-3.5" /> Open
													</Button>
												</Table.Cell>
											</Table.Row>
										{/each}
									</Table.Body>
								</Table.Root>
							{/if}
						</div>

						<!-- Pagination -->
						{#if filtered.length > 0}
							<div class="flex flex-col gap-3 sm:flex-row sm:items-center sm:justify-between">
								<div class="flex items-center gap-2 flex-wrap text-sm text-muted-foreground">
									<span class="whitespace-nowrap">Rows per page</span>
									<Select.Root type="single" value={String(pageSize)} onValueChange={(v) => (pageSize = Number(v ?? 10))}>
										<Select.Trigger class="h-8 w-16 text-sm">{pageSize}</Select.Trigger>
										<Select.Content>
											{#each [10, 25, 50, 100] as n (n)}
												<Select.Item value={String(n)}>{n}</Select.Item>
											{/each}
										</Select.Content>
									</Select.Root>
									<span class="whitespace-nowrap">Showing {showFrom}–{showTo} of {filtered.length}</span>
								</div>
								<div class="flex items-center gap-2">
									<Button variant="outline" size="sm" class="h-8 gap-1" disabled={page <= 1} onclick={() => page--}>
										<ChevronLeft class="w-4 h-4" /> Previous
									</Button>
									<span class="text-xs text-muted-foreground tabular-nums">{page} / {totalPages}</span>
									<Button variant="outline" size="sm" class="h-8 gap-1" disabled={page >= totalPages} onclick={() => page++}>
										Next <ChevronRight class="w-4 h-4" />
									</Button>
								</div>
							</div>
						{/if}
					</Tabs.Content>

					<!-- ── Queue Tab ── -->
					<Tabs.Content value="queue" class="pt-4 pb-6">
						<div class="rounded-md border border-border overflow-x-auto">
							{#if queueJobs.length === 0}
								<div class="flex flex-col items-center justify-center py-20 text-muted-foreground gap-2">
									<ListTodo class="w-10 h-10 opacity-20" />
									<p class="font-medium">Queue is empty</p>
									<p class="text-sm opacity-60">Deployment jobs will appear here when queued</p>
								</div>
							{:else}
								<Table.Root>
									<Table.Header>
										<Table.Row>
											<Table.Head class="text-sm font-medium">Job ID</Table.Head>
											<Table.Head class="text-sm font-medium">Label</Table.Head>
											<Table.Head class="text-sm font-medium">Type</Table.Head>
											<Table.Head class="text-sm font-medium">State</Table.Head>
											<Table.Head class="text-sm font-medium">Added</Table.Head>
											<Table.Head class="text-sm font-medium">Processed</Table.Head>
											<Table.Head class="text-sm font-medium">Finished</Table.Head>
											<Table.Head class="text-sm font-medium">Error</Table.Head>
										</Table.Row>
									</Table.Header>
									<Table.Body>
										{#each queueJobs as job (job.id)}
											<Table.Row class="hover:bg-muted/30 transition-colors">
												<Table.Cell class="font-mono text-sm text-muted-foreground">{job.id}</Table.Cell>
												<Table.Cell class="text-sm max-w-[180px] truncate">{job.label}</Table.Cell>
												<Table.Cell class="text-sm text-muted-foreground capitalize">{job.type}</Table.Cell>
												<Table.Cell>
													<span class="inline-flex items-center px-2 py-0.5 rounded-full text-[10px] font-medium border capitalize {queueBadge(job.state)}">
														{job.state}
													</span>
												</Table.Cell>
												<Table.Cell class="text-sm text-muted-foreground whitespace-nowrap">{job.added}</Table.Cell>
												<Table.Cell class="text-sm text-muted-foreground whitespace-nowrap">{job.processed}</Table.Cell>
												<Table.Cell class="text-sm text-muted-foreground whitespace-nowrap">{job.finished}</Table.Cell>
												<Table.Cell class="text-sm text-destructive max-w-[160px] truncate">{job.error || '—'}</Table.Cell>
											</Table.Row>
										{/each}
									</Table.Body>
								</Table.Root>
							{/if}
						</div>
					</Tabs.Content>

				</Tabs.Root>
			</div>

		</div>
	</main>
</PageLayout>

<script lang="ts">
	import { goto } from '$app/navigation';
	import {
		Workflow, Server, Activity, Monitor, Settings, Container,
		Cpu, RefreshCw, ChevronDown, ChevronRight, AlertTriangle,
		Info, CheckCircle2
	} from '@lucide/svelte';
	import PageLayout from '$lib/components/PageLayout.svelte';
	import { getAuthSession } from '$lib/auth';
	import * as Card from '$lib/components/ui/card';
	import * as Table from '$lib/components/ui/table';
	import * as Tabs from '$lib/components/ui/tabs';
	import { Button } from '$lib/components/ui/button';
	import { Badge } from '$lib/components/ui/badge';
	import { Progress } from '$lib/components/ui/progress';

	const session = getAuthSession();
	if (!session) goto('/auth', { replaceState: true });

	// ─── Static data ──────────────────────────────────────────────────────────────
	type SwarmNode = {
		id: string;
		hostname: string;
		status: 'Ready' | 'Down';
		availability: 'Active' | 'Pause' | 'Drain';
		managerStatus: 'Leader' | 'Reachable' | '';
		role: 'Manager' | 'Worker';
		os: string;
		arch: string;
		cpus: number;
		memory: string;
		engineVersion: string;
	};

	type NodeContainer = {
		id: string;
		name: string;
		image: string;
		state: string;
		desiredState: string;
		node: string;
		ports: string;
		cpu: string;
		memory: string;
		blockIO: string;
		netIO: string;
	};

	const nodes: SwarmNode[] = [
		{ id: 'n1', hostname: 'manager-1', status: 'Ready', availability: 'Active', managerStatus: 'Leader',    role: 'Manager', os: 'linux', arch: 'x86_64', cpus: 4,  memory: '8 GiB',  engineVersion: '26.1.3' },
		{ id: 'n2', hostname: 'worker-1',  status: 'Ready', availability: 'Active', managerStatus: 'Reachable', role: 'Manager', os: 'linux', arch: 'x86_64', cpus: 4,  memory: '8 GiB',  engineVersion: '26.1.3' },
		{ id: 'n3', hostname: 'worker-2',  status: 'Ready', availability: 'Active', managerStatus: '',          role: 'Worker',  os: 'linux', arch: 'x86_64', cpus: 2,  memory: '4 GiB',  engineVersion: '26.1.3' },
		{ id: 'n4', hostname: 'worker-3',  status: 'Down',  availability: 'Drain',  managerStatus: '',          role: 'Worker',  os: 'linux', arch: 'x86_64', cpus: 2,  memory: '4 GiB',  engineVersion: '26.1.2' },
	];

	const containers: NodeContainer[] = [
		{ id: 'c1',  name: 'rustploy-app.1',   image: 'rustploy:latest',        state: 'Running 3h',   desiredState: 'Running', node: 'manager-1', ports: '3000',       cpu: '0.8%',  memory: '128 MiB', blockIO: '1.2MB / 0.4MB', netIO: '5.1MB / 2.3MB' },
		{ id: 'c2',  name: 'rustploy-app.2',   image: 'rustploy:latest',        state: 'Running 3h',   desiredState: 'Running', node: 'worker-1',  ports: '3000',       cpu: '0.6%',  memory: '112 MiB', blockIO: '0.9MB / 0.3MB', netIO: '4.8MB / 2.1MB' },
		{ id: 'c3',  name: 'api-service.1',    image: 'api-service:v2',         state: 'Running 5h',   desiredState: 'Running', node: 'manager-1', ports: '8080',       cpu: '1.2%',  memory: '256 MiB', blockIO: '2.1MB / 1.0MB', netIO: '8.4MB / 4.2MB' },
		{ id: 'c4',  name: 'postgres-db.1',    image: 'postgres:16-alpine',     state: 'Running 12h',  desiredState: 'Running', node: 'worker-1',  ports: '5432',       cpu: '3.4%',  memory: '512 MiB', blockIO: '45MB / 20MB',   netIO: '1.2MB / 0.8MB' },
		{ id: 'c5',  name: 'redis-cache.1',    image: 'redis:7-alpine',         state: 'Running 12h',  desiredState: 'Running', node: 'worker-2',  ports: '6379',       cpu: '0.2%',  memory: '32 MiB',  blockIO: '0.1MB / 0.1MB', netIO: '2.1MB / 1.9MB' },
		{ id: 'c6',  name: 'traefik.1',        image: 'traefik:v3.1',           state: 'Running 2d',   desiredState: 'Running', node: 'manager-1', ports: '80,443',     cpu: '0.5%',  memory: '48 MiB',  blockIO: '0.5MB / 0.2MB', netIO: '120MB / 80MB'  },
		{ id: 'c7',  name: 'worker-service.1', image: 'rustploy-worker:latest', state: 'Running 1h',   desiredState: 'Running', node: 'worker-2',  ports: '',           cpu: '2.1%',  memory: '180 MiB', blockIO: '3.2MB / 1.5MB', netIO: '0.8MB / 0.4MB' },
		{ id: 'c8',  name: 'backup-agent.1',   image: 'backup-agent:1.0',       state: 'Failed',       desiredState: 'Running', node: 'worker-2',  ports: '',           cpu: '--',    memory: '--',      blockIO: '--',            netIO: '--'            },
	];

	// ─── Derived counts ───────────────────────────────────────────────────────────
	const totalNodes   = nodes.length;
	const activeNodes  = nodes.filter(n => n.status === 'Ready');
	const managerNodes = nodes.filter(n => n.managerStatus === 'Leader' || n.managerStatus === 'Reachable');
	const downNodes    = nodes.filter(n => n.status !== 'Ready' || n.availability !== 'Active');
	const isMultiNode  = totalNodes > 1;

	const services       = [...new Set(containers.map(c => c.name.split('.')[0]))];
	const runningCtrs    = containers.filter(c => c.state.startsWith('Running'));
	const unscheduled    = containers.filter(c => c.desiredState === 'Running' && !c.state.startsWith('Running'));

	// Group containers by node
	const nodeGroups = activeNodes
		.map(node => ({
			node,
			containers: containers.filter(c => c.node === node.hostname)
		}))
		.filter(g => g.containers.length > 0)
		.sort((a, b) => a.node.hostname.localeCompare(b.node.hostname));

	let expandedNodes = $state<Set<string>>(new Set(nodeGroups.map(g => g.node.hostname)));
	let activeTab = $state('overview');
	let refreshing = $state(false);

	function toggleNode(hostname: string) {
		const s = new Set(expandedNodes);
		s.has(hostname) ? s.delete(hostname) : s.add(hostname);
		expandedNodes = s;
	}

	function refresh() { refreshing = true; setTimeout(() => (refreshing = false), 800); }

	function managerBadgeVariant(status: string): 'default' | 'secondary' | 'outline' {
		if (status === 'Leader')    return 'default';
		if (status === 'Reachable') return 'secondary';
		return 'outline';
	}

	function stateBadgeClass(state: string) {
		if (state.startsWith('Running')) return 'bg-green-500/15 text-green-500 border-green-500/20';
		if (state === 'Failed')          return 'bg-red-500/15 text-red-500 border-red-500/20';
		return 'bg-muted text-muted-foreground border-border';
	}
</script>

<PageLayout>
	<header class="flex items-center gap-2 px-6 py-3 border-b border-border text-sm">
		<Workflow class="w-4 h-4 text-muted-foreground" />
		<span class="font-medium">Swarm</span>
	</header>

	<main class="flex-1 p-6 overflow-y-auto">
		<div class="rounded-xl border border-border bg-card">

			<!-- Header -->
			<div class="flex items-start justify-between gap-3 flex-wrap px-6 pt-6 pb-4 border-b border-border">
				<div>
					<h1 class="text-xl font-bold flex items-center gap-2">
						<Workflow class="w-5 h-5" /> Docker Swarm Overview
					</h1>
					<p class="text-sm text-muted-foreground mt-0.5">Monitor and manage your Docker Swarm cluster</p>
				</div>
				<div class="flex items-center gap-2">
					<Button variant="outline" size="sm" class="gap-1.5 h-8" onclick={refresh}>
						<RefreshCw class="w-3.5 h-3.5 {refreshing ? 'animate-spin' : ''}" />
						Refresh
					</Button>
					<Button size="sm" class="gap-1.5 h-8" onclick={() => goto('/settings/cluster')}>
						<Settings class="w-3.5 h-3.5" /> Manage Cluster
					</Button>
				</div>
			</div>

			<div class="px-6 pt-4">
				<Tabs.Root value={activeTab} onValueChange={(v) => (activeTab = v)}>
					<Tabs.List>
						<Tabs.Trigger value="overview">Overview</Tabs.Trigger>
						<Tabs.Trigger value="containers">Containers</Tabs.Trigger>
					</Tabs.List>

					<!-- ── Overview Tab ── -->
					<Tabs.Content value="overview" class="pt-4 pb-6 space-y-6">

						<!-- 3 stat cards -->
						<div class="grid gap-4 md:grid-cols-3">
							<!-- Total Nodes -->
							<Card.Root class="bg-background">
								<Card.Header class="flex flex-row items-center justify-between space-y-0 pb-2">
									<Card.Title class="text-sm font-medium">Total Nodes</Card.Title>
									<div class="p-2 bg-primary/10 rounded-md">
										<Server class="w-4 h-4 text-primary" />
									</div>
								</Card.Header>
								<Card.Content>
									<p class="text-2xl font-bold">{totalNodes}</p>
									{#if downNodes.length > 0}
										<p class="text-xs text-destructive mt-1">{downNodes.length} node(s) down or drained</p>
									{/if}
								</Card.Content>
							</Card.Root>

							<!-- Active Nodes -->
							<Card.Root class="bg-background">
								<Card.Header class="flex flex-row items-center justify-between space-y-0 pb-2">
									<div class="flex items-center gap-2">
										<Card.Title class="text-sm font-medium">Active Nodes</Card.Title>
										<Badge variant="outline" class="text-[10px] text-green-500 border-green-500/30">Online</Badge>
									</div>
									<div class="p-2 bg-primary/10 rounded-md">
										<Activity class="w-4 h-4 text-primary" />
									</div>
								</Card.Header>
								<Card.Content>
									<p class="text-2xl font-bold">{activeNodes.length} / {totalNodes}</p>
									<div class="text-[11px] text-muted-foreground mt-1 space-y-0.5">
										{#each activeNodes as n (n.id)}<div>{n.hostname}</div>{/each}
									</div>
								</Card.Content>
							</Card.Root>

							<!-- Manager Nodes -->
							<Card.Root class="bg-background">
								<Card.Header class="flex flex-row items-center justify-between space-y-0 pb-2">
									<div class="flex items-center gap-2">
										<Card.Title class="text-sm font-medium">Manager Nodes</Card.Title>
										<Badge variant="outline" class="text-[10px] text-green-500 border-green-500/30">Online</Badge>
									</div>
									<div class="p-2 bg-primary/10 rounded-md">
										<Monitor class="w-4 h-4 text-primary" />
									</div>
								</Card.Header>
								<Card.Content>
									<p class="text-2xl font-bold">{managerNodes.length} / {totalNodes}</p>
									<div class="text-[11px] text-muted-foreground mt-1 space-y-0.5">
										{#each managerNodes as n (n.id)}<div>{n.hostname} ({n.managerStatus})</div>{/each}
									</div>
								</Card.Content>
							</Card.Root>
						</div>

						<!-- Node detail cards -->
						<div class="grid grid-cols-1 xl:grid-cols-2 2xl:grid-cols-3 gap-4">
							{#each nodes as node (node.id)}
								{@const isDown = node.status !== 'Ready' || node.availability !== 'Active'}
								<Card.Root class="bg-background {isDown ? 'border-destructive/40' : ''}">
									<Card.Header class="pb-3">
										<div class="flex items-center justify-between">
											<div class="flex items-center gap-2">
												<div class="relative">
													<Server class="w-5 h-5 text-muted-foreground" />
													{#if isDown}
														<span class="absolute -top-1 -right-1 w-2.5 h-2.5 rounded-full bg-destructive"></span>
													{:else}
														<span class="absolute -top-1 -right-1 w-2.5 h-2.5 rounded-full bg-green-500"></span>
													{/if}
												</div>
												<span class="font-semibold text-sm">{node.hostname}</span>
											</div>
											<div class="flex items-center gap-1.5">
												{#if node.managerStatus}
													<Badge variant={managerBadgeVariant(node.managerStatus)} class="text-[10px]">{node.managerStatus}</Badge>
												{/if}
												<Badge variant={isDown ? 'destructive' : 'secondary'} class="text-[10px] capitalize">
													{isDown ? node.availability : 'Active'}
												</Badge>
											</div>
										</div>
									</Card.Header>
									<Card.Content class="space-y-3 pt-0">
										<div class="grid grid-cols-2 gap-x-4 gap-y-1.5 text-xs">
											<div class="text-muted-foreground">Status</div>
											<div class="font-medium {isDown ? 'text-destructive' : 'text-green-500'}">{node.status}</div>
											<div class="text-muted-foreground">Role</div>
											<div class="font-medium capitalize">{node.role}</div>
											<div class="text-muted-foreground">CPUs</div>
											<div class="font-mono">{node.cpus}</div>
											<div class="text-muted-foreground">Memory</div>
											<div class="font-mono">{node.memory}</div>
											<div class="text-muted-foreground">OS / Arch</div>
											<div class="font-mono">{node.os} / {node.arch}</div>
											<div class="text-muted-foreground">Engine</div>
											<div class="font-mono">{node.engineVersion}</div>
										</div>
										{#if !isDown}
											{@const nodeCtrs = containers.filter(c => c.node === node.hostname)}
											<div class="pt-1 space-y-1.5">
												<div class="flex items-center justify-between text-[11px] text-muted-foreground">
													<span>Containers</span>
													<span class="font-medium text-foreground">{nodeCtrs.length}</span>
												</div>
												<Progress value={Math.min((nodeCtrs.length / 5) * 100, 100)} class="h-1.5" />
											</div>
										{/if}
									</Card.Content>
								</Card.Root>
							{/each}
						</div>
					</Tabs.Content>

					<!-- ── Containers Tab ── -->
					<Tabs.Content value="containers" class="pt-4 pb-6 space-y-4">

						<!-- Summary cards -->
						<div class="grid gap-4 md:grid-cols-3">
							<Card.Root class="bg-background">
								<Card.Header class="flex flex-row items-center justify-between space-y-0 pb-2">
									<Card.Title class="text-sm font-medium">Swarm Nodes</Card.Title>
									<div class="p-2 bg-primary/10 rounded-md">
										<Server class="w-4 h-4 text-primary" />
									</div>
								</Card.Header>
								<Card.Content>
									<p class="text-2xl font-bold">{totalNodes}</p>
									{#if downNodes.length > 0}
										<p class="text-xs text-destructive mt-1">{downNodes.length} node(s) down or drained</p>
									{/if}
								</Card.Content>
							</Card.Root>
							<Card.Root class="bg-background">
								<Card.Header class="flex flex-row items-center justify-between space-y-0 pb-2">
									<Card.Title class="text-sm font-medium">Services</Card.Title>
									<div class="p-2 bg-primary/10 rounded-md">
										<Cpu class="w-4 h-4 text-primary" />
									</div>
								</Card.Header>
								<Card.Content>
									<p class="text-2xl font-bold">{services.length}</p>
									{#if unscheduled.length > 0}
										<p class="text-sm text-muted-foreground mt-1">{unscheduled.length} with no running tasks</p>
									{/if}
								</Card.Content>
							</Card.Root>
							<Card.Root class="bg-background">
								<Card.Header class="flex flex-row items-center justify-between space-y-0 pb-2">
									<Card.Title class="text-sm font-medium">Running Containers</Card.Title>
									<div class="p-2 bg-primary/10 rounded-md">
										<Container class="w-4 h-4 text-primary" />
									</div>
								</Card.Header>
								<Card.Content>
									<p class="text-2xl font-bold">{runningCtrs.length}</p>
								</Card.Content>
							</Card.Root>
						</div>

						<!-- Down nodes alert -->
						{#if downNodes.length > 0}
							<div class="flex items-start gap-3 rounded-lg border border-destructive/30 bg-destructive/10 px-4 py-3 text-sm text-destructive">
								<AlertTriangle class="w-4 h-4 shrink-0 mt-0.5" />
								<div>
									<p class="font-semibold mb-1">{downNodes.length} Node(s) Unavailable</p>
									<p class="text-xs opacity-80 mb-2">The following nodes are not ready or have been drained. Containers scheduled on these nodes may not be running.</p>
									<ul class="text-xs space-y-0.5 list-disc list-inside">
										{#each downNodes as n (n.id)}
											<li><strong>{n.hostname}</strong> — Status: {n.status}, Availability: {n.availability}{n.managerStatus ? ` (${n.managerStatus})` : ''}</li>
										{/each}
									</ul>
								</div>
							</div>
						{/if}

						<!-- Multi-node metrics note -->
						{#if isMultiNode}
							<div class="flex items-start gap-3 rounded-lg border border-border bg-muted/30 px-4 py-3 text-sm text-muted-foreground">
								<Info class="w-4 h-4 shrink-0 mt-0.5" />
								<div>
									<p class="font-semibold text-foreground mb-0.5">Multi-Node Metrics Note</p>
									<p class="text-xs">CPU, memory, and I/O metrics are collected from the manager node via <code class="bg-muted px-1 py-0.5 rounded text-[10px]">docker stats</code>. Containers on worker nodes may show "—" for metrics.</p>
								</div>
							</div>
						{/if}

						<!-- Node sections (collapsible) -->
						<div class="flex flex-col gap-3">
							{#each nodeGroups as group (group.node.id)}
								{@const isExpanded = expandedNodes.has(group.node.hostname)}
								{@const isDown = group.node.status !== 'Ready' || group.node.availability !== 'Active'}
								{@const runningCount = group.containers.filter(c => c.state.startsWith('Running')).length}

								<Card.Root class="bg-background">
									<!-- Collapsible header -->
									<button
										class="w-full text-left px-6 py-4 hover:bg-muted/30 transition-colors rounded-t-xl"
										onclick={() => toggleNode(group.node.hostname)}
									>
										<div class="flex items-center justify-between">
											<div class="flex items-center gap-3">
												{#if isExpanded}
													<ChevronDown class="w-4 h-4 text-muted-foreground" />
												{:else}
													<ChevronRight class="w-4 h-4 text-muted-foreground" />
												{/if}
												<div class="relative">
													<Server class="w-5 h-5 text-muted-foreground" />
													{#if isDown}
														<span class="absolute -top-1 -right-1 w-2.5 h-2.5 rounded-full bg-destructive"></span>
													{/if}
												</div>
												<span class="font-semibold text-sm">{group.node.hostname}</span>
												{#if group.node.managerStatus}
													<Badge variant={managerBadgeVariant(group.node.managerStatus)} class="text-[10px]">
														{group.node.managerStatus}
													</Badge>
												{:else}
													<Badge variant="outline" class="text-[10px]">Worker</Badge>
												{/if}
												<Badge variant="secondary" class="text-[10px]">
													{group.containers.length} container{group.containers.length !== 1 ? 's' : ''}
												</Badge>
												{#if isDown}
													<Badge variant="destructive" class="text-[10px]">{group.node.status} / {group.node.availability}</Badge>
												{:else if runningCount === group.containers.length}
													<Badge variant="outline" class="text-[10px] text-green-500 border-green-500/30">
														<CheckCircle2 class="w-3 h-3 mr-1" /> All Running
													</Badge>
												{:else}
													<Badge variant="secondary" class="text-[10px] text-yellow-500">{runningCount}/{group.containers.length} Running</Badge>
												{/if}
											</div>
										</div>
									</button>

									<!-- Collapsible content -->
									{#if isExpanded}
										<div class="border-t border-border">
											<Table.Root>
												<Table.Header>
													<Table.Row>
														<Table.Head class="text-sm pl-6">Container</Table.Head>
														<Table.Head class="text-sm">State</Table.Head>
														<Table.Head class="text-sm text-right">CPU</Table.Head>
														<Table.Head class="text-sm text-right">Memory</Table.Head>
														<Table.Head class="text-sm text-right">Block I/O</Table.Head>
														<Table.Head class="text-sm text-right">Network I/O</Table.Head>
													</Table.Row>
												</Table.Header>
												<Table.Body>
													{#each group.containers as c (c.id)}
														<Table.Row class="hover:bg-muted/20 transition-colors">
															<Table.Cell class="pl-6">
																<div>
																	<p class="text-xs font-mono font-medium">{c.name}</p>
																	<p class="text-[10px] text-muted-foreground font-mono truncate max-w-[200px]">{c.image}</p>
																</div>
															</Table.Cell>
															<Table.Cell>
																<span class="inline-flex items-center px-2 py-0.5 rounded-full text-[10px] font-medium border {stateBadgeClass(c.state)}">
																	{c.state}
																</span>
															</Table.Cell>
															<Table.Cell class="text-sm text-right font-mono text-muted-foreground">{c.cpu}</Table.Cell>
															<Table.Cell class="text-sm text-right font-mono text-muted-foreground">{c.memory}</Table.Cell>
															<Table.Cell class="text-sm text-right font-mono text-muted-foreground">{c.blockIO}</Table.Cell>
															<Table.Cell class="text-sm text-right font-mono text-muted-foreground">{c.netIO}</Table.Cell>
														</Table.Row>
													{/each}
												</Table.Body>
											</Table.Root>
										</div>
									{/if}
								</Card.Root>
							{/each}
						</div>

						<!-- Unscheduled services alert -->
						{#if unscheduled.length > 0}
							<div class="flex items-start gap-3 rounded-lg border border-border bg-muted/30 px-4 py-3 text-sm text-muted-foreground">
								<Info class="w-4 h-4 shrink-0 mt-0.5" />
								<div>
									<p class="font-semibold text-foreground mb-1">{unscheduled.length} Service(s) With No Running Tasks</p>
									<p class="text-xs mb-2">These services exist in the swarm but have no running containers. They may be scaled to 0 replicas or failing to start.</p>
									<ul class="text-xs space-y-0.5 list-disc list-inside">
										{#each unscheduled as c (c.id)}
											<li><strong>{c.name}</strong></li>
										{/each}
									</ul>
								</div>
							</div>
						{/if}

					</Tabs.Content>
				</Tabs.Root>
			</div>
		</div>
	</main>
</PageLayout>

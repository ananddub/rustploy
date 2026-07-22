<script lang="ts">
	import { goto } from '$app/navigation';
	import {
		Container,
		Search,
		RefreshCw,
		MoreHorizontal,
		Terminal,
		FileText,
		Trash2,
		ScrollText,
		ChevronLeft,
		ChevronRight,
		ArrowUpDown,
		ChevronDown,
		Copy,
		Network,
		HardDrive,
		Upload
	} from '@lucide/svelte';
	import PageLayout from '$lib/components/PageLayout.svelte';
	import { getAuthSession } from '$lib/auth';
	import * as Table from '$lib/components/ui/table';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import * as Select from '$lib/components/ui/select';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { Badge } from '$lib/components/ui/badge';
	import { toastSuccess } from '$lib/toast';

	const session = getAuthSession();
	if (!session) goto('/auth', { replaceState: true });

	type DockerContainer = {
		containerId: string;
		name: string;
		state: string;
		status: string;
		image: string;
		ports: string;
	};

	let containers = $state<DockerContainer[]>([
		{
			containerId: 'a1b2c3d4e5f6a1b2c3d4e5f6',
			name: 'rustploy-app',
			state: 'running',
			status: 'Up 3 hours',
			image: 'rustploy:latest',
			ports: '0.0.0.0:3000->3000/tcp'
		},
		{
			containerId: 'b2c3d4e5f6a7b2c3d4e5f6a7',
			name: 'postgres-db',
			state: 'running',
			status: 'Up 5 hours',
			image: 'postgres:16-alpine',
			ports: '0.0.0.0:5432->5432/tcp'
		},
		{
			containerId: 'c3d4e5f6a7b8c3d4e5f6a7b8',
			name: 'redis-cache',
			state: 'running',
			status: 'Up 5 hours',
			image: 'redis:7-alpine',
			ports: '0.0.0.0:6379->6379/tcp'
		},
		{
			containerId: 'd4e5f6a7b8c9d4e5f6a7b8c9',
			name: 'traefik-proxy',
			state: 'running',
			status: 'Up 12 hours',
			image: 'traefik:v3.1',
			ports: '0.0.0.0:80->80/tcp, 0.0.0.0:443->443/tcp'
		},
		{
			containerId: 'e5f6a7b8c9d0e5f6a7b8c9d0',
			name: 'worker-old',
			state: 'exited',
			status: 'Exited (0) 2 days ago',
			image: 'rustploy-worker:0.2.1',
			ports: ''
		},
		{
			containerId: 'f6a7b8c9d0e1f6a7b8c9d0e1',
			name: 'nginx-temp',
			state: 'paused',
			status: 'Up 1 hour (Paused)',
			image: 'nginx:1.25-alpine',
			ports: '0.0.0.0:8080->80/tcp'
		},
		{
			containerId: 'a2b3c4d5e6f7a2b3c4d5e6f7',
			name: 'mailhog',
			state: 'running',
			status: 'Up 2 days',
			image: 'mailhog/mailhog:latest',
			ports: '0.0.0.0:8025->8025/tcp'
		},
		{
			containerId: 'b3c4d5e6f7a8b3c4d5e6f7a8',
			name: 'backup-agent',
			state: 'restarting',
			status: 'Restarting (1) 3 min ago',
			image: 'backup-agent:1.0.0',
			ports: ''
		}
	]);

	const STATES = ['running', 'exited', 'paused', 'restarting', 'created', 'removing', 'dead'];

	// ─── Toolbar state ────────────────────────────────────────────────────────────
	let search = $state('');
	let stateFilter = $state('all');
	let sortKey = $state<keyof DockerContainer | ''>('');
	let sortDir = $state<'asc' | 'desc'>('asc');
	let refreshing = $state(false);
	let page = $state(1);
	const pageSize = 10;

	// Column visibility
	let visibleCols = $state({ name: true, state: true, status: true, image: true, ports: true });
	type ColKey = keyof typeof visibleCols;

	// ─── Derived ──────────────────────────────────────────────────────────────────
	const filtered = $derived.by(() => {
		let list = containers.filter((c) => {
			const matchName = !search || c.name.toLowerCase().includes(search.toLowerCase());
			const matchState = stateFilter === 'all' || c.state === stateFilter;
			return matchName && matchState;
		});
		if (sortKey) {
			list = [...list].sort((a, b) => {
				const av = a[sortKey as keyof DockerContainer] ?? '';
				const bv = b[sortKey as keyof DockerContainer] ?? '';
				return sortDir === 'asc' ? av.localeCompare(bv) : bv.localeCompare(av);
			});
		}
		return list;
	});

	const totalPages = $derived(Math.max(1, Math.ceil(filtered.length / pageSize)));
	const paginated = $derived(filtered.slice((page - 1) * pageSize, page * pageSize));
	const showFrom = $derived(filtered.length === 0 ? 0 : (page - 1) * pageSize + 1);
	const showTo = $derived(Math.min(page * pageSize, filtered.length));

	// Reset page on filter change
	$effect(() => {
		search;
		stateFilter;
		page = 1;
	});

	function toggleSort(key: keyof DockerContainer) {
		if (sortKey === key) sortDir = sortDir === 'asc' ? 'desc' : 'asc';
		else {
			sortKey = key;
			sortDir = 'asc';
		}
	}

	function refresh() {
		refreshing = true;
		setTimeout(() => (refreshing = false), 1000);
	}

	function copyId(id: string) {
		navigator.clipboard.writeText(id);
		toastSuccess('Container ID copied to clipboard');
	}

	// ─── State badge helpers ──────────────────────────────────────────────────────
	function stateBadgeVariant(state: string) {
		if (state === 'running') return 'bg-green-700';
		if (state === 'exited' || state === 'dead') return 'bg-destructive';
		if (state === 'paused') return 'bg-yellow-700';
		if (state === 'restarting') return 'bg-blue-700 animate-pulse';
		return 'bg-muted-foreground/40  ';
	}

	const allCols: { key: ColKey; label: string }[] = [
		{ key: 'name', label: 'Name' },
		{ key: 'state', label: 'State' },
		{ key: 'status', label: 'Status' },
		{ key: 'image', label: 'Image' },
		{ key: 'ports', label: 'Ports' }
	];
</script>

<PageLayout>
	<header class="flex items-center gap-2 px-6 py-3 border-b border-border text-sm">
		<Container class="w-4 h-4 text-muted-foreground" />
		<span class="font-medium">Docker</span>
	</header>

	<main class="flex-1 p-6 overflow-y-auto">
		<div class="rounded-xl border border-border bg-card overflow-hidden">
			<!-- Card header -->
			<div class="px-6 pt-5 pb-4 border-b border-border">
				<div class="flex items-center gap-2 mb-0.5">
					<Container class="w-5 h-5 text-muted-foreground" />
					<h1 class="text-xl font-bold">Docker Containers</h1>
				</div>
				<p class="text-sm text-muted-foreground">See all the containers running on your server</p>
			</div>

			<div class="p-6 space-y-4">
				<!-- Toolbar -->
				<div class="flex items-center gap-2 flex-wrap">
					<!-- Search -->
					<div class="relative flex-1 min-w-[180px] max-w-sm">
						<Search
							class="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-muted-foreground"
						/>
						<Input bind:value={search} placeholder="Filter by name..." class="pl-9 h-9" />
					</div>

					<!-- State filter -->
					<Select.Root
						type="single"
						value={stateFilter}
						onValueChange={(v) => (stateFilter = v ?? 'all')}
					>
						<Select.Trigger class="w-40 h-9">
							<span class="text-sm capitalize"
								>{stateFilter === 'all' ? 'All states' : stateFilter}</span
							>
						</Select.Trigger>
						<Select.Content>
							<Select.Item value="all">All states</Select.Item>
							{#each STATES as s (s)}
								<Select.Item value={s} class="capitalize">{s}</Select.Item>
							{/each}
						</Select.Content>
					</Select.Root>

					<!-- Refresh -->
					<Button
						variant="outline"
						size="icon"
						class="h-9 w-9 shrink-0 sm:ml-auto"
						onclick={refresh}
						disabled={refreshing}
					>
						<RefreshCw class="w-4 h-4 {refreshing ? 'animate-spin' : ''}" />
					</Button>

					<!-- Column toggle -->
					<DropdownMenu.Root>
						<DropdownMenu.Trigger>
							<Button variant="outline" class="h-9 gap-1.5 text-sm">
								Columns <ChevronDown class="w-4 h-4" />
							</Button>
						</DropdownMenu.Trigger>
						<DropdownMenu.Content align="end">
							{#each allCols as col (col.key)}
								<DropdownMenu.CheckboxItem
									checked={visibleCols[col.key]}
									onCheckedChange={(v) => (visibleCols[col.key] = !!v)}
									class="capitalize"
								>
									{col.label}
								</DropdownMenu.CheckboxItem>
							{/each}
						</DropdownMenu.Content>
					</DropdownMenu.Root>
				</div>

				<!-- Table -->
				<div class="rounded-md border border-border overflow-hidden">
					{#if containers.length === 0}
						<div class="flex flex-col items-center justify-center h-64 text-muted-foreground gap-2">
							<Container class="w-10 h-10 opacity-30" />
							<p class="text-base font-medium">No containers found</p>
						</div>
					{:else if filtered.length === 0}
						<div class="flex items-center justify-center h-40 text-muted-foreground text-sm">
							No results match your filters
						</div>
					{:else}
						<Table.Root>
							<Table.Header>
								<Table.Row>
									{#if visibleCols.name}
										<Table.Head>
											<button
												class="flex pl-2 items-center gap-1.5 text-xs font-medium hover:text-foreground transition-colors"
												onclick={() => toggleSort('name')}
											>
												Name <ArrowUpDown class="w-3.5 h-3.5 opacity-50" />
											</button>
										</Table.Head>
									{/if}
									{#if visibleCols.state}
										<Table.Head>
											<button
												class="flex items-center gap-1.5 text-xs font-medium hover:text-foreground transition-colors"
												onclick={() => toggleSort('state')}
											>
												State <ArrowUpDown class="w-3.5 h-3.5 opacity-50" />
											</button>
										</Table.Head>
									{/if}
									{#if visibleCols.status}
										<Table.Head>
											<button
												class="flex items-center gap-1.5 text-xs font-medium hover:text-foreground transition-colors"
												onclick={() => toggleSort('status')}
											>
												Status <ArrowUpDown class="w-3.5 h-3.5 opacity-50" />
											</button>
										</Table.Head>
									{/if}
									{#if visibleCols.image}
										<Table.Head>
											<button
												class="flex items-center gap-1.5 text-xs font-medium hover:text-foreground transition-colors"
												onclick={() => toggleSort('image')}
											>
												Image <ArrowUpDown class="w-3.5 h-3.5 opacity-50" />
											</button>
										</Table.Head>
									{/if}
									{#if visibleCols.ports}
										<Table.Head>
											<button
												class="flex items-center gap-1.5 text-xs font-medium hover:text-foreground transition-colors"
												onclick={() => toggleSort('ports')}
											>
												Ports <ArrowUpDown class="w-3.5 h-3.5 opacity-50" />
											</button>
										</Table.Head>
									{/if}
									<Table.Head class="w-10"></Table.Head>
								</Table.Row>
							</Table.Header>
							<Table.Body>
								{#each paginated as c (c.containerId)}
									<Table.Row class="hover:bg-muted/40 transition-colors">
										{#if visibleCols.name}
											<Table.Cell>
												<div class="pl-2">
													<p class="text font-medium">{c.name}</p>
													<p class="text-[10px] text-muted-foreground font-mono">
														{c.containerId.slice(0, 12)}
													</p>
												</div>
											</Table.Cell>
										{/if}
										{#if visibleCols.state}
											<Table.Cell>
												<div class="flex items-center">
													<Badge
														class={`text-[10px] capitalize px-1.5 py-0 ${stateBadgeVariant(c.state)}`}
													>
														{c.state}
													</Badge>
												</div>
											</Table.Cell>
										{/if}
										{#if visibleCols.status}
											<Table.Cell class="text-sm text-muted-foreground">{c.status}</Table.Cell>
										{/if}
										{#if visibleCols.image}
											<Table.Cell>
												<span class="text-xs font-mono text-muted-foreground">{c.image}</span>
											</Table.Cell>
										{/if}
										{#if visibleCols.ports}
											<Table.Cell>
												<span
													class="text-xs font-mono text-muted-foreground max-w-[200px] truncate block"
													title={c.ports}
												>
													{c.ports || '—'}
												</span>
											</Table.Cell>
										{/if}
										<Table.Cell>
											<DropdownMenu.Root>
												<DropdownMenu.Trigger>
													<Button variant="ghost" size="sm" class="h-8 w-8 p-0">
														<MoreHorizontal class="w-4 h-4" />
													</Button>
												</DropdownMenu.Trigger>
												<DropdownMenu.Content align="end" class="w-48">
													<DropdownMenu.Label class="text-sm">Actions</DropdownMenu.Label>
													<DropdownMenu.Item
														class="gap-2 text-sm cursor-pointer"
														onclick={() => copyId(c.containerId)}
													>
														<Copy class="w-3.5 h-3.5" /> Copy Container ID
													</DropdownMenu.Item>
													<DropdownMenu.Separator />
													<DropdownMenu.Item class="gap-2 text-sm cursor-pointer">
														<ScrollText class="w-3.5 h-3.5" /> View Logs
													</DropdownMenu.Item>
													<DropdownMenu.Item class="gap-2 text-sm cursor-pointer">
														<FileText class="w-3.5 h-3.5" /> Config
													</DropdownMenu.Item>
													<DropdownMenu.Item class="gap-2 text-sm cursor-pointer">
														<HardDrive class="w-3.5 h-3.5" /> Mounts
													</DropdownMenu.Item>
													<DropdownMenu.Item class="gap-2 text-sm cursor-pointer">
														<Network class="w-3.5 h-3.5" /> Networks
													</DropdownMenu.Item>
													<DropdownMenu.Item class="gap-2 text-sm cursor-pointer">
														<Terminal class="w-3.5 h-3.5" /> Terminal
													</DropdownMenu.Item>
													<DropdownMenu.Item class="gap-2 text-sm cursor-pointer">
														<Upload class="w-3.5 h-3.5" /> Upload File
													</DropdownMenu.Item>
													<DropdownMenu.Separator />
													<DropdownMenu.Item
														class="gap-2 text-sm cursor-pointer text-destructive"
														variant="destructive"
													>
														<Trash2 class="w-3.5 h-3.5" /> Remove
													</DropdownMenu.Item>
												</DropdownMenu.Content>
											</DropdownMenu.Root>
										</Table.Cell>
									</Table.Row>
								{/each}
							</Table.Body>
						</Table.Root>
					{/if}
				</div>

				<!-- Pagination -->
				{#if filtered.length > 0}
					<div class="flex items-center justify-between">
						<p class="text-sm text-muted-foreground">
							Showing {showFrom}–{showTo} of {filtered.length} containers
						</p>
						<div class="flex items-center gap-2">
							<Button
								variant="outline"
								size="sm"
								class="h-8"
								disabled={page <= 1}
								onclick={() => (page = Math.max(1, page - 1))}
							>
								<ChevronLeft class="w-4 h-4" /> Previous
							</Button>
							<span class="text-xs text-muted-foreground px-1">{page} / {totalPages}</span>
							<Button
								variant="outline"
								size="sm"
								class="h-8"
								disabled={page >= totalPages}
								onclick={() => (page = Math.min(totalPages, page + 1))}
							>
								Next <ChevronRight class="w-4 h-4" />
							</Button>
						</div>
					</div>
				{/if}
			</div>
		</div>
	</main>
</PageLayout>

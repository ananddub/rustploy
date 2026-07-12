<script lang="ts">
	import { goto } from '$app/navigation';
	import {
		Package,
		Search,
		RefreshCw,
		MoreVertical,
		Terminal,
		FileText,
		Trash2,
		ScrollText,
		ChevronLeft,
		ChevronRight
	} from '@lucide/svelte';
	import PageLayout from '$lib/components/PageLayout.svelte';
	import { getAuthSession } from '$lib/auth';
	import * as Card from '$lib/components/ui/card';
	import * as Table from '$lib/components/ui/table';
	import * as DropdownMenu from '$lib/components/ui/dropdown-menu';
	import * as Select from '$lib/components/ui/select';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { Badge } from '$lib/components/ui/badge';
	import { Skeleton } from '$lib/components/ui/skeleton';

	const session = getAuthSession();
	if (!session) goto('/auth', { replaceState: true });

	// Static container data
	const containers = [
		{
			id: 'a1b2c3d4e5f6',
			name: 'rustploy-app',
			state: 'running',
			status: 'Up 3 hours',
			image: 'rustploy:latest',
			ports: '0.0.0.0:3000->3000/tcp'
		},
		{
			id: 'b2c3d4e5f6a7',
			name: 'postgres-db',
			state: 'running',
			status: 'Up 5 hours',
			image: 'postgres:16-alpine',
			ports: '0.0.0.0:5432->5432/tcp'
		},
		{
			id: 'c3d4e5f6a7b8',
			name: 'redis-cache',
			state: 'running',
			status: 'Up 5 hours',
			image: 'redis:7-alpine',
			ports: '0.0.0.0:6379->6379/tcp'
		},
		{
			id: 'd4e5f6a7b8c9',
			name: 'traefik-proxy',
			state: 'running',
			status: 'Up 12 hours',
			image: 'traefik:v3.1',
			ports: '0.0.0.0:80->80/tcp, 0.0.0.0:443->443/tcp'
		},
		{
			id: 'e5f6a7b8c9d0',
			name: 'worker-old',
			state: 'exited',
			status: 'Exited (0) 2 days ago',
			image: 'rustploy-worker:0.2.1',
			ports: ''
		},
		{
			id: 'f6a7b8c9d0e1',
			name: 'nginx-temp',
			state: 'paused',
			status: 'Up 1 hour (Paused)',
			image: 'nginx:1.25-alpine',
			ports: '0.0.0.0:8080->80/tcp'
		}
	];

	let searchQuery = $state('');
	let stateFilter = $state('all');
	let isRefreshing = $state(false);
	let currentPage = $state(1);
	const pageSize = 5;

	const filtered = $derived(
		containers.filter((c) => {
			const matchSearch =
				!searchQuery || c.name.toLowerCase().includes(searchQuery.toLowerCase());
			const matchState = stateFilter === 'all' || c.state === stateFilter;
			return matchSearch && matchState;
		})
	);

	const totalPages = $derived(Math.ceil(filtered.length / pageSize));
	const paginatedContainers = $derived(
		filtered.slice((currentPage - 1) * pageSize, currentPage * pageSize)
	);
	const showingFrom = $derived(filtered.length === 0 ? 0 : (currentPage - 1) * pageSize + 1);
	const showingTo = $derived(Math.min(currentPage * pageSize, filtered.length));

	function handleRefresh() {
		isRefreshing = true;
		setTimeout(() => {
			isRefreshing = false;
		}, 1000);
	}

	function getStateBadgeVariant(state: string) {
		if (state === 'running') return 'default' as const;
		if (state === 'exited') return 'destructive' as const;
		return 'outline' as const;
	}

	function getStateBadgeClass(state: string) {
		if (state === 'running') return 'bg-green-600 hover:bg-green-600 text-white';
		if (state === 'exited') return '';
		return '';
	}
</script>

<PageLayout>
	<main class="flex-1 p-6 space-y-6">
		<Card.Root>
			<Card.Header>
				<Card.Title class="text-lg flex items-center gap-2">
					<Package class="w-5 h-5" />
					Docker Containers
				</Card.Title>
				<Card.Description>See all containers on your server</Card.Description>
			</Card.Header>
			<Card.Content class="space-y-4">
				<!-- Toolbar -->
				<div class="flex items-center gap-3 flex-wrap">
					<div class="relative flex-1 min-w-[200px] max-w-sm">
						<Search
							class="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-muted-foreground"
						/>
						<Input
							bind:value={searchQuery}
							placeholder="Filter by name..."
							class="pl-9 h-9"
						/>
					</div>

					<Select.Root type="single" value={stateFilter} onValueChange={(v) => { stateFilter = v ?? 'all'; currentPage = 1; }}>
						<Select.Trigger class="w-[140px] h-9">
							<span class="text-sm capitalize">{stateFilter === 'all' ? 'All States' : stateFilter}</span>
						</Select.Trigger>
						<Select.Content>
							<Select.Item value="all">All States</Select.Item>
							<Select.Item value="running">Running</Select.Item>
							<Select.Item value="exited">Exited</Select.Item>
							<Select.Item value="paused">Paused</Select.Item>
						</Select.Content>
					</Select.Root>

					<Button variant="outline" size="sm" class="h-9 gap-1.5" onclick={handleRefresh}>
						<RefreshCw class="w-4 h-4 {isRefreshing ? 'animate-spin' : ''}" />
						Refresh
					</Button>
				</div>

				<!-- Table -->
				{#if filtered.length > 0}
					<div class="rounded-md border">
						<Table.Root>
							<Table.Header>
								<Table.Row>
									<Table.Head>Name</Table.Head>
									<Table.Head>State</Table.Head>
									<Table.Head>Status</Table.Head>
									<Table.Head>Image</Table.Head>
									<Table.Head>Ports</Table.Head>
									<Table.Head class="w-[50px]"></Table.Head>
								</Table.Row>
							</Table.Header>
							<Table.Body>
								{#each paginatedContainers as container (container.id)}
									<Table.Row>
										<Table.Cell>
											<div>
												<p class="text-sm font-medium">{container.name}</p>
												<p class="text-xs text-muted-foreground font-mono">
													{container.id.slice(0, 12)}
												</p>
											</div>
										</Table.Cell>
										<Table.Cell>
											<Badge
												variant={getStateBadgeVariant(container.state)}
												class="{getStateBadgeClass(container.state)} capitalize text-xs"
											>
												{container.state}
											</Badge>
										</Table.Cell>
										<Table.Cell>
											<span class="text-sm text-muted-foreground">{container.status}</span>
										</Table.Cell>
										<Table.Cell>
											<span class="text-sm font-mono text-muted-foreground">
												{container.image}
											</span>
										</Table.Cell>
										<Table.Cell>
											<span
												class="text-sm font-mono text-muted-foreground max-w-[180px] truncate block"
											>
												{container.ports || '—'}
											</span>
										</Table.Cell>
										<Table.Cell>
											<DropdownMenu.Root>
												<DropdownMenu.Trigger>
													<Button variant="ghost" size="sm" class="h-8 w-8 p-0">
														<MoreVertical class="w-4 h-4" />
													</Button>
												</DropdownMenu.Trigger>
												<DropdownMenu.Content align="end">
													<DropdownMenu.Item class="gap-2 cursor-pointer">
														<ScrollText class="w-4 h-4" />
														View Logs
													</DropdownMenu.Item>
													<DropdownMenu.Item class="gap-2 cursor-pointer">
														<Terminal class="w-4 h-4" />
														Terminal
													</DropdownMenu.Item>
													<DropdownMenu.Item class="gap-2 cursor-pointer">
														<FileText class="w-4 h-4" />
														Config
													</DropdownMenu.Item>
													<DropdownMenu.Separator />
													<DropdownMenu.Item class="gap-2 cursor-pointer text-destructive" variant="destructive">
														<Trash2 class="w-4 h-4" />
														Remove
													</DropdownMenu.Item>
												</DropdownMenu.Content>
											</DropdownMenu.Root>
										</Table.Cell>
									</Table.Row>
								{/each}
							</Table.Body>
						</Table.Root>
					</div>

					<!-- Pagination -->
					<div class="flex items-center justify-between pt-2">
						<p class="text-sm text-muted-foreground">
							Showing {showingFrom} to {showingTo} of {filtered.length}
						</p>
						<div class="flex items-center gap-2">
							<Button
								variant="outline"
								size="sm"
								class="h-8 gap-1"
								disabled={currentPage <= 1}
								onclick={() => (currentPage = Math.max(1, currentPage - 1))}
							>
								<ChevronLeft class="w-4 h-4" />
								Previous
							</Button>
							<Button
								variant="outline"
								size="sm"
								class="h-8 gap-1"
								disabled={currentPage >= totalPages}
								onclick={() => (currentPage = Math.min(totalPages, currentPage + 1))}
							>
								Next
								<ChevronRight class="w-4 h-4" />
							</Button>
						</div>
					</div>
				{:else}
					<!-- Empty state -->
					<div class="flex flex-col items-center justify-center py-16 text-center">
						<Package class="w-12 h-12 text-muted-foreground/50 mb-4" />
						<p class="text-sm font-medium text-muted-foreground">No containers found</p>
						<p class="text-xs text-muted-foreground/70 mt-1">
							Try adjusting your search or filter criteria
						</p>
					</div>
				{/if}
			</Card.Content>
		</Card.Root>
	</main>
</PageLayout>

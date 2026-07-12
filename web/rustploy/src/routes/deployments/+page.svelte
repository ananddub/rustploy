<script lang="ts">
	import { goto } from '$app/navigation';
	import {
		Zap,
		Search,
		Rocket,
		Boxes,
		Loader2,
		CircleCheck,
		CircleX,
		ListTodo
	} from '@lucide/svelte';
	import PageLayout from '$lib/components/PageLayout.svelte';
	import StatusBadge from '$lib/components/shared/StatusBadge.svelte';
	import { getAuthSession } from '$lib/auth';
	import { deploymentControllerActive } from '$lib/client/sdk.gen';
	import type { ActiveDeploymentDto } from '$lib/client/types.gen';
	import * as Card from '$lib/components/ui/card';
	import * as Table from '$lib/components/ui/table';
	import * as Tabs from '$lib/components/ui/tabs';
	import { Button } from '$lib/components/ui/button';
	import { Input } from '$lib/components/ui/input';
	import { Badge } from '$lib/components/ui/badge';
	import { Skeleton } from '$lib/components/ui/skeleton';

	const session = getAuthSession();
	if (!session) goto('/auth', { replaceState: true });

	let deployments = $state<ActiveDeploymentDto[]>([]);
	let loading = $state(true);
	let searchQuery = $state('');
	let filterState = $state<string>('all');
	let activeTab = $state('deployments');

	$effect(() => {
		deploymentControllerActive()
			.then((res: any) => {
				deployments = (res.data as ActiveDeploymentDto[]) ?? [];
			})
			.catch(() => {})
			.finally(() => {
				loading = false;
			});
	});

	const filtered = $derived(
		deployments.filter((d) => {
			const matchSearch =
				!searchQuery ||
				`${d.id}`.includes(searchQuery) ||
				d.kind?.toLowerCase().includes(searchQuery.toLowerCase());
			const matchState = filterState === 'all' || d.state === filterState;
			return matchSearch && matchState;
		})
	);

	const stats = $derived({
		total: deployments.length,
		running: deployments.filter((d) => d.state === 'running').length,
		success: deployments.filter((d) => d.state === 'done').length,
		failed: deployments.filter((d) => d.state === 'error').length
	});

	const stateFilters = ['all', 'running', 'done', 'error'];

	// Static empty queue data for now
	const queuedJobs: { id: string; type: string; state: string; createdAt: string; updatedAt: string }[] = [];

	function getServiceIcon(kind: string | undefined) {
		if (kind === 'application') return Rocket;
		if (kind === 'compose') return Boxes;
		return Rocket;
	}
</script>

<PageLayout>
	<header class="flex items-center justify-between px-6 py-3 border-b border-border">
		<div class="flex items-center gap-2 text-sm">
			<Zap class="w-4 h-4 text-primary" />
			<span class="font-medium">Deployments</span>
		</div>
	</header>

	<main class="flex-1 p-6 space-y-6">
		<!-- Stats Bar -->
		<div class="grid grid-cols-2 md:grid-cols-4 gap-1.5">
			<Card.Root>
				<Card.Content class="p-4">
					<div class="flex items-center justify-between">
						<div>
							<p class="text-xs text-muted-foreground uppercase tracking-wide">Total</p>
							<p class="text-2xl font-bold mt-1">{stats.total}</p>
						</div>
						<div class="w-9 h-9 rounded-full bg-primary/10 flex items-center justify-center">
							<Zap class="w-4 h-4 text-primary" />
						</div>
					</div>
				</Card.Content>
			</Card.Root>

			<Card.Root>
				<Card.Content class="p-4">
					<div class="flex items-center justify-between">
						<div>
							<p class="text-xs text-muted-foreground uppercase tracking-wide">Running</p>
							<p class="text-2xl font-bold mt-1 text-yellow-400">{stats.running}</p>
						</div>
						<div class="w-9 h-9 rounded-full bg-yellow-500/10 flex items-center justify-center">
							<Loader2 class="w-4 h-4 text-yellow-400 animate-spin" />
						</div>
					</div>
				</Card.Content>
			</Card.Root>

			<Card.Root>
				<Card.Content class="p-4">
					<div class="flex items-center justify-between">
						<div>
							<p class="text-xs text-muted-foreground uppercase tracking-wide">Succeeded</p>
							<p class="text-2xl font-bold mt-1 text-green-400">{stats.success}</p>
						</div>
						<div class="w-9 h-9 rounded-full bg-green-500/10 flex items-center justify-center">
							<CircleCheck class="w-4 h-4 text-green-400" />
						</div>
					</div>
				</Card.Content>
			</Card.Root>

			<Card.Root>
				<Card.Content class="p-4">
					<div class="flex items-center justify-between">
						<div>
							<p class="text-xs text-muted-foreground uppercase tracking-wide">Failed</p>
							<p class="text-2xl font-bold mt-1 text-red-400">{stats.failed}</p>
						</div>
						<div class="w-9 h-9 rounded-full bg-red-500/10 flex items-center justify-center">
							<CircleX class="w-4 h-4 text-red-400" />
						</div>
					</div>
				</Card.Content>
			</Card.Root>
		</div>

		<!-- Tabs -->
		<Tabs.Root value={activeTab} onValueChange={(v) => (activeTab = v)}>
			<Tabs.List>
				<Tabs.Trigger value="deployments">
					<Rocket class="w-3.5 h-3.5 mr-1.5" />
					Deployments
				</Tabs.Trigger>
				<Tabs.Trigger value="queue">
					<ListTodo class="w-3.5 h-3.5 mr-1.5" />
					Queue
				</Tabs.Trigger>
			</Tabs.List>

			<!-- Deployments Tab -->
			<Tabs.Content value="deployments" class="mt-4 space-y-4">
				<!-- Filter Toolbar -->
				<div class="flex items-center gap-3 flex-wrap">
					<div class="relative flex-1 max-w-sm">
						<Search class="absolute left-3 top-1/2 -translate-y-1/2 w-4 h-4 text-muted-foreground" />
						<Input
							bind:value={searchQuery}
							placeholder="Search by name or ID..."
							class="pl-9"
						/>
					</div>
					<div class="flex items-center gap-1.5">
						{#each stateFilters as sf}
							<Button
								variant={filterState === sf ? 'default' : 'outline'}
								size="sm"
								class="text-xs capitalize h-8"
								onclick={() => (filterState = sf)}
							>
								{sf === 'all' ? 'All' : sf === 'done' ? 'Done' : sf === 'error' ? 'Error' : 'Running'}
							</Button>
						{/each}
					</div>
				</div>

				<!-- Entry count -->
				<p class="text-xs text-muted-foreground">
					Showing {filtered.length} of {deployments.length} entries
				</p>

				<!-- Deployments Table -->
				<Card.Root>
					<Card.Content class="p-0">
						{#if loading}
							<div class="p-6 space-y-4">
								{#each Array(5) as _}
									<div class="flex items-center gap-4">
										<Skeleton class="h-4 w-8" />
										<Skeleton class="h-4 w-32" />
										<Skeleton class="h-6 w-20 rounded-full" />
										<Skeleton class="h-4 flex-1" />
									</div>
								{/each}
							</div>
						{:else if filtered.length === 0}
							<div class="flex flex-col items-center justify-center py-16 text-muted-foreground">
								<Rocket class="w-10 h-10 mb-3 opacity-20" />
								<p class="text-sm">No deployments found</p>
								<p class="text-xs mt-1 opacity-60">Adjust your filters or wait for new deployments</p>
							</div>
						{:else}
							<Table.Root>
								<Table.Header>
									<Table.Row>
										<Table.Head>Service</Table.Head>
										<Table.Head>Status</Table.Head>
										<Table.Head class="text-right">Started</Table.Head>
									</Table.Row>
								</Table.Header>
								<Table.Body>
									{#each filtered as d (d.id)}
										{@const ServiceIcon = getServiceIcon(d.kind)}
										<Table.Row class="cursor-pointer hover:bg-accent/50 transition-colors">
											<Table.Cell>
												<div class="flex items-center gap-3">
													<div class="w-8 h-8 rounded-md bg-muted flex items-center justify-center">
														<ServiceIcon class="w-4 h-4 text-muted-foreground" />
													</div>
													<div class="flex flex-col">
														<span class="text-sm font-medium">Deployment #{d.id}</span>
														<Badge variant="outline" class="w-fit text-[10px] capitalize mt-0.5">{d.kind ?? 'unknown'}</Badge>
													</div>
												</div>
											</Table.Cell>
											<Table.Cell>
												<StatusBadge status={d.state ?? 'pending'} pulse={d.state === 'running'} />
											</Table.Cell>
											<Table.Cell class="text-right text-xs text-muted-foreground">
												—
											</Table.Cell>
										</Table.Row>
									{/each}
								</Table.Body>
							</Table.Root>
						{/if}
					</Card.Content>
				</Card.Root>
			</Tabs.Content>

			<!-- Queue Tab -->
			<Tabs.Content value="queue" class="mt-4 space-y-4">
				<Card.Root>
					<Card.Content class="p-0">
						{#if queuedJobs.length === 0}
							<div class="flex flex-col items-center justify-center py-16 text-muted-foreground">
								<ListTodo class="w-10 h-10 mb-3 opacity-20" />
								<p class="text-sm">No queued jobs</p>
								<p class="text-xs mt-1 opacity-60">Jobs waiting to be processed will appear here</p>
							</div>
						{:else}
							<Table.Root>
								<Table.Header>
									<Table.Row>
										<Table.Head class="w-20">ID</Table.Head>
										<Table.Head>Type</Table.Head>
										<Table.Head>State</Table.Head>
										<Table.Head>Created</Table.Head>
										<Table.Head class="text-right">Updated</Table.Head>
									</Table.Row>
								</Table.Header>
								<Table.Body>
									{#each queuedJobs as job (job.id)}
										<Table.Row>
											<Table.Cell class="font-mono text-xs">#{job.id}</Table.Cell>
											<Table.Cell class="text-sm capitalize">{job.type}</Table.Cell>
											<Table.Cell>
												<Badge variant="secondary" class="text-[10px] capitalize">{job.state}</Badge>
											</Table.Cell>
											<Table.Cell class="text-xs text-muted-foreground">{job.createdAt}</Table.Cell>
											<Table.Cell class="text-right text-xs text-muted-foreground">{job.updatedAt}</Table.Cell>
										</Table.Row>
									{/each}
								</Table.Body>
							</Table.Root>
						{/if}
					</Card.Content>
				</Card.Root>
			</Tabs.Content>
		</Tabs.Root>
	</main>
</PageLayout>
